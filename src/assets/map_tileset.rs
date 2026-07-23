mod basic_map_tileset;
mod bonuses_tileset;
mod non_standard_map_tileset;

pub use basic_map_tileset::{MapTileType, TILEMAP as BASIC_TILEMAP};
use bevy::prelude::*;
pub use bonuses_tileset::{PowerUpTileType, TILEMAP as BONUSES_TILEMAP};
pub use non_standard_map_tileset::{MapGateTileType, TILEMAP as NON_STANDARD_TILEMAP};

use crate::assets::{ImageAssets, TilesetHandles, material::ColouringMaterial};

#[derive(Debug, Clone)]
pub struct MapTilesetHandles {
    pub floor: TilesetHandles<ColouringMaterial>,
    pub basic: TilesetHandles<ColouringMaterial>,
    pub non_standard: TilesetHandles<ColouringMaterial>,
    pub bonuses: TilesetHandles<ColouringMaterial>,
}

const C1: f32 = 0.0 / 255.0;
const C2: f32 = 96.0 / 255.0;
const C3: f32 = 168.0 / 255.0;

pub fn prepare_tilemap_material(
    image_assets: &ImageAssets,
    material: &mut Assets<ColouringMaterial>,
) -> MapTilesetHandles {
    let colours = (
        Color::srgba(C1, C1, C1, 1.0),
        Color::srgba(C2, C2, C2, 1.0),
        Color::srgba(C3, C3, C3, 1.0),
        Color::srgba(1.0, 1.0, 1.0, 0.0),
    );

    let floor_material = material.add(ColouringMaterial::new(
        image_assets.tilemap.clone(),
        basic_map_tileset::TILEMAP.atlas_size,
        colours.0,
        colours.1,
        colours.2,
        Color::srgba(0.85, 0.95, 0.75, 1.0),
    ));
    let basic_material = material.add(ColouringMaterial::new(
        image_assets.tilemap.clone(),
        basic_map_tileset::TILEMAP.atlas_size,
        colours.0,
        colours.1,
        colours.2,
        colours.3,
    ));
    let non_standard_material = material.add(ColouringMaterial::new(
        image_assets.tilemap.clone(),
        non_standard_map_tileset::TILEMAP.atlas_size,
        colours.0,
        colours.1,
        colours.2,
        colours.3,
    ));
    let bonuses_material = material.add(ColouringMaterial::new(
        image_assets.character.clone(),
        bonuses_tileset::TILEMAP.atlas_size,
        colours.0,
        colours.1,
        colours.2,
        colours.3,
    ));

    MapTilesetHandles {
        floor: TilesetHandles(floor_material),
        basic: TilesetHandles(basic_material),
        non_standard: TilesetHandles(non_standard_material),
        bonuses: TilesetHandles(bonuses_material),
    }
}
