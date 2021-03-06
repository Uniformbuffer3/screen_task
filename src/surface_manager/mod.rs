use crate::surface::{Surface, SurfaceInfo, SurfaceSource, SurfaceSourceInfo};
use wgpu_engine::*;

mod prepare_texture;
mod prepare_texture_view;
mod prepare_texture_write;

#[derive(Debug)]
/**
Manager responsible to correctly manage rendering resources (like buffers) where surface related data are stored,
so it is also responsible to correctly synchronize the data with the gpu.
*/
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

    /// Returns the underlying BufferId.
    pub fn buffer_id(&self) -> &BufferId {
        self.data_buffer.id()
    }

    /// Book a identifier for a surface id. The returned id will not be assigned to other surfaces.
    pub fn book_id(&mut self) -> usize {
        let id = self.id_counter;
        self.id_counter += 1;
        id
    }

    /// Returns how many surfaces are stored.
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Create a new surface and assign it the provided id.
    pub fn create_surface(
        &mut self,
        update_context: &mut UpdateContext,
        label: String,
        id: usize,
        source: SurfaceSource,
        position: [i32; 3],
        size: [u32; 2],
    ) {
        log::info!(target: "ScreenTask","Creating surface {}",id);
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
        let next_slot = self.data_buffer.next_slot() as u32;
        let surface_data = surface.generate_data(next_slot);
        self.data_buffer.request(id, surface, surface_data);
        self.stack.push(id);
    }

    /// Update the source of the surface with the provided id.
    pub fn update_source(
        &mut self,
        update_context: &mut UpdateContext,
        id: &usize,
        source: SurfaceSource,
    ) {
        log::info!(target: "ScreenTask","Updating source of surface {}",id);
        let device = self.device;
        if let Some(surface_info) = self.data_buffer.associated_data_mut(id) {
            if let Some(texture_descriptor) =
                update_context.texture_descriptor_ref(&surface_info.texture_id)
            {
                let (texture_descriptor, texture_data, layout) =
                    Self::prepare_texture(device, texture_descriptor.label.clone(), source);
                let texture_size = texture_descriptor.size.clone();
                update_context
                    .update_texture_descriptor(&mut surface_info.texture_id, texture_descriptor);

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
        } else {
            println!("Failed");
        }
    }

    /// Update the data of the surface with the provided id.
    pub fn update_data(&mut self, update_context: &mut UpdateContext, id: &usize, data: Vec<u8>) {
        log::info!(target: "ScreenTask","Updating data of surface {}",id);
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

    /// Resize the surface with the provided id.
    pub fn resize_surface(&mut self, id: &usize, size: [u32; 2]) -> bool {
        log::info!(target: "ScreenTask","Resizing surface {} to {:?}",id,size);
        let size = [size[0] as f32, size[1] as f32];
        let offset = field_offset::offset_of!(Surface => size);
        self.data_buffer.pending_write_field(id, offset, size)
    }

    /// Move the surface with the provided id.
    pub fn move_surface(&mut self, id: &usize, position: [i32; 3]) -> bool {
        log::info!(target: "ScreenTask","Moving surface {} to {:?}",id,position);
        let position = [position[0] as f32, position[1] as f32, position[2] as f32];
        let offset = field_offset::offset_of!(Surface => position);
        self.data_buffer.pending_write_field(id, offset, position)
    }

    /// Remove the surface with the provided id.
    pub fn remove_surface(&mut self, update_context: &mut UpdateContext, id: &usize) -> bool {
        log::info!(target: "ScreenTask","Removing surface {}",id);
        if let Some(associated_data) = self.data_buffer.release_pending(id) {
            let index = self
                .stack
                .iter()
                .position(|current_id| current_id == id)
                .unwrap();
            self.stack.remove(index);
            update_context
                .remove_texture_view(&associated_data.texture_view_id)
                .unwrap();
            update_context
                .remove_texture(&associated_data.texture_id)
                .unwrap();

            true
        } else {
            log::error!(target: "ScreenTask","Failed to remove surface {} because it does not exists",id);
            false
        }
    }

    /// Returns the TextureViewIds of all the stored
    pub fn rectangle_views(&self) -> Vec<TextureViewId> {
        self.stack
            .iter()
            .map(|id| {
                self.data_buffer
                    .associated_data(id)
                    .map(|associated_data| associated_data.texture_view_id)
                    .unwrap()
            })
            .collect()
    }

    /// Update the texture indexs associated with the surfaces.
    pub fn update_image_indexes(&mut self) {
        self.stack
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(image_index, id)| {
                let offset = field_offset::offset_of!(Surface => image_index);
                self.data_buffer
                    .pending_write_field(&id, offset, image_index as u32);
            });
    }

    /// Update buffer data and returns eventual commands that need to be scheduled with a command buffer.
    pub fn update(&mut self, update_context: &mut UpdateContext) -> Vec<Command> {
        self.data_buffer.update(update_context)
    }
}
