use bevy::prelude::*;

use crate::{
    assets::{ImageAssets, TILESET_TILE_SIZE, TilesetHandles, material::ColouringMaterial},
    tileset_enum,
};

const C1: f32 = 0.0 / 255.0;
const C2: f32 = 96.0 / 255.0;
const C3: f32 = 168.0 / 255.0;

tileset_enum!(
    Bomb,
    TILESET_TILE_SIZE,
    (222, 324),
    Bomb => (185, 154),
);

pub fn prepare_tilemap_material(
    image_assets: &ImageAssets,
    material: &mut Assets<ColouringMaterial>,
) -> TilesetHandles<ColouringMaterial> {
    TilesetHandles(material.add(ColouringMaterial::new(
        image_assets.enemies.clone(),
        TILEMAP.atlas_size,
        Color::srgba(C1, C1, C1, 1.0),
        Color::srgba(C2, C2, C2, 1.0),
        Color::srgba(C3, C3, C3, 1.0),
        Color::srgba(1.0, 1.0, 1.0, 0.0),
    )))
}
