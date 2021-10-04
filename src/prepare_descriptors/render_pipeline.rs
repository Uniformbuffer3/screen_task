use crate::display::Display;
use crate::surface::Surface;
use crate::ScreenTask;
use wgpu_engine::*;

impl ScreenTask {
    pub(crate) fn prepare_render_pipeline(
        update_context: &mut UpdateContext,
        device: DeviceId,
        display: &Display,
        layout: PipelineLayoutId,
        vertex_shader: ShaderModuleId,
        fragment_shader: ShaderModuleId,
    ) -> RenderPipelineDescriptor {
        let format = update_context
            .swapchain_descriptor_ref(display.swapchain())
            .unwrap()
            .format;

        RenderPipelineDescriptor {
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
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            fragment: Some(FragmentState {
                module: fragment_shader,
                entry_point: String::from("main"),
                targets: vec![wgpu::ColorTargetState {
                    format,
                    blend: None,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
        }
    }
}
