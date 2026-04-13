#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct PauseBlur {
    intensity: f32,
}
@group(0) @binding(2) var<uniform> settings: PauseBlur;

// 5-tap 1D Gaussian weights (sum ≈ 1.0)
const OFFSETS: array<f32, 5> = array<f32, 5>(-2.0, -1.0, 0.0, 1.0, 2.0);
const WEIGHTS: array<f32, 5> = array<f32, 5>(0.06136, 0.24477, 0.38774, 0.24477, 0.06136);

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let texel_size = 1.0 / vec2<f32>(textureDimensions(screen_texture));
    var color = vec4<f32>(0.0);

    for (var y: i32 = 0; y < 5; y++) {
        for (var x: i32 = 0; x < 5; x++) {
            let offset = vec2<f32>(OFFSETS[x], OFFSETS[y]) * texel_size * settings.intensity;
            let weight = WEIGHTS[x] * WEIGHTS[y];
            color += textureSample(screen_texture, texture_sampler, in.uv + offset) * weight;
        }
    }

    return color;
}
