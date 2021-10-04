use crate::surface_manager::SurfaceManager;
use crate::DisplayResources;
use crate::PushConstants;
use crate::ScreenTask;
use wgpu_engine::*;

impl ScreenTask {
    pub(crate) fn prepare_command_buffer(
        update_context: &mut UpdateContext,
        device: DeviceId,
        display_resources: &Vec<DisplayResources>,
        bind_group: BindGroupId,
        surface_manager: &SurfaceManager,
    ) -> CommandBufferDescriptor {
        let render_passes: Vec<_> = display_resources
            .iter()
            .map(|display_resources| Command::RenderPass {
                label: Self::TASK_NAME.to_string(),
                depth_stencil: Some(*display_resources.display.depth_stencil_view()),
                color_attachments: vec![RenderPassColorAttachment {
                    view: ColorView::Swapchain(*display_resources.display.swapchain()),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                }],
                commands: vec![
                    RenderCommand::SetPipeline {
                        pipeline: display_resources.render_pipeline,
                    },
                    RenderCommand::SetPushConstants {
                        stages: wgpu::ShaderStage::VERTEX,
                        offset: 0,
                        data: bytemuck::bytes_of(&PushConstants::new(
                            display_resources.display.size(),
                            1024,
                        ))
                        .to_vec(),
                    },
                    RenderCommand::SetBindGroup {
                        index: 0,
                        bind_group: bind_group,
                        offsets: Vec::new(),
                    },
                    RenderCommand::SetVertexBuffer {
                        slot: 0,
                        buffer: *surface_manager.buffer_id(),
                        slice: Slice::from(..),
                    },
                    RenderCommand::Draw {
                        vertices: 0..4,
                        instances: 0..surface_manager.len() as u32,
                    },
                ],
            })
            .collect();

        CommandBufferDescriptor {
            device,
            label: Self::TASK_NAME.to_string() + " command buffer",
            commands: render_passes,
        }
    }
}
