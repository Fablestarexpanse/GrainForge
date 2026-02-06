// ═══════════════════════════════════════════════════════════════════════════
// GRAINFORGE NOISE LIBRARY
// GPU-optimized noise functions for procedural grain generation
// ═══════════════════════════════════════════════════════════════════════════

// PCG Random Number Generator (high quality, GPU-optimized)
fn pcg(v: u32) -> u32 {
    var state = v * 747796405u + 2891336453u;
    var word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}

fn pcg2d(v: vec2<u32>) -> vec2<u32> {
    var p = v * vec2(1664525u, 1013904223u);
    p.x += p.y * 1664525u;
    p.y += p.x * 1664525u;
    p ^= p >> vec2(16u);
    p.x += p.y * 1664525u;
    p.y += p.x * 1664525u;
    p ^= p >> vec2(16u);
    return p;
}

// Convert uint to float in [0, 1)
fn uint_to_float(x: u32) -> f32 {
    return f32(x) * (1.0 / 4294967296.0);
}

// Box-Muller transform: uniform -> gaussian
fn box_muller(u1: f32, u2: f32) -> vec2<f32> {
    let r = sqrt(-2.0 * log(max(u1, 1e-10)));
    let theta = 6.283185307 * u2;
    return vec2(r * cos(theta), r * sin(theta));
}

// ─────────────────────────────────────────────────────────────────────────────
// VALUE NOISE
// ─────────────────────────────────────────────────────────────────────────────

fn hash21(p: vec2<f32>) -> f32 {
    let h = pcg(bitcast<u32>(p.x) ^ (bitcast<u32>(p.y) * 747796405u));
    return uint_to_float(h);
}

fn value_noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    
    // Quintic interpolation (smoother than cubic)
    let u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
    
    let a = hash21(i);
    let b = hash21(i + vec2(1.0, 0.0));
    let c = hash21(i + vec2(0.0, 1.0));
    let d = hash21(i + vec2(1.0, 1.0));
    
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

// ─────────────────────────────────────────────────────────────────────────────
// SIMPLEX NOISE (2D)
// ─────────────────────────────────────────────────────────────────────────────

fn simplex_noise(p: vec2<f32>) -> f32 {
    const K1: f32 = 0.366025404;  // (sqrt(3)-1)/2
    const K2: f32 = 0.211324865;  // (3-sqrt(3))/6
    
    let i = floor(p + (p.x + p.y) * K1);
    let a = p - i + (i.x + i.y) * K2;
    
    let o = select(vec2(0.0, 1.0), vec2(1.0, 0.0), a.x > a.y);
    
    let b = a - o + K2;
    let c = a - 1.0 + 2.0 * K2;
    
    var h = max(0.5 - vec3(dot(a, a), dot(b, b), dot(c, c)), vec3(0.0));
    h = h * h * h * h;
    
    let n = vec3(
        dot(a, hash22(i) - 0.5),
        dot(b, hash22(i + o) - 0.5),
        dot(c, hash22(i + 1.0) - 0.5)
    );
    
    return dot(n, h) * 70.0;
}

fn hash22(p: vec2<f32>) -> vec2<f32> {
    let h = pcg2d(vec2<u32>(bitcast<u32>(p.x), bitcast<u32>(p.y)));
    return vec2(uint_to_float(h.x), uint_to_float(h.y));
}

// ─────────────────────────────────────────────────────────────────────────────
// VORONOI / WORLEY NOISE
// ─────────────────────────────────────────────────────────────────────────────

struct VoronoiResult {
    distance: f32,      // Distance to nearest cell
    cell_id: vec2<f32>, // ID of nearest cell (for coloring)
}

fn voronoi(p: vec2<f32>, jitter: f32) -> VoronoiResult {
    let n = floor(p);
    let f = fract(p);
    
    var min_dist = 8.0;
    var min_cell = vec2(0.0);
    
    for (var j = -1; j <= 1; j++) {
        for (var i = -1; i <= 1; i++) {
            let g = vec2(f32(i), f32(j));
            let o = hash22(n + g) * jitter;
            let r = g + o - f;
            let d = dot(r, r);
            
            if (d < min_dist) {
                min_dist = d;
                min_cell = n + g;
            }
        }
    }
    
    return VoronoiResult(sqrt(min_dist), min_cell);
}

// ─────────────────────────────────────────────────────────────────────────────
// FRACTAL BROWNIAN MOTION
// ─────────────────────────────────────────────────────────────────────────────

fn fbm(
    p: vec2<f32>,
    octaves: i32,
    lacunarity: f32,
    persistence: f32,
) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var frequency = 1.0;
    var pos = p;
    
    for (var i = 0; i < octaves; i++) {
        value += amplitude * simplex_noise(pos * frequency);
        frequency *= lacunarity;
        amplitude *= persistence;
    }
    
    return value;
}

// ─────────────────────────────────────────────────────────────────────────────
// DOMAIN WARPING
// ─────────────────────────────────────────────────────────────────────────────

fn domain_warp(
    p: vec2<f32>,
    strength: f32,
    octaves: i32,
) -> f32 {
    let q = vec2(
        fbm(p + vec2(0.0, 0.0), octaves, 2.0, 0.5),
        fbm(p + vec2(5.2, 1.3), octaves, 2.0, 0.5)
    );
    
    let r = vec2(
        fbm(p + strength * q + vec2(1.7, 9.2), octaves, 2.0, 0.5),
        fbm(p + strength * q + vec2(8.3, 2.8), octaves, 2.0, 0.5)
    );
    
    return fbm(p + strength * r, octaves, 2.0, 0.5);
}
