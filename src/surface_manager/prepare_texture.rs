use crate::surface::SurfaceSource;
use crate::surface_manager::SurfaceManager;
use wgpu_engine::*;

impl SurfaceManager {
    /// Generate the texture descriptor, image data layout and the eventual data from a SurfaceSource.
    pub fn prepare_texture(
        device: DeviceId,
        label: String,
        source: SurfaceSource,
    ) -> (TextureDescriptor, Option<Vec<u8>>, wgpu::ImageDataLayout) {
        let width;
        let height;
        let depth_or_array_layers;
        let image_layout;
        let texture_data;
        let texture_source;
        let texture_format;

        match source {
            /*
            SurfaceSource::File { ref path } => {
                use image::io::Reader as ImageReader;
                let img = ImageReader::open(path.clone())
                    .unwrap()
                    .decode()
                    .unwrap()
                    .into_rgba8();
                width = img.dimensions().0;
                height = img.dimensions().1;
                depth_or_array_layers = 1;
                let sample_layout = img.sample_layout();

                image_layout = wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(
                        sample_layout.width * sample_layout.channels as u32 * 1,
                    ),
                    rows_per_image: std::num::NonZeroU32::new(sample_layout.height),
                };

                texture_data = Some(img.into_raw());
                texture_source = TextureSource::Local;
                texture_format = wgpu::TextureFormat::Rgba8UnormSrgb
            }
            */
            SurfaceSource::HostAllocation { info, ref data } => {
                width = info.size[0];
                height = info.size[1];
                depth_or_array_layers = 1;
                image_layout = wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(info.stride),
                    rows_per_image: std::num::NonZeroU32::new(height),
                };

                texture_data = Some(data.clone());
                texture_source = TextureSource::Local;
                texture_format = info.format
            }
            SurfaceSource::Dmabuf { info } => {
                width = info.size[0];
                height = info.size[1];
                depth_or_array_layers = 1;
                image_layout = wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(info.plane_stride),
                    rows_per_image: std::num::NonZeroU32::new(height),
                };
                texture_data = None;
                use std::os::unix::io::AsRawFd;

                let plane_layouts: Vec<wgpu_engine::PlaneLayout> = vec![PlaneLayout {
                    slice: info.plane_offset
                        ..(info.plane_stride * info.size[0] * info.size[1]) as u64,
                    row_pitch: info.plane_stride as u64,
                    array_pitch: 1,
                    depth_pitch: 1,
                }];

                let drm_properties = wgpu_engine::DrmFormatImageProperties {
                    drm_modifier: info.modifier,
                    plane_layouts,
                };
                texture_source = TextureSource::DmaBuf {
                    fd: info.fd.as_raw_fd(),
                    drm_properties: Some(drm_properties),
                    offset: 0,
                };
                texture_format = wgpu::TextureFormat::Rgba8UnormSrgb
            } /*
              SurfaceSource::OpaqueFd { offset, fd , size, stride, format}=>{
                  width = size[0];
                  height = size[1];
                  depth_or_array_layers = 1;
                  use std::os::unix::io::AsRawFd;
                  texture_source = TextureSource::OpaqueFd{fd: fd.as_raw_fd(), offset};
                  image_layout = wgpu::ImageDataLayout {
                      offset: 0,
                      bytes_per_row: std::num::NonZeroU32::new(stride),
                      rows_per_image: std::num::NonZeroU32::new(height),
                  };

                  texture_data = None;
                  texture_format = format;
              },
              */
        }

        let descriptor = TextureDescriptor {
            device,
            label: label.clone() + " texture",
            source: texture_source,
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: texture_format,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        };

        (descriptor, texture_data, image_layout)
    }
}
