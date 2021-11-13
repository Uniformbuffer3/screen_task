use bytemuck::{Pod, Zeroable};
use std::path::PathBuf;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Surface {
    pub position: [f32; 3],
    pub size: [f32; 2],
    pub image_index: u32,
}

#[derive(Debug, Clone)]
pub enum SurfaceSource {
    //File { path: PathBuf },
    Dmabuf {
        info: DmabufInfo,
    },
    HostAllocation {
        info: HostAllocationInfo,
        data: Vec<u8>,
    },
    /*
    OpaqueFd {
        offset: u64,
        fd: std::os::unix::io::RawFd,
        size: [u32;2],
        stride: u32,
        format: wgpu_engine::TextureFormat
    },
    */
}
impl SurfaceSource {
    pub fn from_file_path(path: PathBuf) -> Self {
        use image::io::Reader as ImageReader;
        let img = ImageReader::open(path.clone())
            .unwrap()
            .decode()
            .unwrap()
            .into_rgba8();

        let sample_layout = img.sample_layout();

        let size = [img.dimensions().0, img.dimensions().1];
        let format = crate::wgpu::TextureFormat::Rgba8UnormSrgb;
        let stride = sample_layout.width * sample_layout.channels as u32 * 1;

        let info = HostAllocationInfo {
            size,
            format,
            stride,
        };
        let data = img.into_raw();
        Self::HostAllocation { info, data }
    }
}

#[derive(Debug, Clone)]
pub enum SurfaceSourceInfo {
    Dmabuf(DmabufInfo),
    HostAllocation(HostAllocationInfo),
}
impl From<DmabufInfo> for SurfaceSourceInfo {
    fn from(info: DmabufInfo) -> Self {
        Self::Dmabuf(info)
    }
}
impl From<HostAllocationInfo> for SurfaceSourceInfo {
    fn from(info: HostAllocationInfo) -> Self {
        Self::HostAllocation(info)
    }
}
impl From<SurfaceSource> for SurfaceSourceInfo {
    fn from(source: SurfaceSource) -> Self {
        Self::from(&source)
    }
}
impl From<&SurfaceSource> for SurfaceSourceInfo {
    fn from(source: &SurfaceSource) -> Self {
        match source {
            SurfaceSource::Dmabuf { info } => Self::Dmabuf(info.clone()),
            SurfaceSource::HostAllocation { info, .. } => Self::HostAllocation(info.clone()),
        }
    }
}


#[derive(Debug, Clone)]
pub struct DmabufInfo {
    pub size: [u32; 2],
    pub modifier: wgpu_engine::DrmModifier,
    pub fd: std::os::unix::io::RawFd,
    pub plane_offset: u64,
    pub plane_stride: u32,
}

#[derive(Debug, Clone)]
pub struct HostAllocationInfo {
    pub size: [u32; 2],
    pub format: wgpu_engine::TextureFormat,
    pub stride: u32,
}

#[derive(Debug)]
pub struct SurfaceInfo {
    pub texture_id: wgpu_engine::TextureId,
    pub texture_view_id: wgpu_engine::TextureViewId,

    pub info: SurfaceSourceInfo,
    pub position: [f32; 3],
    pub size: [f32; 2],
}
impl SurfaceInfo {
    pub fn new(
        texture_id: wgpu_engine::TextureId,
        texture_view_id: wgpu_engine::TextureViewId,
        info: SurfaceSourceInfo,
        position: [i32; 3],
        size: [u32; 2],
    ) -> Self {
        let position = [position[0] as f32, position[1] as f32, position[2] as f32];
        let size = [size[0] as f32, size[1] as f32];
        Self {
            info,
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
