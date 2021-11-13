use wgpu_engine::*;

use crate::screen_task::device_resources::DeviceResources;
use crate::screen_task::ScreenTask;

impl ScreenTask {
    pub(crate) fn update_render_pipeline(
        update_context: &mut UpdateContext,
        device: DeviceId,
        device_resources: &mut DeviceResources,
    ) {
        for display_resources in &mut device_resources.displays {
            let (render_pipeline_descriptor, render_pipeline_ready) = Self::prepare_render_pipeline(
                update_context,
                device,
                &display_resources.display,
                device_resources.pipeline_layout,
                device_resources.vertex_shader,
                device_resources.fragment_shader,
                &device_resources.surface_manager,
            );
            if display_resources.render_pipeline_ready != render_pipeline_ready {
                update_context.update_render_pipeline_descriptor(
                    &mut display_resources.render_pipeline,
                    render_pipeline_descriptor,
                );
                display_resources.render_pipeline_ready = render_pipeline_ready;
            }
        }
    }
}
