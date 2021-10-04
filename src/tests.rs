use wgpu_engine::*;
use crate::ScreenTask;
use crate::PushConstants;

#[test]
fn screen_task() {
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

/*
#[test]
fn rectangle_task() {
    use std::collections::HashMap;
    env_logger::init();
    use crate::WGpuEngine;
    use pal::definitions::*;




    let mut wgpu_engine = WGpuEngine::new(features, limits.clone());

    let mut platform = pal::Platform::new(vec![Box::new(wgpu_engine.wgpu_context())]);
    platform.request(vec![Request::from(SurfaceRequest::Create(None))]);

    let surfaces: HashMap<SurfaceId, (EntityId, [u32; 2])> = platform
        .events()
        .into_iter()
        .filter_map(|event| match event {
            pal::Event::Surface(ref surface_event) => {
                let surface_id = surface_event.id;
                match &surface_event.event_type {
                    pal::SurfaceEventType::Added(surface_info) => {
                        if let Surface::WGpu(surface) = &surface_info.surface {
                            let resource_id = wgpu_engine.create_surface(
                                String::from("MainSurface"),
                                surface.clone(),
                                surface_info.size.width,
                                surface_info.size.height,
                            );

                            match resource_id {
                                Ok(resource_id) => Some((
                                    surface_id,
                                    (
                                        resource_id,
                                        [surface_info.size.width, surface_info.size.height],
                                    ),
                                )),
                                Err(_) => None,
                            }
                        } else {
                            panic!("It is not of WGpu type");
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        })
        .collect();

    let target_surface = *surfaces.values().next().unwrap();

    let mut task = wgpu_engine
        .create_task(
            "ScreenTask".into(),
            features,
            limits,
            move |context, resources| {
                ScreenTask::new(context, resources, target_surface.0, target_surface.1)
            },
        )
        .unwrap();

    task.create_surface(
        String::from("Surface"),
        SurfaceSource::File {
            path: PathBuf::from("/home/fabio/wgpu_engine/src/logo.png"),
        },
        [150, 150, 0],
        [100, 100],
    );
    //task.create_surface_from_file(String::from("/home/fabio/wgpu_engine/src/logo.png"));

    let mut tasks = vec![task];
    'main_loop: loop {
        for event in platform.events() {
            match event {
                pal::Event::Surface(ref surface_event) => {
                    let surface_id = surface_event.id;
                    let resource_id = match surfaces.get(&surface_id) {
                        Some(resource_id) => resource_id.0,
                        None => continue,
                    };
                    match &surface_event.event_type {
                        pal::SurfaceEventType::Added(surface_info) => {
                            if let Surface::WGpu(surface) = &surface_info.surface {
                                wgpu_engine
                                    .create_surface(
                                        String::from("MainSurface"),
                                        surface.clone(),
                                        surface_info.size.width,
                                        surface_info.size.height,
                                    )
                                    .unwrap();
                            } else {
                                panic!("It is not of WGpu type");
                            }
                        }
                        pal::SurfaceEventType::Resized(size) => {
                            wgpu_engine
                                .resize_surface(&resource_id, size.width, size.height)
                                .unwrap();
                        }
                        pal::SurfaceEventType::Removed => {
                            wgpu_engine.remove_surface(resource_id);
                            if wgpu_engine.surface_count() == 0 {
                                break 'main_loop;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        wgpu_engine.dispatch_tasks(&mut tasks);
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}*/
