use std::{
    future::Future,
    io::{Cursor, Read},
    sync::Arc,
};

use anyhow::{Context, Result};
use async_fn_stream::try_fn_stream;
use async_std::stream::StreamExt;
use brush_render::{
    camera::{self, Camera},
    gaussian_splats::Splats,
    Backend,
};
use brush_train::scene::SceneView;
use glam::Vec3;
use zip::ZipArchive;

use crate::{
    archive_file_at_path, colmap_read_model, find_base_path, stream_fut_parallel, DataStream,
    Dataset, LoadDatasetArgs, LoadInitArgs, ZipData,
};

fn read_views(
    mut archive: ZipArchive<Cursor<ZipData>>,
    load_args: &LoadDatasetArgs,
) -> Result<Vec<impl Future<Output = Result<SceneView>>>> {
    let (is_binary, base_path) =
        if let Some(path) = find_base_path("sparse/0/cameras.bin", &archive) {
            (true, path)
        } else if let Some(path) = find_base_path("sparse/0/cameras.txt", &archive) {
            (false, path)
        } else {
            anyhow::bail!("No COLMAP data found (either text or binary.")
        };

    let (cam_path, img_path) = if is_binary {
        (
            base_path.join("sparse/0/cameras.bin"),
            base_path.join("sparse/0/images.bin"),
        )
    } else {
        (
            base_path.join("sparse/0/cameras.txt"),
            base_path.join("sparse/0/images.txt"),
        )
    };

    println!("{cam_path:?} {img_path:?}");
    let cam_model_data = {
        let mut cam_file = archive_file_at_path(&cam_path, &mut archive)?;
        colmap_read_model::read_cameras(&mut cam_file, is_binary)?
    };

    let img_infos = {
        let img_file = archive_file_at_path(&img_path, &mut archive)?;
        let mut buf_reader = std::io::BufReader::new(img_file);
        colmap_read_model::read_images(&mut buf_reader, is_binary)?
    };

    let mut img_info_list = img_infos.into_iter().collect::<Vec<_>>();

    // Sort by image ID. Not entirely sure whether it's better to
    // load things in COLMAP order or sorted by file name. Either way, at least
    // it is consistent
    img_info_list.sort_by_key(|key_img| key_img.0);

    let handles = img_info_list
        .into_iter()
        .take(load_args.max_frames.unwrap_or(usize::MAX))
        .map(move |(_, img_info)| {
            let mut archive = archive.clone();
            let cam = cam_model_data[&img_info.camera_id].clone();
            let translation = img_info.tvec;
            let quat = img_info.quat;
            let img_path = img_info.name.clone();
            let load_args = load_args.clone();
            let base_path = base_path.clone();

            async move {
                let focal = cam.focal();

                let fovx = camera::focal_to_fov(focal.x, cam.width as u32);
                let fovy = camera::focal_to_fov(focal.y, cam.height as u32);

                let center = cam.principal_point();
                let center_uv = center / glam::vec2(cam.width as f32, cam.height as f32);

                let img_path = base_path.join(format!("images/{img_path}"));

                let image_data = archive_file_at_path(&img_path, &mut archive)?;
                let img_bytes = image_data.bytes().collect::<std::io::Result<Vec<u8>>>()?;
                let mut img = image::load_from_memory(&img_bytes)?;

                if let Some(max) = load_args.max_resolution {
                    img = crate::clamp_img_to_max_size(img, max);
                }

                // Convert w2c to c2w.
                let world_to_cam = glam::Affine3A::from_rotation_translation(quat, translation);
                let cam_to_world = world_to_cam.inverse();
                let (_, quat, translation) = cam_to_world.to_scale_rotation_translation();

                let converted_cam =
                    Camera::new(translation, quat, glam::vec2(fovx, fovy), center_uv);

                let view = SceneView {
                    name: img_path.to_str().context("Invalid file name")?.to_owned(),
                    camera: converted_cam,
                    image: Arc::new(img),
                };
                anyhow::Result::<SceneView>::Ok(view)
            }
        })
        .collect();

    Ok(handles)
}

pub(crate) fn read_dataset_views(
    archive: ZipArchive<Cursor<ZipData>>,
    load_args: &LoadDatasetArgs,
) -> Result<DataStream<Dataset>> {
    let handles = read_views(archive, load_args)?;

    // 'real' colmap scenes are assumed to be opaque and not have a background, aka
    // a black background.
    let load_args = load_args.clone();
    let stream = stream_fut_parallel(handles);

    let mut train_views = vec![];
    let mut eval_views = vec![];

    let stream = stream.enumerate().map(move |(i, view)| {
        // I cannot wait for let chains.
        if let Some(eval_period) = load_args.eval_split_every {
            if i % eval_period == 0 {
                eval_views.push(view?);
            } else {
                train_views.push(view?);
            }
        } else {
            train_views.push(view?);
        }
        let background = Vec3::ZERO;

        Ok(Dataset::from_views(
            train_views.clone(),
            eval_views.clone(),
            background,
        ))
    });

    Ok(Box::pin(stream))
}

pub(crate) fn read_init_splat<B: Backend>(
    mut archive: ZipArchive<Cursor<ZipData>>,
    device: &B::Device,
    load_args: &LoadInitArgs,
) -> Result<DataStream<Splats<B>>> {
    let (is_binary, base_path) =
        if let Some(path) = find_base_path("sparse/0/cameras.bin", &archive) {
            (true, path)
        } else if let Some(path) = find_base_path("sparse/0/cameras.txt", &archive) {
            (false, path)
        } else {
            anyhow::bail!("No COLMAP data found (either text or binary.")
        };

    let points_path = if is_binary {
        base_path.join("sparse/0/points3D.bin")
    } else {
        base_path.join("sparse/0/points3D.txt")
    };

    // Extract COLMAP sfm points.
    let points_data = {
        let mut points_file = archive_file_at_path(&points_path, &mut archive)?;
        colmap_read_model::read_points3d(&mut points_file, is_binary)?
    };

    let device = device.clone();
    let sh_degree = load_args.sh_degree;

    let stream = try_fn_stream(|emitter| async move {
        let positions = points_data.values().map(|p| p.xyz).collect();
        let colors = points_data
            .values()
            .map(|p| Vec3::new(p.rgb[0] as f32, p.rgb[1] as f32, p.rgb[2] as f32) / 255.0)
            .collect();

        let splats = Splats::from_point_cloud(positions, colors, sh_degree, &device);
        emitter.emit(splats).await;
        Ok(())
    });

    Ok(Box::pin(stream))
}
