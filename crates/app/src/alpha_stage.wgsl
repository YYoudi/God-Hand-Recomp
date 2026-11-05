// Original ordered alpha-stage component; not game shader code.
struct Params { enabled: u32, comparison: u32, reference: u32, branch: u32, };
@group(0) @binding(0) var source_texture: texture_2d<f32>;
@group(0) @binding(1) var source_sampler: sampler;
@group(0) @binding(2) var<uniform> params: Params;
struct Input {
    @location(0) clip: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
};
struct Varying {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) @interpolate(linear) color: vec4<f32>,
};
@vertex fn vertex(input: Input) -> Varying {
    var result: Varying;
    result.clip = input.clip;
    result.uv = input.uv;
    result.color = input.color;
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
    let color = floor(clamp(input.color, vec4<f32>(0.0), vec4<f32>(255.0)));
    let bytes = clamp(floor(texel * color / 128.0), vec4<f32>(0.0), vec4<f32>(255.0));
    let accepted = passes(u32(bytes.a));
    if ((params.branch == 0u && !accepted) || (params.branch == 1u && accepted)) { discard; }
    var result: Output;
    result.rgb = vec4<f32>(bytes.rgb / 255.0, 1.0);
    result.framebuffer_alpha = bytes.a / 255.0;
    return result;
}
