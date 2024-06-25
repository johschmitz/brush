// Autogenerated by brush-wgsl from source wgsl files. Do not edit.
#![allow(dead_code, clippy::all)]
fn create_composer() -> naga_oil::compose::Composer {
    let mut composer = naga_oil::compose::Composer::default();
    composer.add_composable_module(naga_oil::compose::ComposableModuleDescriptor {
        source: include_str!("./helpers.wgsl"),
        file_path: "helpers.wgsl",
        as_name: Some("helpers".to_string()),
        ..Default::default()
    }).unwrap();
    composer.add_composable_module(naga_oil::compose::ComposableModuleDescriptor {
        source: include_str!("./grads.wgsl"),
        file_path: "grads.wgsl",
        as_name: Some("grads".to_string()),
        ..Default::default()
    }).unwrap();
    composer
}
pub(crate) mod gather_grads {
    pub(crate) const WORKGROUP_SIZE: [u32; 3] = [256, 1, 1];
    #[repr(C, align(16))]
    #[derive(bytemuck::Pod, bytemuck::Zeroable, Debug, PartialEq, Clone, Copy)]
    pub(crate) struct ShCoeffs {
            pub(crate) b0_c0: [f32; 4],
            pub(crate) b1_c0: [f32; 4],
            pub(crate) b1_c1: [f32; 4],
            pub(crate) b1_c2: [f32; 4],
            pub(crate) b2_c0: [f32; 4],
            pub(crate) b2_c1: [f32; 4],
            pub(crate) b2_c2: [f32; 4],
            pub(crate) b2_c3: [f32; 4],
            pub(crate) b2_c4: [f32; 4],
            pub(crate) b3_c0: [f32; 4],
            pub(crate) b3_c1: [f32; 4],
            pub(crate) b3_c2: [f32; 4],
            pub(crate) b3_c3: [f32; 4],
            pub(crate) b3_c4: [f32; 4],
            pub(crate) b3_c5: [f32; 4],
            pub(crate) b3_c6: [f32; 4],
            pub(crate) b4_c0: [f32; 4],
            pub(crate) b4_c1: [f32; 4],
            pub(crate) b4_c2: [f32; 4],
            pub(crate) b4_c3: [f32; 4],
            pub(crate) b4_c4: [f32; 4],
            pub(crate) b4_c5: [f32; 4],
            pub(crate) b4_c6: [f32; 4],
            pub(crate) b4_c7: [f32; 4],
            pub(crate) b4_c8: [f32; 4],
    }
    
    pub(crate) fn create_shader_source(
       shader_defs: std::collections::HashMap<String, naga_oil::compose::ShaderDefValue>
    ) -> naga::Module {
        super::create_composer().make_naga_module(naga_oil::compose::NagaModuleDescriptor {
            source: include_str!("gather_grads.wgsl"),
            file_path: "src/shaders/gather_grads.wgsl",
            shader_defs,
            ..Default::default()
        }).unwrap()
    }
}
pub(crate) mod get_tile_bin_edges {
    pub(crate) const WORKGROUP_SIZE: [u32; 3] = [256, 1, 1];
    pub(crate) const THREAD_COUNT: u32 = 256;
    pub(crate) const VERTICAL_GROUPS: u32 = 8;
    
    pub(crate) fn create_shader_source(
       shader_defs: std::collections::HashMap<String, naga_oil::compose::ShaderDefValue>
    ) -> naga::Module {
        super::create_composer().make_naga_module(naga_oil::compose::NagaModuleDescriptor {
            source: include_str!("get_tile_bin_edges.wgsl"),
            file_path: "src/shaders/get_tile_bin_edges.wgsl",
            shader_defs,
            ..Default::default()
        }).unwrap()
    }
}
pub(crate) mod grads {
    #[repr(C, align(4))]
    #[derive(bytemuck::Pod, bytemuck::Zeroable, Debug, PartialEq, Clone, Copy)]
    pub(crate) struct ScatterGradient {
            pub(crate) x: f32,
            pub(crate) y: f32,
            pub(crate) conic_x: f32,
            pub(crate) conic_y: f32,
            pub(crate) conic_z: f32,
            pub(crate) r: f32,
            pub(crate) g: f32,
            pub(crate) b: f32,
            pub(crate) a: f32,
    }
}
pub(crate) mod helpers {
    pub(crate) const COV_BLUR: f32 = 0.3 as f32;
    #[repr(C, align(4))]
    #[derive(bytemuck::Pod, bytemuck::Zeroable, Debug, PartialEq, Clone, Copy)]
    pub(crate) struct PackedVec3 {
            pub(crate) x: f32,
            pub(crate) y: f32,
            pub(crate) z: f32,
    }
    #[repr(C, align(4))]
    #[derive(bytemuck::Pod, bytemuck::Zeroable, Debug, PartialEq, Clone, Copy)]
    pub(crate) struct ProjectedSplat {
            pub(crate) x: f32,
            pub(crate) y: f32,
            pub(crate) conic_x: f32,
            pub(crate) conic_y: f32,
            pub(crate) conic_z: f32,
            pub(crate) r: f32,
            pub(crate) g: f32,
            pub(crate) b: f32,
            pub(crate) a: f32,
    }
    #[repr(C, align(16))]
    #[derive(bytemuck::Pod, bytemuck::Zeroable, Debug, PartialEq, Clone, Copy)]
    pub(crate) struct RenderUniforms {
            pub(crate) viewmat: [[f32; 4]; 4],
            pub(crate) focal: [f32; 2],
            pub(crate) img_size: [u32; 2],
            pub(crate) tile_bounds: [u32; 2],
            pub(crate) pixel_center: [f32; 2],
            pub(crate) background: [f32; 4],
            pub(crate) sh_degree: u32,
            pub(crate) num_visible: u32,
            pub(crate) total_splats: u32,
            pub(crate) padding: u32,
    }
    pub(crate) const TILE_SIZE: u32 = 400;
    pub(crate) const TILE_WIDTH: u32 = 20;
}
pub(crate) mod map_gaussian_to_intersects {
    pub(crate) const WORKGROUP_SIZE: [u32; 3] = [256, 1, 1];
    
