use wgpu_engine::*;

mod surface_manager;
pub use surface_manager::SurfaceManager;

mod display;
pub use display::Display;

mod surface;
pub use surface::*;

mod screen_task;
use screen_task::*;

mod shaders;

mod prepare_descriptors;

fn main(){
    env_logger::init();

    let features = wgpu::Features::EXTERNAL_MEMORY
        | wgpu::Features::PUSH_CONSTANTS
        | wgpu::Features::UNSIZED_BINDING_ARRAY
        | wgpu::Features::SAMPLED_TEXTURE_BINDING_ARRAY
        | wgpu::Features::SAMPLED_TEXTURE_ARRAY_DYNAMIC_INDEXING
        | wgpu::Features::SAMPLED_TEXTURE_ARRAY_NON_UNIFORM_INDEXING;

    let mut limits = wgpu::Limits::default();
    limits.max_push_constant_size = std::mem::size_of::<PushConstants>() as u32;

    wgpu_engine::quick_run(
        1,
        features,
        limits,
        |_id, _tokio_runtime, update_context| ScreenTask::new(update_context)
    );
}
