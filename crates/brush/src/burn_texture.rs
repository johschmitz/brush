use std::sync::Arc;

use brush_kernel::BurnBack;
use burn::tensor::Tensor;
use egui::TextureId;
use wgpu::ImageDataLayout;

fn copy_buffer_to_texture(img: Tensor<BurnBack, 3>, texture: &wgpu::Texture) {
    let [height, width, _] = img.shape().dims;
    let primitive = img.into_primitive();

    primitive.client.run_custom_command(move |server| {
        let img_res = server.get_resource_binding(primitive.handle.clone().binding());

        // Put compute passes in encoder before copying the buffer.
        let bytes_per_row = Some(4 * width as u32);
        let encoder = server.get_command_encoder();

        // Now copy the buffer to the texture.
        encoder.copy_buffer_to_texture(
            wgpu::ImageCopyBuffer {
                buffer: img_res.buffer.as_ref(),
                layout: ImageDataLayout {
                    offset: 0,
                    bytes_per_row,
                    rows_per_image: None,
                },
            },
            wgpu::ImageCopyTexture {
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: width as u32,
                height: height as u32,
                depth_or_array_layers: 1,
            },
        );
    });
}

pub struct BurnTexture {
    pub texture: wgpu::Texture,
    pub id: TextureId,
}

fn create_texture(size: glam::UVec2, device: Arc<wgpu::Device>) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Splat backbuffer"),
        size: wgpu::Extent3d {
            width: size.x,
            height: size.y,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
    })
}

// TODO: Slightly awkward combination of tensor <-> texture and texture <-> egui.
impl BurnTexture {
    pub fn new(tensor: Tensor<BurnBack, 3>, frame: &eframe::Frame) -> Self {
        let render_state = frame.wgpu_render_state().unwrap();
        let device = render_state.device.clone();
        let [h, w, _] = tensor.shape().dims;
        let texture = create_texture(glam::uvec2(w as u32, h as u32), device.clone());
        let view = texture.create_view(&Default::default());
        let mut renderer = render_state.renderer.write();
        let id = renderer.register_native_texture(&device, &view, wgpu::FilterMode::Linear);
        Self { texture, id }
    }

    pub fn update_texture(&mut self, tensor: Tensor<BurnBack, 3>, frame: &eframe::Frame) {
        let [h, w, _] = tensor.shape().dims;
        let size = glam::uvec2(w as u32, h as u32);

        let render_state = frame.wgpu_render_state().unwrap();
        let device = render_state.device.clone();

        let dirty = self.texture.width() != size.x || self.texture.height() != size.y;

        if dirty {
            self.texture = create_texture(glam::uvec2(w as u32, h as u32), device.clone());
            let mut renderer = render_state.renderer.write();

            renderer.update_egui_texture_from_wgpu_texture(
                &device,
                &self.texture.create_view(&Default::default()),
                wgpu::FilterMode::Linear,
                self.id,
            )
        }

        copy_buffer_to_texture(tensor, &self.texture);
    }
}