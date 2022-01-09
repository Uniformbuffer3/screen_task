use crate::display::Display;
use crate::surface::Surface;
use crate::ScreenTask;
use crate::SurfaceManager;
use wgpu_engine::*;

impl ScreenTask {
    /// Generate the render pipeline descriptor.
    pub(crate) fn prepare_render_pipeline(
        update_context: &mut UpdateContext,
        device: DeviceId,
        display: &Display,
        layout: PipelineLayoutId,
        vertex_shader: ShaderModuleId,
        fragment_shader: ShaderModuleId,
        surface_manager: &SurfaceManager,
    ) -> (RenderPipelineDescriptor, bool) {
        log::info!(target: "ScreenTask","Preparing render pipeline descriptor");
        let format = update_context
            .swapchain_descriptor_ref(display.swapchain())
            .unwrap()
            .format;
        let ready = surface_manager.len() > 0;

        let descriptor = RenderPipelineDescriptor {
            device,
            label: Self::TASK_NAME.to_string() + " render pipeline",
            layout: Some(layout),
            vertex: VertexState {
                module: vertex_shader,
                entry_point: String::from("main"),
                buffers: vec![VertexBufferLayout {
                    array_stride: std::mem::size_of::<Surface>() as wgpu::BufferAddress,
                    step_mode: wgpu::InputStepMode::Instance,
                    attributes: wgpu::vertex_attr_array![
                        0 => Float32x3,
                        1 => Float32x2,
                        2 => Uint32,
                    ]
                    .to_vec(),
                }],
            },
            primitive: wgpu::PrimitiveState {
                //front_face: wgpu::FrontFace::Ccw,
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..Default::default()
            },
            multisample: wgpu::MultisampleState::default(),
            depth_stencil: Some(DepthStencilState {
                id: *display.depth_stencil_view(),
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            fragment: if ready {
                Some(FragmentState {
                    module: fragment_shader,
                    entry_point: String::from("main"),
                    targets: vec![wgpu::ColorTargetState {
                        format,
                        blend: None,
                        write_mask: wgpu::ColorWrite::ALL,
                    }],
                })
            } else {
                None
            },
        };
        (descriptor, ready)
    }
}
