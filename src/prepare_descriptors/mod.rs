use wgpu_engine::*;

mod bind_group;
mod bind_group_layout;
mod command_buffer;
mod pipeline_layout;
mod render_pipeline;

use crate::DeviceResources;
use crate::Display;
use crate::DisplayResources;
use crate::PushConstants;
use crate::ScreenTask;

use crate::shaders::*;
use crate::surface_manager::SurfaceManager;

impl ScreenTask {
    pub(crate) fn init_device_resources(
        update_context: &mut UpdateContext,
        device: DeviceId,
        swapchain: SwapchainId,
        display_position: [u32; 2],
    ) -> DeviceResources {
        let surface_manager = SurfaceManager::new(update_context, device);

        let vertex_shader_descriptor = ShaderModuleDescriptor {
            device,
            label: String::from("ScreenTask VS"),
            source: ShaderSource::SpirV(VERTEX_SHADER_CODE.to_vec()),
            flags: wgpu::ShaderFlags::empty(),
        };
        let vertex_shader = update_context
            .add_shader_module_descriptor(vertex_shader_descriptor)
            .unwrap();

        let fragment_shader_descriptor = ShaderModuleDescriptor {
            device,
            label: String::from("ScreenTask FS"),
            source: ShaderSource::SpirV(FRAGMENT_SHADER_CODE.to_vec()),
            flags: wgpu::ShaderFlags::empty(),
        };
        let fragment_shader = update_context
            .add_shader_module_descriptor(fragment_shader_descriptor)
            .unwrap();

        let sampler_descriptor = SamplerDescriptor {
            device,
            label: Self::TASK_NAME.to_string() + " sampler",
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            min_filter: wgpu::FilterMode::Linear,
            mag_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: f32::MAX,
            compare: None,
            anisotropy_clamp: None,
            border_color: None,
        };

        let sampler = update_context
            .add_sampler_descriptor(sampler_descriptor)
            .unwrap();

        let bind_group_layout_descriptor =
            Self::prepare_bind_group_layout(update_context, device, &surface_manager);
        let bind_group_layout = update_context
            .add_bind_group_layout_descriptor(bind_group_layout_descriptor)
            .unwrap();

        let bind_group_descriptor = Self::prepare_bind_group(
            update_context,
            device,
            &surface_manager,
            bind_group_layout,
            sampler,
        );
        let bind_group = update_context
            .add_bind_group_descriptor(bind_group_descriptor)
            .unwrap();

        let pipeline_layout_descriptor =
            Self::prepare_pipeline_layout(update_context, device, bind_group_layout);
        let pipeline_layout = update_context
            .add_pipeline_layout_descriptor(pipeline_layout_descriptor)
            .unwrap();

        let display = Display::new(update_context, device, swapchain, display_position);
        let render_pipeline_descriptor = Self::prepare_render_pipeline(
            update_context,
            device,
            &display,
            pipeline_layout,
            vertex_shader,
            fragment_shader,
        );
        let render_pipeline = update_context
            .add_render_pipeline_descriptor(render_pipeline_descriptor)
            .unwrap();

        let displays = vec![DisplayResources {
            display,
            render_pipeline,
        }];

        let command_buffer_descriptor = Self::prepare_command_buffer(
            update_context,
            device,
            &displays,
            bind_group,
            &surface_manager,
        );
        let command_buffer = update_context
            .add_command_buffer_descriptor(command_buffer_descriptor)
            .unwrap();

        let data_copy_command_buffer_descriptor = CommandBufferDescriptor {
            device,
            label: Self::TASK_NAME.to_string() + " data copy command buffer",
            commands: vec![],
        };
        let data_copy_command_buffer = update_context
            .add_command_buffer_descriptor(data_copy_command_buffer_descriptor)
            .unwrap();

        let data_copy_command_buffer_updated = false;

        DeviceResources {
            displays,

            surface_manager,

            fragment_shader,
            vertex_shader,
            sampler,

            bind_group_layout,
            bind_group,

            pipeline_layout,

            command_buffer,

            data_copy_command_buffer,
            data_copy_command_buffer_updated,
        }
    }
}
