// Original ordered alpha-stage component; not game shader code.
struct Params { enabled: u32, comparison: u32, reference: u32, branch: u32, blend: u32, pad0:u32, pad1:u32, pad2:u32, };
@group(0) @binding(0) var source_texture: texture_2d<f32>;
@group(0) @binding(1) var source_sampler: sampler;
@group(0) @binding(2) var<uniform> params: Params;
@group(0) @binding(3) var destination_rgb: texture_2d<f32>;
struct Input {
    @location(0) clip: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) constant_color: vec4<f32>,
    @location(4) constant_mask: u32,
};
struct Varying {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) @interpolate(linear) color: vec4<f32>,
    @location(2) @interpolate(flat) constant_color: vec4<f32>,
    @location(3) @interpolate(flat) constant_mask: u32,
};
@vertex fn vertex(input: Input) -> Varying {
    var result: Varying;
    result.clip = input.clip;
    result.uv = input.uv;
    result.color = input.color;
    result.constant_color = input.constant_color;
    result.constant_mask = input.constant_mask;
    return result;
}
fn passes(alpha: u32) -> bool {
    if (params.enabled == 0u) { return true; }
    switch params.comparison {
        case 0u: { return false; }
        case 1u: { return true; }
        case 2u: { return alpha < params.reference; }
        case 3u: { return alpha <= params.reference; }
        case 4u: { return alpha == params.reference; }
        case 5u: { return alpha >= params.reference; }
        case 6u: { return alpha > params.reference; }
        default: { return alpha != params.reference; }
    }
}
struct Output {
    @location(0) rgb: vec4<f32>,
    @location(1) framebuffer_alpha: f32,
};
@fragment fn fragment(input: Varying) -> Output {
    let texel = round(textureSample(source_texture, source_sampler, input.uv) * 255.0);
    let constant = vec4<bool>((input.constant_mask & 1u) != 0u, (input.constant_mask & 2u) != 0u,
        (input.constant_mask & 4u) != 0u, (input.constant_mask & 8u) != 0u);
    let source_color = select(input.color, input.constant_color, constant);
    let color = floor(clamp(source_color, vec4<f32>(0.0), vec4<f32>(255.0)));
    let bytes = clamp(floor(texel * color / 128.0), vec4<f32>(0.0), vec4<f32>(255.0));
    let accepted = passes(u32(bytes.a));
    if ((params.branch == 0u && !accepted) || (params.branch == 1u && accepted)) { discard; }
    var result: Output;
    var rgb = bytes.rgb;
    if (params.blend == 1u) {
        let destination = vec3<i32>(round(textureLoad(destination_rgb,vec2<i32>(input.clip.xy),0).rgb * 255.0));
        let product = (vec3<i32>(bytes.rgb) - destination) * i32(bytes.a);
        let blended = destination + (product >> vec3<u32>(7u));
        rgb = vec3<f32>(clamp(blended,vec3<i32>(0),vec3<i32>(255)));
    }
    result.rgb = vec4<f32>(rgb / 255.0, 1.0);
    result.framebuffer_alpha = bytes.a / 255.0;
    return result;
}
