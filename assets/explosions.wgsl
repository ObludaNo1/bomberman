#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var tileset_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var tileset_texture_sampler: sampler;

@group(#{MATERIAL_BIND_GROUP}) @binding(6) var<uniform> uv_min: vec2<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(7) var<uniform> uv_max: vec2<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(8) var<uniform> flip_x: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(9) var<uniform> inset: vec2<f32>;

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    var uv = input.uv;
    if (flip_x > 0.5) {
        uv.x = 1.0 - uv.x;
    }
    let atlas_uv = uv_min + uv * (uv_max - uv_min);
    // Clamp the UV coordinates to avoid sampling outside the intended area of the texture. We can
    // have a pixel which lies with less than 50% of its area inside the rasterized triangle and its
    // UV coordinates will be outside of usable range.
    let safe_uv = clamp(atlas_uv, uv_min + inset, uv_max - inset);
    return textureSample(tileset_texture, tileset_texture_sampler, safe_uv);



}