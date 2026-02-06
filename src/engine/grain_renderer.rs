use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor,
    ComputePassDescriptor, Device, Extent3d, Queue, ShaderStages, StorageTextureAccess, Texture,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor,
};
use bytemuck::{Pod, Zeroable};
use std::sync::Arc;

use crate::engine::compute_pipeline::GrainComputePipeline;
use crate::core::error::GrainError;

/// GPU parameters passed to the compute shader
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct GrainParams {
    pub grain_amount: f32,
    pub grain_size: f32,
    pub width: f32,
    pub height: f32,
    pub seed: f32,
    pub _padding: [f32; 3], // Align to 32 bytes
}

impl Default for GrainParams {
    fn default() -> Self {
        Self {
            grain_amount: 0.5,
            grain_size: 1.0,
            width: 512.0,
            height: 512.0,
            seed: 0.0,
            _padding: [0.0; 3],
        }
    }
}

/// Renders grain texture using the compute pipeline
pub struct GrainRenderer {
    pipeline: GrainComputePipeline,
    output_texture: Texture,
    bind_group_layout: BindGroupLayout,
    bind_group: BindGroup,
    params_buffer: Buffer,
    width: u32,
    height: u32,
}

impl GrainRenderer {
    pub fn new(device: &Device, width: u32, height: u32) -> Result<Self, GrainError> {
        let pipeline = GrainComputePipeline::new(device)?;

        // Create output texture
        let output_texture = device.create_texture(&TextureDescriptor {
            label: Some("Grain Output Texture"),
            size: Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        // Create params buffer
        let params_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Grain Params Buffer"),
            size: std::mem::size_of::<GrainParams>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Grain Bind Group Layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::WriteOnly,
                        format: TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let texture_view = output_texture.create_view(&TextureViewDescriptor::default());

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Grain Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: params_buffer.as_entire_binding(),
                },
            ],
        });

        Ok(Self {
            pipeline,
            output_texture,
            bind_group_layout,
            bind_group,
            params_buffer,
            width,
            height,
        })
    }

    /// Render grain with the given parameters
    pub fn render(&self, device: &Device, queue: &Queue, params: &GrainParams) {
        // Update params buffer
        queue.write_buffer(&self.params_buffer, 0, bytemuck::cast_slice(&[*params]));

        // Create command encoder
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Grain Compute Encoder"),
        });

        {
            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("Grain Compute Pass"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.pipeline.pipeline);
            compute_pass.set_bind_group(0, &self.bind_group, &[]);

            // Dispatch: workgroup size is 8x8, so we need ceil(width/8) x ceil(height/8)
            let workgroups_x = (self.width + 7) / 8;
            let workgroups_y = (self.height + 7) / 8;
            compute_pass.dispatch_workgroups(workgroups_x, workgroups_y, 1);
        }

        queue.submit(std::iter::once(encoder.finish()));
    }

    /// Get the output texture for display
    pub fn output_texture(&self) -> &Texture {
        &self.output_texture
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
