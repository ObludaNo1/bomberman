use bevy::{
    mesh::MeshVertexBufferLayoutRef,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{
        AsBindGroup, BlendComponent, BlendFactor, BlendOperation, BlendState, ColorWrites,
        RenderPipelineDescriptor, SpecializedMeshPipelineError,
    },
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dKey},
};

use crate::assets::COLOURING_SHADER_PATH;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
// This material intentionally carries both palette data and per-entity frame data. In an ideal
// separation, UV/flip would live in a separate bind group, while palette data would be shared
// across all entities using the same tileset.
//
// We keep them together because this project uses Bevy's built-in Material2d path, where extending
// with an extra user-managed bind group requires a custom render pipeline. The current approach
// keeps rendering simple and still allows per-entity animation by giving each animated entity its
// own material handle.
pub struct ColouringMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub tileset_texture: Handle<Image>,

    #[uniform(2)]
    pub black_colour: LinearRgba,
    #[uniform(3)]
    pub dark_colour: LinearRgba,
    #[uniform(4)]
    pub light_colour: LinearRgba,
    #[uniform(5)]
    pub background_colour: LinearRgba,

    // Per-frame atlas UV rectangle written by gameplay systems.
    #[uniform(6)]
    pub uv_min: Vec2,
    #[uniform(7)]
    pub uv_max: Vec2,
    // Stored as f32 to avoid shader bool uniform layout pitfalls across backends.
    #[uniform(8)]
    pub flip_x: f32,

    #[uniform(9)]
    pub inset: Vec2,
}

impl ColouringMaterial {
    pub(super) fn new(
        tileset_texture: Handle<Image>,
        image_size: UVec2,
        black_colour: Color,
        dark_colour: Color,
        light_colour: Color,
        background_colour: Color,
    ) -> Self {
        Self {
            tileset_texture,
            inset: Vec2::new(0.75 / image_size.x as f32, 0.75 / image_size.y as f32),
            black_colour: black_colour.to_linear(),
            dark_colour: dark_colour.to_linear(),
            light_colour: light_colour.to_linear(),
            background_colour: background_colour.to_linear(),
            uv_min: Vec2::ZERO,
            uv_max: Vec2::ONE,
            flip_x: 0.0,
        }
    }

    pub fn set_uv_rect(&mut self, rect: Rect) {
        self.uv_min = rect.min;
        self.uv_max = rect.max;
    }

    pub fn set_flip_x(&mut self, flip_x: bool) {
        self.flip_x = if flip_x { 1.0 } else { 0.0 };
    }
}

impl Material2d for ColouringMaterial {
    fn fragment_shader() -> ShaderRef {
        COLOURING_SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // Set up additive blending for glowing effect
        if let Some(fragment) = &mut descriptor.fragment {
            if let Some(target) = fragment.targets.first_mut() {
                if let Some(target_state) = target.as_mut() {
                    target_state.blend = Some(BlendState {
                        color: BlendComponent {
                            src_factor: BlendFactor::SrcAlpha,
                            dst_factor: BlendFactor::OneMinusSrcAlpha,
                            operation: BlendOperation::Add,
                        },
                        alpha: BlendComponent {
                            src_factor: BlendFactor::SrcAlpha,
                            dst_factor: BlendFactor::OneMinusSrcAlpha,
                            operation: BlendOperation::Add,
                        },
                    });
                    target_state.write_mask = ColorWrites::ALL;
                }
            }
        }
        Ok(())
    }
}
