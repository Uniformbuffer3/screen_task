use crate::display::DisplayResources;
use crate::surface_manager::SurfaceManager;
use wgpu_engine::*;


/// Rendering resources related to a single device.
pub struct DeviceResources {
    pub displays: Vec<DisplayResources>,

    pub surface_manager: SurfaceManager,

    pub fragment_shader: ShaderModuleId,
    pub vertex_shader: ShaderModuleId,
    pub sampler: SamplerId,

    pub bind_group_layout: BindGroupLayoutId,
    pub bind_group: BindGroupId,

    pub pipeline_layout: PipelineLayoutId,

    pub command_buffer: CommandBufferId,
    pub data_copy_command_buffer: CommandBufferId,
    pub data_copy_command_buffer_updated: bool,
}
