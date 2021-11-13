use bytemuck::{Pod, Zeroable};
use std::collections::HashMap;
use ultraviolet::{Mat4, Vec4};
use wgpu_engine::*;

mod device_resources;
mod events;
mod prepare_descriptors;
mod task_impl;
mod update_descriptors;

pub use crate::display::{Display, DisplayResources};
pub use crate::screen_task::device_resources::DeviceResources;
pub use crate::screen_task::events::ScreenTaskEvent;
pub use crate::surface::*;
pub use crate::surface_manager::SurfaceManager;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct PushConstants {
    pub projection_matrix: Mat4,
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

pub const DEPTH_STENCIL_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub struct ScreenTask {
    pending_events: Vec<ScreenTaskEvent>,
    devices: HashMap<DeviceId, DeviceResources>,
}

impl ScreenTask {
    pub const TASK_NAME: &'static str = "ScreenTask";

    pub fn new(_update_context: &mut UpdateContext) -> Self {
        let pending_events = Vec::new();
        let _task_name = Self::TASK_NAME.to_string();
        let devices = HashMap::new();

        Self {
            pending_events,
            devices,
        }
    }

    pub fn create_surface(
        &mut self,
        external_id: usize,
        label: impl Into<String>,
        source: SurfaceSource,
        position: [i32; 3],
        size: [u32; 2],
    ) {
        let label = label.into();
        self.pending_events.push(ScreenTaskEvent::CreateSurface {
            id: external_id,
            label,
            source,
            position,
            size,
        });
    }

    pub fn update_source(&mut self, external_id: usize, source: SurfaceSource) {
        self.pending_events.push(ScreenTaskEvent::UpdateSource {
            id: external_id,
            source,
        });
    }

    pub fn update_data(&mut self, external_id: usize, data: Vec<u8>) {
        self.pending_events.push(ScreenTaskEvent::UpdateData {
            id: external_id,
            data,
        });
    }

    pub fn resize_surface(&mut self, external_id: usize, size: [u32; 2]) {
        self.pending_events.push(ScreenTaskEvent::ResizeSurface {
            id: external_id,
            size,
        });
    }

    pub fn move_surface(&mut self, external_id: usize, position: [i32; 3]) {
        self.pending_events.push(ScreenTaskEvent::MoveSurface {
            id: external_id,
            position,
        });
    }

    pub fn remove_surface(&mut self, external_id: usize) {
        self.pending_events
            .push(ScreenTaskEvent::RemoveSurface { id: external_id });
    }

    pub fn features_and_limits() -> (wgpu::Features, wgpu::Limits) {
        let mut features = wgpu::Features::PUSH_CONSTANTS
            | wgpu::Features::UNSIZED_BINDING_ARRAY
            | wgpu::Features::SAMPLED_TEXTURE_BINDING_ARRAY
            | wgpu::Features::SAMPLED_TEXTURE_ARRAY_DYNAMIC_INDEXING
            | wgpu::Features::SAMPLED_TEXTURE_ARRAY_NON_UNIFORM_INDEXING;

        #[cfg(feature = "wgpu_custom_backend")]
        {
            features |= wgpu::Features::EXTERNAL_MEMORY;
        }

        let mut limits = wgpu::Limits::default();
        limits.max_push_constant_size = std::mem::size_of::<PushConstants>() as u32;

        (features, limits)
    }
}
