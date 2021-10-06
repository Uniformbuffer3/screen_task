use bytemuck::{Pod, Zeroable};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;
use ultraviolet::{Mat4, Vec4};
use wgpu_engine::*;

pub use crate::surface_manager::SurfaceManager;
pub use crate::display::Display;
pub use crate::surface::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct PushConstants {
    projection_matrix: Mat4,
}
impl PushConstants {
    pub fn new(target_surface_size: [u32; 2], max_surface_count: u32) -> Self {
        let projection_matrix = Mat4::new(
            Vec4::new(2.0 / target_surface_size[0] as f32, 0.0, 0.0, 0.0),
            Vec4::new(0.0, -2.0 / target_surface_size[1] as f32, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0 / max_surface_count as f32, 0.0),
            Vec4::new(-1.0, 1.0, 0.0, 0.0),
        );
        Self { projection_matrix }
    }
}

pub const DEPTH_STENCIL_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;

pub enum TaskEvent {
    CreateSurface {
        id: usize,
        label: String,
        source: Arc<SurfaceSource>,
        position: [u32; 3],
        size: [u32; 2],
    },
}

pub struct DisplayResources {
    pub display: Display,
    pub render_pipeline: RenderPipelineId,
}

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

pub struct ScreenTask {
    pending_events: Vec<TaskEvent>,
    devices: HashMap<DeviceId, DeviceResources>,
}

impl ScreenTask {
    pub const TASK_NAME: &'static str = "ScreenTask";

    pub fn new(update_context: &mut UpdateContext) -> Self {
        let pending_events = Vec::new();
        let task_name = Self::TASK_NAME.to_string();
        let devices = HashMap::new();

        Self {
            pending_events,
            devices,
        }
    }

    pub fn create_surface(
        &mut self,
        external_id: usize,
        label: String,
        source: SurfaceSource,
        position: [u32; 3],
        size: [u32; 2],
    ) -> usize {
        self.pending_events.push(TaskEvent::CreateSurface {
            id: external_id,
            label,
            source: Arc::new(source),
            position,
            size,
        });
        external_id
    }

    pub fn resize_surface(&mut self) {
        unimplemented!()
    }

    fn update_resources(
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

        let bind_group_descriptor = Self::prepare_bind_group(
            update_context,
            device,
            &device_resources.surface_manager,
            device_resources.bind_group_layout,
            device_resources.sampler,
        );
        update_context
            .update_bind_group_descriptor(&mut device_resources.bind_group, bind_group_descriptor);

        for display_resources in &mut device_resources.displays {
            let render_pipeline_descriptor = Self::prepare_render_pipeline(
                update_context,
                device,
                &display_resources.display,
                device_resources.pipeline_layout,
                device_resources.vertex_shader,
                device_resources.fragment_shader,
            );
            update_context.update_render_pipeline_descriptor(
                &mut display_resources.render_pipeline,
                render_pipeline_descriptor,
            );
        }

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

    fn elaborate_events(&mut self, update_context: &mut UpdateContext) {
        let mut update = false;
        for event in self.pending_events.drain(..) {
            match event {
                TaskEvent::CreateSurface {
                    id,
                    label,
                    source,
                    position,
                    size,
                } => {
                    self.devices.values_mut().for_each(|device_resources| {
                        device_resources.surface_manager.create_surface(
                            update_context,
                            label.clone(),
                            id,
                            source.clone(),
                            position,
                            size,
                        );
                    });

                    update = true;
                }
            }
        }

        if update {
            self.devices
                .iter_mut()
                .for_each(|(device, device_resources)| {
                    Self::update_resources(update_context, *device, device_resources);

                    let commands = device_resources.surface_manager.update(update_context);
                    if !commands.is_empty() {

                        let data_copy_command_buffer_descriptor = CommandBufferDescriptor {
                            device: *device,
                            label: Self::TASK_NAME.to_string() + " data copy command buffer",
                            commands,
                        };
                        update_context.update_command_buffer_descriptor(
                            &mut device_resources.data_copy_command_buffer,
                            data_copy_command_buffer_descriptor,
                        );
                        device_resources.data_copy_command_buffer_updated = true;

                    }
                })
        }
    }

    pub fn features_and_limits() -> (wgpu::Features, wgpu::Limits) {
        let mut features =
            wgpu::Features::PUSH_CONSTANTS
            | wgpu::Features::UNSIZED_BINDING_ARRAY
            | wgpu::Features::SAMPLED_TEXTURE_BINDING_ARRAY
            | wgpu::Features::SAMPLED_TEXTURE_ARRAY_DYNAMIC_INDEXING
            | wgpu::Features::SAMPLED_TEXTURE_ARRAY_NON_UNIFORM_INDEXING;

        #[cfg(feature="wgpu_custom_backend")]
        {
            features |= wgpu::Features::EXTERNAL_MEMORY;
        }


        let mut limits = wgpu::Limits::default();
        limits.max_push_constant_size = std::mem::size_of::<PushConstants>() as u32;

        (features, limits)
    }
}

impl TaskTrait for ScreenTask {
    fn name(&self) -> String {
        Self::TASK_NAME.to_string()
    }