    pub(crate) fn create_shader_source(
       shader_defs: std::collections::HashMap<String, naga_oil::compose::ShaderDefValue>
    ) -> naga::Module {
        super::create_composer().make_naga_module(naga_oil::compose::NagaModuleDescriptor {
            source: include_str!("map_gaussian_to_intersects.wgsl"),
            file_path: "src/shaders/map_gaussian_to_intersects.wgsl",
            shader_defs,
            ..Default::default()
        }).unwrap()
    }
}
pub(crate) mod project_backwards {
    pub(crate) const WORKGROUP_SIZE: [u32; 3] = [256, 1, 1];
    
    pub(crate) fn create_shader_source(
       shader_defs: std::collections::HashMap<String, naga_oil::compose::ShaderDefValue>
    ) -> naga::Module {
        super::create_composer().make_naga_module(naga_oil::compose::NagaModuleDescriptor {
            source: include_str!("project_backwards.wgsl"),
            file_path: "src/shaders/project_backwards.wgsl",
            shader_defs,
            ..Default::default()
        }).unwrap()
    }
}
pub(crate) mod project_forward {
    pub(crate) const WORKGROUP_SIZE: [u32; 3] = [256, 1, 1];
    
    pub(crate) fn create_shader_source(
       shader_defs: std::collections::HashMap<String, naga_oil::compose::ShaderDefValue>
    ) -> naga::Module {
        super::create_composer().make_naga_module(naga_oil::compose::NagaModuleDescriptor {
            source: include_str!("project_forward.wgsl"),
            file_path: "src/shaders/project_forward.wgsl",
            shader_defs,
            ..Default::default()
        }).unwrap()
    }
}
pub(crate) mod project_visible {
    pub(crate) const WORKGROUP_SIZE: [u32; 3] = [256, 1, 1];
    #[repr(C, align(16))]
    #[derive(bytemuck::Pod, bytemuck::Zeroable, Debug, PartialEq, Clone, Copy)]
    pub(crate) struct ShCoeffs {
            pub(crate) b0_c0: [f32; 4],
            pub(crate) b1_c0: [f32; 4],
            pub(crate) b1_c1: [f32; 4],
            pub(crate) b1_c2: [f32; 4],
            pub(crate) b2_c0: [f32; 4],
            pub(crate) b2_c1: [f32; 4],
            pub(crate) b2_c2: [f32; 4],
            pub(crate) b2_c3: [f32; 4],
            pub(crate) b2_c4: [f32; 4],
            pub(crate) b3_c0: [f32; 4],
            pub(crate) b3_c1: [f32; 4],
            pub(crate) b3_c2: [f32; 4],
            pub(crate) b3_c3: [f32; 4],
            pub(crate) b3_c4: [f32; 4],
            pub(crate) b3_c5: [f32; 4],
            pub(crate) b3_c6: [f32; 4],
            pub(crate) b4_c0: [f32; 4],
            pub(crate) b4_c1: [f32; 4],
            pub(crate) b4_c2: [f32; 4],
            pub(crate) b4_c3: [f32; 4],
            pub(crate) b4_c4: [f32; 4],
            pub(crate) b4_c5: [f32; 4],
            pub(crate) b4_c6: [f32; 4],
            pub(crate) b4_c7: [f32; 4],
            pub(crate) b4_c8: [f32; 4],
    }
    
    pub(crate) fn create_shader_source(
       shader_defs: std::collections::HashMap<String, naga_oil::compose::ShaderDefValue>
    ) -> naga::Module {
        super::create_composer().make_naga_module(naga_oil::compose::NagaModuleDescriptor {
            source: include_str!("project_visible.wgsl"),
            file_path: "src/shaders/project_visible.wgsl",
            shader_defs,
            ..Default::default()
        }).unwrap()
    }
}
pub(crate) mod rasterize {
    pub(crate) const WORKGROUP_SIZE: [u32; 3] = [20, 20, 1];
    
    pub(crate) fn create_shader_source(
       shader_defs: std::collections::HashMap<String, naga_oil::compose::ShaderDefValue>
    ) -> naga::Module {
        super::create_composer().make_naga_module(naga_oil::compose::NagaModuleDescriptor {
            source: include_str!("rasterize.wgsl"),
            file_path: "src/shaders/rasterize.wgsl",
            shader_defs,
            ..Default::default()
        }).unwrap()
    }
}
pub(crate) mod rasterize_backwards {
    pub(crate) const WORKGROUP_SIZE: [u32; 3] = [20, 20, 1];
    
    pub(crate) fn create_shader_source(
       shader_defs: std::collections::HashMap<String, naga_oil::compose::ShaderDefValue>
    ) -> naga::Module {
        super::create_composer().make_naga_module(naga_oil::compose::NagaModuleDescriptor {
            source: include_str!("rasterize_backwards.wgsl"),
            file_path: "src/shaders/rasterize_backwards.wgsl",
            shader_defs,
            ..Default::default()
        }).unwrap()
    }
}
