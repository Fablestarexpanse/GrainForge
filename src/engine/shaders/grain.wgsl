// Basic Grain Compute Shader
// Uses the Noise Library to generate film grain

// Note: We need to include noise.wgsl. In WGPU we typically concatenate or use imports if supported (not standard WGSL yet).
// For this implementation, we assume we append this file to noise.wgsl when creating the shader module, OR we just assume functions are available if we do that in Rust.
// The Rust side will handle concatenation.

@group(0) @binding(0) var output_texture: texture_storage_2d<rgba8unorm, write>;

struct Params {
    grain_amount: f32,
    grain_size: f32,
    width: f32,
    height: f32,
    seed: f32,
}

@group(0) @binding(1) var<uniform> params: Params;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dimensions = vec2<u32>(textureDimensions(output_texture));
    let coords = vec2<u32>(global_id.xy);

    if (coords.x >= dimensions.x || coords.y >= dimensions.y) {
        return;
    }

    let uv = vec2<f32>(coords) / vec2<f32>(dimensions);
    
    // Use simplex noise for grain
    let noise_scale = 100.0 / max(0.1, params.grain_size);
    let p = uv * noise_scale + vec2(params.seed * 10.0, 0.0);
    
    let noise_val = simplex_noise(p);
    
    // Map noise [-1, 1] to [0, 1] and apply intensity
    let grain = (noise_val * 0.5 + 0.5) * params.grain_amount;
    
    // Output grayscale for now
    let color = vec4<f32>(grain, grain, grain, 1.0);
    
    textureStore(output_texture, coords, color);
}
