#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var tileset_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var tileset_texture_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> colour_1: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> colour_2: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> colour_3: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var<uniform> colour_4: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(6) var<uniform> uv_min: vec2<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(7) var<uniform> uv_max: vec2<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(8) var<uniform> flip_x: f32;

const BLACK_COLOUR: f32 = 0.0 / 255.0;
const DARK_COLOUR: f32 = 96.0 / 255.0;
const LIGHT_COLOUR: f32 = 168.0 / 255.0;
const WHITE_COLOUR: f32 = 248.0 / 255.0;

// Compute thresholds as the midpoint between each pair of colours, normalized to [0, 1] range
const BLACK_THRESHOLD: f32 = BLACK_COLOUR + (DARK_COLOUR - BLACK_COLOUR) / 2.0;
const DARK_THRESHOLD: f32 = DARK_COLOUR + (LIGHT_COLOUR - DARK_COLOUR) / 2.0;
const LIGHT_THRESHOLD: f32 = LIGHT_COLOUR + (WHITE_COLOUR - LIGHT_COLOUR) / 2.0;

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    var uv = input.uv;
    if (flip_x > 0.5) {
        uv.x = 1.0 - uv.x;
    }
    let atlas_uv = uv_min + uv * (uv_max - uv_min);
    let texture_r = textureSample(tileset_texture, tileset_texture_sampler, atlas_uv).r;

    if (texture_r <= BLACK_THRESHOLD) {
        return colour_1;
    } else if (texture_r <= DARK_THRESHOLD) {
        return colour_2;
    } else if (texture_r <= LIGHT_THRESHOLD) {
        return colour_3;
    } else {
        return colour_4;
    }
}