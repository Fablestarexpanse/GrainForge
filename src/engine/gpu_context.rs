use wgpu::{Device, Queue, TextureFormat};
use std::sync::Arc;
use crate::core::error::GrainError;

/// Wrapper around WGPU state shared from eframe
pub struct GpuContext {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub format: TextureFormat,
}

impl GpuContext {
    pub fn new(cc: &eframe::CreationContext) -> Result<Self, GrainError> {
        let wgpu_state = cc.wgpu_render_state.as_ref()
            .ok_or_else(|| GrainError::GpuInit("WGPU state not available in eframe".to_string()))?;
            
        Ok(Self {
            device: Arc::new(wgpu_state.device.clone()),
            queue: Arc::new(wgpu_state.queue.clone()),
            format: wgpu_state.target_format,
        })
    }
}
