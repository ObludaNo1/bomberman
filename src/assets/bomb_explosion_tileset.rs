use bevy::prelude::*;

use crate::{
    assets::{ImageAssets, TILESET_TILE_SIZE, TilesetHandles, material::ExplosionMaterial},
    tileset_enum,
};

tileset_enum!(
    BombExplosion,
    TILESET_TILE_SIZE,
    (222, 205),
    ExplosionCenter1 => (19, 170),
    ExplosionStraight1 => (36, 153),
    ExplosionEnd1 => (2, 153),
    ExplosionCenter2 => (72, 170),
    ExplosionStraight2 => (89, 153),
    ExplosionEnd2 => (55, 153),
    ExplosionCenter3 => (125, 170),
    ExplosionStraight3 => (142, 153),
    ExplosionEnd3 => (108, 153),
    ExplosionCenter4 => (178, 170),
    ExplosionStraight4 => (195, 153),
    ExplosionEnd4 => (161, 153),
);

pub fn prepare_tilemap_material(
    image_assets: &ImageAssets,
    material: &mut Assets<ExplosionMaterial>,
) -> TilesetHandles<ExplosionMaterial> {
    TilesetHandles(material.add(ExplosionMaterial::new(
        image_assets.character.clone(),
        TILEMAP.atlas_size,
    )))
}

// Color::srgba(0.9, 0.2, 0.05, 1.0),
// Color::srgba(0.9, 0.65, 0.05, 0.75),
// Color::srgba(0.9, 0.9, 0.05, 0.5),
// Color::srgba(0.0, 0.0, 0.0, 0.0),
