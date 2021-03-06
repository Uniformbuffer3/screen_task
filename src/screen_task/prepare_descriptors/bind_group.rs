use crate::surface_manager::SurfaceManager;
use crate::ScreenTask;
use wgpu_engine::*;

impl ScreenTask {
    /// Generate the bind group descriptor.
    pub(crate) fn prepare_bind_group(
        _update_context: &mut UpdateContext,
        device: DeviceId,
        surface_manager: &SurfaceManager,
        layout: BindGroupLayoutId,
        sampler: SamplerId,
    ) -> BindGroupDescriptor {
        let views = surface_manager.rectangle_views();
        log::info!(target: "ScreenTask","Preparing bind group descriptor with {} images",surface_manager.len());
        let mut entries = Vec::new();
        entries.push(BindGroupEntry {
            binding: 0,
            resource: BindingResource::Sampler(sampler),
        });

        if views.len() > 0 {
            entries.push(BindGroupEntry {
                binding: 1,
                resource: BindingResource::TextureViewArray(views),
            });
        }

        BindGroupDescriptor {
            device,
            label: Self::TASK_NAME.to_string() + " bind group",
            entries,
            layout,
        }
    }
}
