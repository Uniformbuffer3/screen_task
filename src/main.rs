use wgpu_engine::*;

mod surface_manager;
pub use surface_manager::SurfaceManager;

mod display;
pub use display::Display;

mod surface;
pub use surface::*;

mod screen_task;
use crate::screen_task::*;

mod shaders;

fn main() {
    env_logger::init();

    let mut features = wgpu_engine::Features::PUSH_CONSTANTS
        | wgpu_engine::Features::UNSIZED_BINDING_ARRAY
        | wgpu_engine::Features::SAMPLED_TEXTURE_BINDING_ARRAY
        | wgpu_engine::Features::SAMPLED_TEXTURE_ARRAY_DYNAMIC_INDEXING
        | wgpu_engine::Features::SAMPLED_TEXTURE_ARRAY_NON_UNIFORM_INDEXING;

    #[cfg(feature = "wgpu_custom_backend")]
    {
        features |= wgpu_engine::Features::EXTERNAL_MEMORY;
    }

    let mut limits = wgpu_engine::Limits::default();
    limits.max_push_constant_size = std::mem::size_of::<PushConstants>() as u32;

    wgpu_engine::quick_run(
        1,
        features,
        limits,
        |_id, _tokio_runtime, update_context| {
            let mut screen_task = ScreenTask::new(update_context);
            screen_task.create_surface(
                0,
                String::from("surface"),
                SurfaceSource::from_file_path(std::path::PathBuf::from("./gfx_logo.png")),
                [0, 0, 0],
                [100, 100],
            );
            screen_task.create_surface(
                1,
                String::from("surface"),
                SurfaceSource::from_file_path(std::path::PathBuf::from("./gfx_logo.png")),
                [50, 50, 1],
                [100, 100],
            );
            screen_task
        },
        |task| {},
    );
}
// Triangle sopra
