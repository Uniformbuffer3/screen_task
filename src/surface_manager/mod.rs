use crate::surface::{Surface, SurfaceInfo, SurfaceSource, SurfaceSourceInfo};
use wgpu_engine::*;

mod prepare_texture;
mod prepare_texture_view;
mod prepare_texture_write;

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
        source: SurfaceSource,
        position: [i32; 3],
        size: [u32; 2],
    ) {
        let info = SurfaceSourceInfo::from(&source);
        let (texture_descriptor, texture_data, layout) =
            Self::prepare_texture(self.device, label.clone(), source);

        let texture_format = texture_descriptor.format;
        let texture_size = texture_descriptor.size;
        let texture = update_context
            .add_texture_descriptor(texture_descriptor)
            .unwrap();

        let texture_view_descriptor =
            self.prepare_texture_view(label.clone(), texture, texture_format);
        let texture_view = update_context
            .add_texture_view_descriptor(texture_view_descriptor)
            .unwrap();

        if let Some(data) = texture_data {
            let texture_write = Self::prepare_texture_write(texture, data, texture_size, layout);
            update_context.write_resource(&mut vec![texture_write]);
        }


        let surface = SurfaceInfo::new(texture, texture_view, info, position, size);
        let surface_data = surface.generate_data(self.data_buffer.next_slot() as u32);
        self.data_buffer.request(id, surface, surface_data);
        self.stack.push(id);
    }

    pub fn update_source(
        &mut self,
        update_context: &mut UpdateContext,
        id: &usize,
        source: SurfaceSource,
    ) {
        let device = self.device;
        if let Some(surface_info) = self.data_buffer.associated_data_mut(id) {
            if let Some(texture_descriptor) =
                update_context.texture_descriptor_ref(&surface_info.texture_id)
            {
                let (texture_descriptor, texture_data, layout) =
                    Self::prepare_texture(device, texture_descriptor.label.clone(), source);
                let texture_size = texture_descriptor.size.clone();
                //update_context.update_texture_descriptor(&mut surface_info.texture_id,texture_descriptor);
                if let Some(data) = texture_data {
                    let texture_write = Self::prepare_texture_write(
                        surface_info.texture_id,
                        data,
                        texture_size,
                        layout,
                    );
                    update_context.write_resource(&mut vec![texture_write]);
                }
            };
        };
    }

    pub fn update_data(&mut self, update_context: &mut UpdateContext, id: &usize, data: Vec<u8>) {
        if let Some(surface_info) = self.data_buffer.associated_data_mut(id) {
            if let Some(texture_descriptor) =
                update_context.texture_descriptor_ref(&surface_info.texture_id)
            {
                let layout = wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(
                        texture_descriptor.format.describe().block_size as u32
                            * texture_descriptor.size.width,
                    ),
                    rows_per_image: std::num::NonZeroU32::new(texture_descriptor.size.height),
                };
                let texture_write = Self::prepare_texture_write(
                    surface_info.texture_id,
                    data,
                    texture_descriptor.size,
                    layout,
                );
                update_context.write_resource(&mut vec![texture_write]);
            }
        };
    }

    pub fn resize_surface(&mut self, id: &usize, size: [u32; 2]) -> bool {
        let size = [size[0] as f32, size[1] as f32];
        let offset = field_offset::offset_of!(Surface => size);
        self.data_buffer.pending_write_field(id, offset, size)
    }

    pub fn move_surface(&mut self, id: &usize, position: [i32; 3]) -> bool {
        let position = [position[0] as f32, position[1] as f32, position[2] as f32];
        let offset = field_offset::offset_of!(Surface => position);
        self.data_buffer.pending_write_field(id, offset, position)
    }

    pub fn remove_surface(&mut self, update_context: &mut UpdateContext, id: &usize) -> bool {
        self.data_buffer
            .release_pending(id)
            .map(|associated_data| {
                update_context
                    .remove_texture_view(&associated_data.texture_view_id)
                    .unwrap();
                update_context
                    .remove_texture(&associated_data.texture_id)
                    .unwrap();
                let index = self
                    .stack
                    .iter()
                    .position(|current_id| current_id == id)
                    .unwrap();
                self.stack.remove(index);
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
