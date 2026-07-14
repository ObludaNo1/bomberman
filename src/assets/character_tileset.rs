use bevy::{image::ImageLoaderSettings, prelude::*};

use crate::{
    assets::{
        CHARACTER_TEXTURE_PATH, TILESET_TILE_SIZE, TilesetHandles, material::ColouringMaterial,
    },
    tileset_enum,
};

tileset_enum!(
    Character,
    TILESET_TILE_SIZE,
    (222, 205),
    StandingDown => (1, 84),
    MovingDown => (18, 84),
    StandingUp => (35, 84),
    MovingUp => (52, 84),
    MovingRight1 => (69, 84),
    StandingRight => (86, 84),
    MovingRight2 => (103, 84),
    Death1 => (120, 84),
    Death2 => (137, 84),
    Death3 => (154, 84),
    Death4 => (171, 84),
    Empty => (188, 84),

);

pub fn prepare_tilemap_material(
    asset_server: &AssetServer,
    material: &mut Assets<ColouringMaterial>,
) -> TilesetHandles<ColouringMaterial> {
    let image = asset_server.load_with_settings::<Image, ImageLoaderSettings>(
        CHARACTER_TEXTURE_PATH,
        |settings| {
            settings.is_srgb = false;
        },
    );

    let material = material.add(ColouringMaterial::new(
        image,
        TILEMAP.atlas_size,
        Color::srgba(0.0, 0.0, 0.0, 1.0),
        Color::srgba(0.3, 0.3, 0.3, 1.0),
        Color::srgba(0.6, 0.6, 0.6, 1.0),
        Color::srgba(1.0, 1.0, 1.0, 0.0),
    ));

    TilesetHandles(material)
}
