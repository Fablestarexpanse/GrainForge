use wgpu::{ComputePipeline, Device, ShaderModuleDescriptor, ShaderSource};
use crate::core::error::GrainError;

pub struct GrainComputePipeline {
    pub pipeline: ComputePipeline,
}

impl GrainComputePipeline {
    pub fn new(device: &Device) -> Result<Self, GrainError> {
        // Load shaders
        // Concatenate noise library with grain shader
        let noise_src = include_str!("shaders/noise.wgsl");
        let grain_src = include_str!("shaders/grain.wgsl");
        let combined_src = format!("{}\n{}", noise_src, grain_src);

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Grain Compute Shader"),
            source: ShaderSource::Wgsl(combined_src.into()),
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Grain Pipeline"),
            layout: None, // Auto-infer layout
            module: &shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });

        Ok(Self { pipeline })
    }
}
