use crate::surface_manager::SurfaceManager;
use wgpu_engine::*;

impl SurfaceManager {
    pub fn prepare_texture_write(
        texture: TextureId,
        data: Vec<u8>,
        size: wgpu::Extent3d,
        layout: wgpu::ImageDataLayout,
    ) -> ResourceWrite {
        ResourceWrite::Texture(TextureWrite {
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            data,
            layout,
            size,
        })
    }
}
