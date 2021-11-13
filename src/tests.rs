use crate::PushConstants;
use crate::ScreenTask;
use crate::SurfaceSource;
use wgpu_engine::*;

#[test]
fn swap_test() {
    env_logger::init();

    let features = wgpu::Features::EXTERNAL_MEMORY
        | wgpu::Features::PUSH_CONSTANTS
        | wgpu::Features::UNSIZED_BINDING_ARRAY
        | wgpu::Features::SAMPLED_TEXTURE_BINDING_ARRAY
        | wgpu::Features::SAMPLED_TEXTURE_ARRAY_DYNAMIC_INDEXING
        | wgpu::Features::SAMPLED_TEXTURE_ARRAY_NON_UNIFORM_INDEXING;

    let mut limits = wgpu::Limits::default();
    limits.max_push_constant_size = std::mem::size_of::<PushConstants>() as u32;

    let mut upper = false;
    let mut time = std::time::Instant::now();
    wgpu_engine::quick_run(
        1,
        features,
        limits,
        |_id, _tokio_runtime, update_context| {
            let mut screen_task = ScreenTask::new(update_context);
            //screen_task.create_surface(0, String::from("surface"), SurfaceSource::File{path: std::path::PathBuf::from("./gfx_logo.png")}, [0,0,0], [100,100]);
            //screen_task.create_surface(1, String::from("surface"), SurfaceSource::File{path: std::path::PathBuf::from("./gfx_logo.png")}, [50,50,1], [100,100]);
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
                [0, 0, 1],
                [200, 200],
            );
            screen_task
        },
        |screen_task| {
            if time.elapsed().as_millis() > 3000 {
                time = std::time::Instant::now();
                upper = !upper;
                println!("Swapping surfaces!");
                if upper {
                    screen_task.move_surface(0, [0, 0, 0]);
                    screen_task.move_surface(1, [0, 0, 1]);
                } else {
                    screen_task.move_surface(1, [0, 0, 0]);
                    screen_task.move_surface(0, [0, 0, 1]);
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
        },
    );
}

#[test]
fn destroy_surface_test() {
    env_logger::init();

    let features = wgpu::Features::EXTERNAL_MEMORY
        | wgpu::Features::PUSH_CONSTANTS
        | wgpu::Features::UNSIZED_BINDING_ARRAY
        | wgpu::Features::SAMPLED_TEXTURE_BINDING_ARRAY
        | wgpu::Features::SAMPLED_TEXTURE_ARRAY_DYNAMIC_INDEXING
        | wgpu::Features::SAMPLED_TEXTURE_ARRAY_NON_UNIFORM_INDEXING;

    let mut limits = wgpu::Limits::default();
    limits.max_push_constant_size = std::mem::size_of::<PushConstants>() as u32;

    let mut removed = false;
    let time = std::time::Instant::now();
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
            screen_task
        },
        |screen_task| {
            if time.elapsed().as_millis() > 3000 && !removed {
                screen_task.remove_surface(0);
                removed = true;
                println!("Destroying surface!");
            }
            std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
        },
    );
}

#[test]
fn multioutput_test() {
    env_logger::init();

    let features = wgpu::Features::EXTERNAL_MEMORY
        | wgpu::Features::PUSH_CONSTANTS
        | wgpu::Features::UNSIZED_BINDING_ARRAY
        | wgpu::Features::SAMPLED_TEXTURE_BINDING_ARRAY
        | wgpu::Features::SAMPLED_TEXTURE_ARRAY_DYNAMIC_INDEXING
        | wgpu::Features::SAMPLED_TEXTURE_ARRAY_NON_UNIFORM_INDEXING;

    let mut limits = wgpu::Limits::default();
    limits.max_push_constant_size = std::mem::size_of::<PushConstants>() as u32;

    let mut upper = false;
    let mut time = std::time::Instant::now();
    wgpu_engine::quick_run(
        2,
        features,
        limits,
        |_id, _tokio_runtime, update_context| {
            let mut screen_task = ScreenTask::new(update_context);
            //screen_task.create_surface(0, String::from("surface"), SurfaceSource::File{path: std::path::PathBuf::from("./gfx_logo.png")}, [0,0,0], [100,100]);
            //screen_task.create_surface(1, String::from("surface"), SurfaceSource::File{path: std::path::PathBuf::from("./gfx_logo.png")}, [50,50,1], [100,100]);
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
                [0, 0, 1],
                [200, 200],
            );
            screen_task
        },
        |screen_task| {
            if time.elapsed().as_millis() > 3000 {
                time = std::time::Instant::now();
                upper = !upper;
                println!("Swapping surfaces!");
                if upper {
                    screen_task.move_surface(0, [0, 0, 0]);
                    screen_task.move_surface(1, [0, 0, 1]);
                } else {
                    screen_task.move_surface(1, [0, 0, 0]);
                    screen_task.move_surface(0, [0, 0, 1]);
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
        },
    );
}

#[test]
fn projection_matrix_test() {
    use ultraviolet::{Mat4, Vec4};
    let surface_position = Vec4::new(100.0,100.0,0.0,1.0);
    let push_constants = PushConstants::new([800,800],1024);
    println!("{:#?}",push_constants.projection_matrix * surface_position);
}