    fn update_resources(&mut self, update_context: &mut UpdateContext) {
        let events = update_context.events().clone();
            events.iter()
            .for_each(|event| match event {
                ResourceEvent::SwapchainCreated(swapchain) => {
                    let device = update_context.entity_device_id(swapchain).unwrap();
                    match self.devices.entry(device) {
                        Entry::Vacant(vacant) => {
                            let resources = Self::init_device_resources(
                                update_context,
                                device,
                                *swapchain,
                                [0, 0],
                            );
                            vacant.insert(resources);
                        }
                        Entry::Occupied(mut occupied) => {
                            let device_resources = occupied.get_mut();
                            let display = Display::new(update_context, device, *swapchain, [0, 0]);
                            let render_pipeline_descriptor = Self::prepare_render_pipeline(
                                update_context,
                                device,
                                &display,
                                device_resources.pipeline_layout,
                                device_resources.vertex_shader,
                                device_resources.fragment_shader,
                            );
                            let render_pipeline = update_context
                                .add_render_pipeline_descriptor(render_pipeline_descriptor)
                                .unwrap();

                            device_resources.displays.push(DisplayResources {
                                display,
                                render_pipeline,
                            });

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
                }
                ResourceEvent::SwapchainDestroyed(swapchain) => {
                    self.devices.retain(|device, device_resources| {
                        if let Some(index) =
                            device_resources
                                .displays
                                .iter()
                                .position(|display_resources| {
                                    display_resources.display.swapchain() == swapchain
                                })
                        {
                            device_resources.displays.remove(index);
                            if !device_resources.displays.is_empty() {
                                let command_buffer_descriptor = Self::prepare_command_buffer(
                                    update_context,
                                    *device,
                                    &device_resources.displays,
                                    device_resources.bind_group,
                                    &device_resources.surface_manager,
                                );
                                update_context.update_command_buffer_descriptor(
                                    &mut device_resources.command_buffer,
                                    command_buffer_descriptor,
                                );
                                true
                            } else {
                                false
                            }
                        } else {
                            true
                        }
                    });
                }
                _ => (),
            });

        self.elaborate_events(update_context);
    }
    fn command_buffers(&self) -> Vec<CommandBufferId> {
        self.devices
            .values()
            .map(|device_resources| {
                let mut cbs = Vec::new();
                if device_resources.surface_manager.len() > 0 {cbs.push(device_resources.command_buffer);}
                if device_resources.data_copy_command_buffer_updated {
                    cbs.push(device_resources.data_copy_command_buffer);
                }
                cbs
            })
            .flatten()
            .collect()
    }
}
