use crate::PushConstants;
use crate::ScreenTask;
use wgpu_engine::*;

impl ScreenTask {
    /// Generate the pipeline layout descriptor.
    pub(crate) fn prepare_pipeline_layout(
        _update_context: &mut UpdateContext,
        device: DeviceId,
        bind_group_layout: BindGroupLayoutId,
    ) -> PipelineLayoutDescriptor {
        log::info!(target: "ScreenTask","Preparing pipeline layout descriptor");
        let aligned_size = ((std::mem::size_of::<PushConstants>() + 4 - 1) / 4) * 4;
        PipelineLayoutDescriptor {
            device,
            label: Self::TASK_NAME.to_string() + " pipeline layout",
            bind_group_layouts: vec![bind_group_layout],
            push_constant_ranges: vec![wgpu::PushConstantRange {
                stages: wgpu::ShaderStage::VERTEX,
                range: 0..aligned_size as u32,
            }],
        }
    }
}
