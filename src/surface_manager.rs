use crate::surface::{Surface, SurfaceInfo, SurfaceSource};
use bytemuck::{Pod, Zeroable};
use std::path::PathBuf;
use wgpu_engine::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct SurfaceManager {
    device: DeviceId,
    id_counter: usize,
    stack: Vec<usize>,
    data_buffer: BufferManager<Surface, SurfaceInfo>,
}
impl SurfaceManager {
    pub fn new(update_context: &mut UpdateContext, device: DeviceId) -> Self {
        let id_counter = 0;
        let stack = Vec::new();
        let data_buffer = BufferManager::new(
            update_context,
            String::from("SurfaceManager buffer"),
            device,
            32,
            wgpu::BufferUsage::VERTEX,
        );
        Self {
            device,
            id_counter,
            stack,
            data_buffer,
        }
    }

    pub fn buffer_id(&self) -> &BufferId {
        self.data_buffer.id()
    }

    pub fn book_id(&mut self) -> usize {
        let id = self.id_counter;
        self.id_counter += 1;
        id
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn create_surface(
        &mut self,
        update_context: &mut UpdateContext,
        label: String,
        id: usize,
        source: Arc<SurfaceSource>,
        position: [u32; 3],
        size: [u32; 2],
    ) {
        let width;
        let height;
        let depth_or_array_layers;
        let sample_layout;
        let data;
        match source.as_ref() {
            SurfaceSource::File { path } => {
                use image::io::Reader as ImageReader;
                let img = ImageReader::open(path.clone())
                    .unwrap()
                    .decode()
                    .unwrap()
                    .into_rgba8();
                width = img.dimensions().0;
                height = img.dimensions().1;
                depth_or_array_layers = 1;
                sample_layout = img.sample_layout();
                data = img.into_raw();
            }
            _ => panic!(),
        }

        let texture_descriptor = TextureDescriptor {
            device: self.device,
            label: label.clone() + " texture",
            source: TextureSource::Local,
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        };
        let texture_id = update_context
            .add_texture_descriptor(texture_descriptor)
            .unwrap();

        let texture_view_descriptor = TextureViewDescriptor {
            device: self.device,
            label: label + " texture view",
            texture: texture_id,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            dimension: wgpu::TextureViewDimension::D2,
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        };
        let texture_view_id = update_context
            .add_texture_view_descriptor(texture_view_descriptor)
            .unwrap();

        let resource_write = ResourceWrite::Texture(TextureWrite {
            texture: texture_id,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            data,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(
                    sample_layout.width * sample_layout.channels as u32 * 1,
                ),
                rows_per_image: std::num::NonZeroU32::new(sample_layout.height),
            },
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers,
            },
        });
        update_context.write_resource(&mut vec![resource_write]);

        let surface = SurfaceInfo::new(texture_id, texture_view_id, source, position, size);
        let surface_data = surface.generate_data(self.data_buffer.next_slot() as u32);
        self.data_buffer.request(id, surface, surface_data);
        self.stack.push(id);
    }

    pub fn resize_surface(&mut self, id: &usize, size: [u32; 2]) -> bool {
        let size = [size[0] as f32, size[1] as f32];
        let offset = field_offset::offset_of!(Surface => size);
        self.data_buffer.pending_write_field(id, offset, size)
    }

    pub fn move_surface(&mut self, id: &usize, position: [u32; 3]) -> bool {
        let position = [position[0] as f32, position[1] as f32, position[2] as f32];
        let offset = field_offset::offset_of!(Surface => position);
        self.data_buffer.pending_write_field(id, offset, position)
    }

    pub fn remove_surface(&mut self, id: &usize) -> bool {
        self.data_buffer
            .release_pending(id)
            .map(|_associated_data| {
                let index = self
                    .stack
                    .iter()
                    .position(|current_id| current_id == id)
                    .unwrap();
                self.stack.swap_remove(index);
                Some(())
            })
            .is_some()
    }

    pub fn rectangle_views(&self) -> Vec<TextureViewId> {
        self.stack
            .iter()
            .map(|id| {
                self.data_buffer
                    .associated_data(id)
                    .unwrap()
                    .texture_view_id
            })
            .collect()
    }

    pub fn update(&mut self, update_context: &mut UpdateContext) -> Vec<Command> {
        self.data_buffer.update(update_context)
    }
}
