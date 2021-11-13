use crate::surface_manager::SurfaceManager;
use wgpu_engine::*;

impl SurfaceManager {
    pub fn prepare_texture_view(
        &self,
        label: String,
        texture: TextureId,
        format: wgpu::TextureFormat,
    ) -> TextureViewDescriptor {
        TextureViewDescriptor {
            device: self.device,
            label,
            texture,
            format,
            dimension: wgpu::TextureViewDimension::D2,
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        }
    }
}
