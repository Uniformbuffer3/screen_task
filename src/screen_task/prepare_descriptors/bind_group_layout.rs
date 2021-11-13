use crate::surface_manager::SurfaceManager;
use crate::ScreenTask;
use std::num::NonZeroU32;
use wgpu_engine::*;

impl ScreenTask {
    pub(crate) fn prepare_bind_group_layout(
        _update_context: &mut UpdateContext,
        device: DeviceId,
        surface_manager: &SurfaceManager,
    ) -> BindGroupLayoutDescriptor {
        log::info!(target: "ScreenTask","Preparing bind group layout for {} images",surface_manager.len());

        let mut entries = Vec::new();
        entries.push(wgpu_engine::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu_engine::ShaderStage::FRAGMENT,
            ty: wgpu::BindingType::Sampler {
                comparison: true,
                filtering: true,
            },
            count: None,
        });

        if surface_manager.len() > 0 {
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: NonZeroU32::new(surface_manager.len() as u32),
            });
        }

        BindGroupLayoutDescriptor {
            device,
            label: Self::TASK_NAME.to_string() + " bind group layout",
            entries,
        }
    }
}
