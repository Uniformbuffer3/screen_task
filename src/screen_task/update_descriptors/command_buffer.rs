use wgpu_engine::*;

use crate::screen_task::device_resources::DeviceResources;
use crate::screen_task::ScreenTask;

impl ScreenTask {
    pub(crate) fn update_command_buffer(
        update_context: &mut UpdateContext,
        device: DeviceId,
        device_resources: &mut DeviceResources,
    ) {
        let command_buffer_descriptor = Self::prepare_command_buffer(
            update_context,
            device,
            &device_resources.displays,
            device_resources.bind_group,
            &device_resources.surface_manager,
        );
        update_context.update_command_buffer_descriptor(
            &mut device_resources.command_buffer,
            command_buffer_descriptor,
        );
    }
}
