use wgpu_engine::*;

use crate::screen_task::device_resources::DeviceResources;
use crate::screen_task::ScreenTask;

impl ScreenTask {
    pub(crate) fn update_layout_and_bind_groups(
        update_context: &mut UpdateContext,
        device: DeviceId,
        device_resources: &mut DeviceResources,
    ) {
        let bind_group_layout_descriptor = Self::prepare_bind_group_layout(
            update_context,
            device,
            &device_resources.surface_manager,
        );
        update_context.update_bind_group_layout_descriptor(
            &mut device_resources.bind_group_layout,
            bind_group_layout_descriptor,
        );

        let pipeline_layout_descriptor = Self::prepare_pipeline_layout(
            update_context,
            device,
            device_resources.bind_group_layout,
        );
        update_context.update_pipeline_layout_descriptor(
            &mut device_resources.pipeline_layout,
            pipeline_layout_descriptor,
        );

        device_resources.surface_manager.update_image_indexes();
        let bind_group_descriptor = Self::prepare_bind_group(
            update_context,
            device,
            &device_resources.surface_manager,
            device_resources.bind_group_layout,
            device_resources.sampler,
        );
        update_context
            .update_bind_group_descriptor(&mut device_resources.bind_group, bind_group_descriptor);

        Self::update_render_pipeline(update_context, device, device_resources);
        Self::update_command_buffer(update_context, device, device_resources);
    }
}
