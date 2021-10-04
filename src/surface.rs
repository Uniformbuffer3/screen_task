use bytemuck::{Pod, Zeroable};
use std::path::PathBuf;
use wgpu_engine::*;
use std::sync::Arc;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Surface {
    pub position: [f32; 3],
    pub size: [f32; 2],
    pub image_index: u32,
}

#[derive(Debug)]
pub enum SurfaceSource {
    File { path: PathBuf },
    Dmabuf { fd: std::fs::File },
}

#[derive(Debug)]
pub struct SurfaceInfo {
    pub texture_id: TextureId,
    pub texture_view_id: TextureViewId,

    pub source: Arc<SurfaceSource>,
    pub position: [f32; 3],
    pub size: [f32; 2],
}
impl SurfaceInfo {
    pub fn new(
        texture_id: TextureId,
        texture_view_id: TextureViewId,
        source: Arc<SurfaceSource>,
        position: [u32; 3],
        size: [u32; 2],
    ) -> Self {
        let position = [position[0] as f32, position[1] as f32, position[2] as f32];
        let size = [size[0] as f32, size[1] as f32];
        Self {
            source,
            position,
            size,
            texture_id,
            texture_view_id,
        }
    }

    pub fn generate_data(&self, image_index: u32) -> Surface {
        Surface {
            position: self.position,
            size: self.size,
            image_index,
        }
    }
}
