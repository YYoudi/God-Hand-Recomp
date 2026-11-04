// Original static-color inspection experiment. Contract: specs/static-colors-v1.md.
// Uses Bevy's public vertex/decompression interfaces, not any game shader code.
import bevy_pbr::render::{
    forward_io::{Vertex, decompress_vertex},
    mesh_functions::{get_world_from_local, mesh_position_local_to_world},
    view_transformations::position_world_to_clip,
};

@group(3) @binding(0) var color_texture: texture_2d<f32>;
@group(3) @binding(1) var color_sampler: sampler;

struct ColorOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) @interpolate(linear) color: vec4<f32>,
};

@vertex
fn vertex(input: Vertex) -> ColorOutput {
    let decoded = decompress_vertex(input, input.instance_index);
    let world = mesh_position_local_to_world(get_world_from_local(input.instance_index), vec4<f32>(decoded.position, 1.0));
    var output: ColorOutput;
    output.position = position_world_to_clip(world.xyz);
    output.uv = decoded.uv;
    output.color = decoded.color;
    return output;
}

fn display_to_linear(c: vec3<f32>) -> vec3<f32> {
    return select(pow((c + 0.055) / 1.055, vec3<f32>(2.4)), c / 12.92, c <= vec3<f32>(0.04045));
}

@fragment
fn fragment(input: ColorOutput) -> @location(0) vec4<f32> {
    let texture = textureSample(color_texture, color_sampler, input.uv);
    // Nearest UNORM sampling restores the original integer texture channels.
    let texel = round(texture.rgb * 255.0);
    let vertex_bytes = floor(clamp(input.color * 128.0 + 0.00001, vec4<f32>(0.0), vec4<f32>(255.0)));
    let rgb_bytes = clamp(floor(texel * vertex_bytes.rgb / 128.0), vec3<f32>(0.0), vec3<f32>(255.0));
    // Diagnostic host alpha only; not the complete GS alpha-test/blend pipeline.
    let alpha = clamp(texture.a * vertex_bytes.a / 128.0, 0.0, 1.0);
    return vec4<f32>(display_to_linear(rgb_bytes / 255.0), alpha);
}
