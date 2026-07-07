use bevy::{image::ImageLoaderSettings, prelude::*};

use crate::{
    assets::{
        ENEMIES_TEXTURE_PATH, TILESET_TILE_SIZE, TilesetHandles, material::ColouringMaterial,
    },
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
    asset_server: &AssetServer,
    material: &mut Assets<ColouringMaterial>,
) -> TilesetHandles<ColouringMaterial> {
    let image = asset_server.load_with_settings::<Image, ImageLoaderSettings>(
        ENEMIES_TEXTURE_PATH,
        |settings| {
            settings.is_srgb = false;
        },
    );

    let material = material.add(ColouringMaterial::new(
        image,
        TILEMAP.atlas_size,
        Color::srgba(C1, C1, C1, 1.0),
        Color::srgba(C2, C2, C2, 1.0),
        Color::srgba(C3, C3, C3, 1.0),
        Color::srgba(1.0, 1.0, 1.0, 0.0),
    ));

    TilesetHandles(material)
}
