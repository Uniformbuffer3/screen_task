use crate::surface_manager::SurfaceManager;
use std::num::NonZeroU32;
use wgpu_engine::*;

pub struct Display {
    device: DeviceId,
    swapchain: SwapchainId,

    depth_stencil: TextureId,
    depth_stencil_view: TextureViewId,

    position: [u32; 2],
    size: [u32; 2],
}
impl Display {
    pub fn new(
        update_context: &mut UpdateContext,
        device: DeviceId,
        swapchain: SwapchainId,
        position: [u32; 2],
    ) -> Self {
        let swapchain_descriptor = update_context.swapchain_descriptor_ref(&swapchain).unwrap();
        let size = [swapchain_descriptor.width, swapchain_descriptor.height];
        let texture_descriptor = TextureDescriptor {
            device,
            label: String::from("DepthStencil"),
            source: TextureSource::Local,
            size: wgpu_engine::Extent3d {
                width: swapchain_descriptor.width,
                height: swapchain_descriptor.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu_engine::TextureDimension::D2,
            format: crate::DEPTH_STENCIL_FORMAT,
            usage: wgpu_engine::TextureUsage::RENDER_ATTACHMENT,
        };
        let depth_stencil = update_context
            .add_texture_descriptor(texture_descriptor)
            .unwrap();

        let texture_view_descriptor = TextureViewDescriptor {
            device,
            label: String::from("DepthStencil view"),
            texture: depth_stencil,
            dimension: wgpu_engine::TextureViewDimension::D2,
            format: crate::DEPTH_STENCIL_FORMAT,
            aspect: wgpu_engine::TextureAspect::DepthOnly,
            base_mip_level: 0,
            mip_level_count: Some(NonZeroU32::new(1).unwrap()),
            base_array_layer: 0,
            array_layer_count: Some(NonZeroU32::new(1).unwrap()),
        };
        let depth_stencil_view = update_context
            .add_texture_view_descriptor(texture_view_descriptor)
            .unwrap();

        Self {
            device,
            swapchain,
            depth_stencil,
            depth_stencil_view,
            position,
            size,
        }
    }

    pub fn update(&mut self, update_context: &mut UpdateContext) {
        let swapchain_descriptor = update_context
            .swapchain_descriptor_ref(&self.swapchain)
            .unwrap();
        self.size = [swapchain_descriptor.width, swapchain_descriptor.height];
        let texture_descriptor = TextureDescriptor {
            device: self.device,
            label: String::from("DepthStencil"),
            source: TextureSource::Local,
            size: wgpu_engine::Extent3d {
                width: swapchain_descriptor.width,
                height: swapchain_descriptor.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu_engine::TextureDimension::D2,
            format: crate::DEPTH_STENCIL_FORMAT,
            usage: wgpu_engine::TextureUsage::RENDER_ATTACHMENT,
        };
        assert!(
            update_context.update_texture_descriptor(&mut self.depth_stencil, texture_descriptor)
        );

        let texture_view_descriptor = TextureViewDescriptor {
            device: self.device,
            label: String::from("DepthStencil"),
            texture: self.depth_stencil,
            dimension: wgpu_engine::TextureViewDimension::D2,
            format: crate::DEPTH_STENCIL_FORMAT,
            aspect: wgpu_engine::TextureAspect::DepthOnly,
            base_mip_level: 0,
            mip_level_count: Some(NonZeroU32::new(1).unwrap()),
            base_array_layer: 0,
            array_layer_count: Some(NonZeroU32::new(1).unwrap()),
        };
        assert!(update_context
            .update_texture_view_descriptor(&mut self.depth_stencil_view, texture_view_descriptor));
    }

    pub fn swapchain(&self) -> &SwapchainId {
        &self.swapchain
    }
    pub fn depth_stencil(&self) -> &TextureId {
        &self.depth_stencil
    }
    pub fn depth_stencil_view(&self) -> &TextureViewId {
        &self.depth_stencil_view
    }

    pub fn position(&self) -> [u32; 2] {
        self.position
    }
    pub fn size(&self) -> [u32; 2] {
        self.size
    }
}

pub struct DisplayResources {
    pub display: Display,
    pub render_pipeline_ready: bool,
    pub render_pipeline: RenderPipelineId,
}
impl DisplayResources {
    pub fn new(
        update_context: &mut UpdateContext,
        display: Display,
        pipeline_layout: PipelineLayoutId,
        vertex_shader: ShaderModuleId,
        fragment_shader: ShaderModuleId,
        surface_manager: &SurfaceManager,
    ) -> Self {
        let device = display.device;
        let (render_pipeline_descriptor, render_pipeline_ready) =
            crate::screen_task::ScreenTask::prepare_render_pipeline(
                update_context,
                device,
                &display,
                pipeline_layout,
                vertex_shader,
                fragment_shader,
                surface_manager,
            );
        let render_pipeline = update_context
            .add_render_pipeline_descriptor(render_pipeline_descriptor)
            .unwrap();

        DisplayResources {
            display,
            render_pipeline,
            render_pipeline_ready,
        }
    }
}
