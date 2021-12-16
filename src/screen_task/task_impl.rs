use std::collections::hash_map::Entry;
use wgpu_engine::*;

use crate::display::{Display, DisplayResources};
use crate::screen_task::ScreenTask;

impl TaskTrait for ScreenTask {
    fn name(&self) -> String {
        Self::TASK_NAME.to_string()
    }

    fn update_resources(&mut self, update_context: &mut UpdateContext) {
        self.devices.values_mut().for_each(|device_resources| {
            device_resources.data_copy_command_buffer_updated = false;
        });

        let events = update_context.events().clone();
        events.iter().for_each(|event| match event {
            ResourceEvent::SwapchainCreated {
                external_id,
                swapchain,
            } => {
                let device = update_context.entity_device_id(swapchain).unwrap();
                match self.devices.entry(device) {
                    Entry::Vacant(vacant) => {
                        let resources = Self::init_device_resources(
                            update_context,
                            *external_id,
                            device,
                            *swapchain,
                        );
                        vacant.insert(resources);
                    }
                    Entry::Occupied(mut occupied) => {
                        let device_resources = occupied.get_mut();
                        let display =
                            Display::new(update_context, *external_id, device, *swapchain, [0, 0]);
                        let display_resources = DisplayResources::new(
                            update_context,
                            display,
                            device_resources.pipeline_layout,
                            device_resources.vertex_shader,
                            device_resources.fragment_shader,
                            &device_resources.surface_manager,
                        );

                        device_resources.displays.push(display_resources);

                        Self::update_command_buffer(update_context, device, device_resources);
                    }
                }
            }
            ResourceEvent::SwapchainDestroyed(swapchain) => {
                self.devices.retain(|device, device_resources| {
                    if let Some(index) =
                        device_resources
                            .displays
                            .iter()
                            .position(|display_resources| {
                                display_resources.display.swapchain() == swapchain
                            })
                    {
                        device_resources.displays.remove(index);
                        if !device_resources.displays.is_empty() {
                            Self::update_command_buffer(update_context, *device, device_resources);
                            true
                        } else {
                            false
                        }
                    } else {
                        true
                    }
                });
            }
            ResourceEvent::SwapchainUpdated(swapchain) => {
                self.devices
                    .iter_mut()
                    .find_map(|(device, device_resources)| {
                        Self::update_command_buffer(update_context, *device, device_resources);
                        let result = device_resources.displays.iter_mut().find_map(|display| {
                            if display.display.swapchain() == swapchain {
                                display.display.update(update_context);
                                Some(())
                            } else {
                                None
                            }
                        });
                        //
                        result
                    });
            }
        });

        self.elaborate_events(update_context);
    }
    fn command_buffers(&self) -> Vec<CommandBufferId> {
        self.devices
            .values()
            .map(|device_resources| {
                let mut cbs = Vec::new();
                cbs.push(device_resources.command_buffer);
                //if device_resources.surface_manager.len() > 0 {}
                if device_resources.data_copy_command_buffer_updated {
                    cbs.push(device_resources.data_copy_command_buffer);
                }
                cbs
            })
            .flatten()
            .collect()
    }
}
