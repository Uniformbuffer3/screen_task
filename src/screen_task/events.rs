use wgpu_engine::*;

use crate::screen_task::ScreenTask;
use crate::surface::*;

pub enum ScreenTaskEvent {
    CreateSurface {
        id: usize,
        label: String,
        source: SurfaceSource,
        position: [i32; 3],
        size: [u32; 2],
    },
    UpdateSource {
        id: usize,
        source: SurfaceSource,
    },
    UpdateData {
        id: usize,
        data: Vec<u8>,
    },
    ResizeSurface {
        id: usize,
        size: [u32; 2],
    },
    MoveSurface {
        id: usize,
        position: [i32; 3],
    },
    RemoveSurface {
        id: usize,
    },
}

impl ScreenTask {
    pub(crate) fn elaborate_events(&mut self, update_context: &mut UpdateContext) {
        let mut update_resource_needed = false;
        for event in self.pending_events.drain(..) {
            match event {
                ScreenTaskEvent::CreateSurface {
                    id,
                    label,
                    source,
                    position,
                    size,
                } => {
                    self.devices.values_mut().for_each(|device_resources| {
                        device_resources.surface_manager.create_surface(
                            update_context,
                            label.clone(),
                            id,
                            source.clone(),
                            position,
                            size,
                        );
                    });
                    update_resource_needed = true;
                }
                ScreenTaskEvent::UpdateSource { id, source } => {
                    self.devices.values_mut().for_each(|device_resources| {
                        device_resources.surface_manager.update_source(
                            update_context,
                            &id,
                            source.clone(),
                        );
                    });
                }
                ScreenTaskEvent::UpdateData { id, data } => {
                    self.devices.values_mut().for_each(|device_resources| {
                        device_resources.surface_manager.update_data(
                            update_context,
                            &id,
                            data.clone(),
                        );
                    });
                }
                ScreenTaskEvent::ResizeSurface { id, size } => {
                    self.devices.values_mut().for_each(|device_resources| {
                        assert!(device_resources.surface_manager.resize_surface(&id, size));
                    });
                }
                ScreenTaskEvent::MoveSurface { id, position } => {
                    self.devices.values_mut().for_each(|device_resources| {
                        assert!(device_resources.surface_manager.move_surface(&id, position));
                    });
                }
                ScreenTaskEvent::RemoveSurface { id } => {
                    self.devices.values_mut().for_each(|device_resources| {
                        assert!(device_resources
                            .surface_manager
                            .remove_surface(update_context, &id));
                    });
                    update_resource_needed = true;
                }
            }
        }

        self.devices
            .iter_mut()
            .for_each(|(device, device_resources)| {
                if update_resource_needed {
                    Self::update_layout_and_bind_groups(update_context, *device, device_resources);
                }

                let commands = device_resources.surface_manager.update(update_context);
                if !commands.is_empty() {
                    let data_copy_command_buffer_descriptor = CommandBufferDescriptor {
                        device: *device,
                        label: Self::TASK_NAME.to_string() + " data copy command buffer",
                        commands,
                    };
                    update_context.update_command_buffer_descriptor(
                        &mut device_resources.data_copy_command_buffer,
                        data_copy_command_buffer_descriptor,
                    );
                    device_resources.data_copy_command_buffer_updated = true;
                }
            })
    }
}
