#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
    forward_io::{VertexOutput, FragmentOutput},
}

// x = noise_scale, yzw = unused padding (Vec4 = 16 bytes, always aligned)
// MATERIAL_BIND_GROUP is injected by Bevy — must not hardcode the group number
@group(#{MATERIAL_BIND_GROUP}) @binding(100) var<uniform> rock: vec4<f32>;

fn hash3(p: vec3<f32>) -> f32 {
    var q = fract(p * vec3<f32>(0.1031, 0.1030, 0.0973));
    q = q + dot(q, q.yxz + 19.19);
    return fract((q.x + q.y) * q.z);
}

fn value_noise(p: vec3<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(
        mix(
            mix(hash3(i + vec3(0., 0., 0.)), hash3(i + vec3(1., 0., 0.)), u.x),
            mix(hash3(i + vec3(0., 1., 0.)), hash3(i + vec3(1., 1., 0.)), u.x),
            u.y
        ),
        mix(
            mix(hash3(i + vec3(0., 0., 1.)), hash3(i + vec3(1., 0., 1.)), u.x),
            mix(hash3(i + vec3(0., 1., 1.)), hash3(i + vec3(1., 1., 1.)), u.x),
            u.y
        ),
        u.z
    );
}

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    let noise_scale = rock.x;
    let wp = in.world_position.xyz;

    // 4 octaves: macro colour patches → base variation → medium → fine grit
    let n0 = value_noise(wp * noise_scale * 0.25);   // large colour patches
    let n1 = value_noise(wp * noise_scale);           // base variation
    let n2 = value_noise(wp * noise_scale * 2.3);     // medium detail
    let n3 = value_noise(wp * noise_scale * 5.1);     // fine surface grit
    let n  = n0 * 0.30 + n1 * 0.35 + n2 * 0.25 + n3 * 0.10;
    let variation = (n - 0.5) * 0.40;                // ±20% brightness shift

    // Approximate surface normal from noise gradient (central difference).
    // Perturbing N gives the illusion of bumpy stone without extra geometry.
    let eps = 0.08;
    let dnx = value_noise(wp * noise_scale + vec3(eps, 0.0, 0.0))
            - value_noise(wp * noise_scale - vec3(eps, 0.0, 0.0));
    let dnz = value_noise(wp * noise_scale + vec3(0.0, 0.0, eps))
            - value_noise(wp * noise_scale - vec3(0.0, 0.0, eps));
    pbr_input.N = normalize(pbr_input.N + vec3(dnx, 0.0, dnz) * 0.45);

    pbr_input.material.base_color = vec4(
        clamp(pbr_input.material.base_color.rgb + variation, vec3(0.0), vec3(1.0)),
        pbr_input.material.base_color.a,
    );

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
