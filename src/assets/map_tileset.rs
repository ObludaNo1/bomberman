mod basic_map_tileset;
mod non_standard_map_tileset;

pub use basic_map_tileset::{MapTileType, TILEMAP as BASIC_TILEMAP};
use bevy::{image::ImageLoaderSettings, prelude::*};
pub use non_standard_map_tileset::{MapGateTileType, TILEMAP as NON_STANDARD_TILEMAP};

use crate::assets::{TILEMAP_TEXTURE_PATH, TilesetHandles, material::ColouringMaterial};

#[derive(Debug, Clone)]
pub struct MapTilesetHandles {
    pub floor: TilesetHandles<ColouringMaterial>,
    pub basic: TilesetHandles<ColouringMaterial>,
    pub non_standard: TilesetHandles<ColouringMaterial>,
}

const C1: f32 = 0.0 / 255.0;
const C2: f32 = 96.0 / 255.0;
const C3: f32 = 168.0 / 255.0;

pub fn prepare_tilemap_material(
    asset_server: &AssetServer,
    material: &mut Assets<ColouringMaterial>,
) -> MapTilesetHandles {
    let image = asset_server.load_with_settings::<Image, ImageLoaderSettings>(
        TILEMAP_TEXTURE_PATH,
        |settings| {
            settings.is_srgb = false;
        },
    );

    let colours = (
        Color::srgba(C1, C1, C1, 1.0),
        Color::srgba(C2, C2, C2, 1.0),
        Color::srgba(C3, C3, C3, 1.0),
        Color::srgba(1.0, 1.0, 1.0, 0.0),
    );

    let floor_material = material.add(ColouringMaterial::new(
        image.clone(),
        basic_map_tileset::TILEMAP.atlas_size,
        colours.0,
        colours.1,
        colours.2,
        Color::srgba(0.85, 0.95, 0.75, 1.0),
    ));
    let basic_material = material.add(ColouringMaterial::new(
        image.clone(),
        basic_map_tileset::TILEMAP.atlas_size,
        colours.0,
        colours.1,
        colours.2,
        colours.3,
    ));
    let non_standard_material = material.add(ColouringMaterial::new(
        image,
        non_standard_map_tileset::TILEMAP.atlas_size,
        colours.0,
        colours.1,
        colours.2,
        colours.3,
    ));

    MapTilesetHandles {
        floor: TilesetHandles(floor_material),
        basic: TilesetHandles(basic_material),
        non_standard: TilesetHandles(non_standard_material),
    }
}
