use bevy::{image::ImageLoaderSettings, prelude::*};

use crate::{
    assets::{
        ENEMIES_TEXTURE_PATH, TILESET_TILE_SIZE, TilesetHandles, material::ColouringMaterial,
    },
    tileset_enum,
};

tileset_enum!(
    Enemy,
    TILESET_TILE_SIZE,
    (222, 324),
    HoodieLeft1 => (1, 1),
    HoodieLeft2 => (18, 1),
    HoodieLeft3 => (35, 1),
    HoodieDown1 => (52, 1),
    HoodieDown2 => (69, 1),
    HoodieDown3 => (86, 1),
    HoodieUp1 => (103, 1),
    HoodieUp2 => (120, 1),
    HoodieUp3 => (137, 1),
    HoodieDeath1 => (154, 1),
    HoodieDeath2 => (171, 1),
    HoodieDeath3 => (188, 1),
    GhostLeft1 => (1, 18),
    GhostLeft2 => (18, 18),
    GhostLeft3 => (35, 18),
    GhostDown1 => (52, 18),
    GhostDown2 => (69, 18),
    GhostDown3 => (86, 18),
    GhostUp1 => (103, 18),
    GhostUp2 => (120, 18),
    GhostUp3 => (137, 18),
    GhostDeath1 => (154, 18),
    GhostDeath2 => (171, 18),
    GhostDeath3 => (188, 18),
    GhostDeath4 => (205, 18),
    ZombieLeft1 => (1, 137),
    ZombieLeft2 => (18, 137),
    ZombieLeft3 => (35, 137),
    ZombieDown1 => (52, 137),
    ZombieDown2 => (69, 137),
    ZombieDown3 => (86, 137),
    ZombieUp1 => (103, 137),
    ZombieUp2 => (120, 137),
    ZombieUp3 => (137, 137),
    ZombieDeath1 => (154, 137),
    ZombieDeath2 => (171, 137),
    ZombieDeath3 => (188, 137),
    ZombieDeath4 => (205, 137),
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
        Color::srgba(0.0, 0.0, 0.0, 1.0),
        Color::srgba(0.3, 0.3, 0.3, 1.0),
        Color::srgba(0.6, 0.6, 0.6, 1.0),
        Color::srgba(1.0, 1.0, 1.0, 0.0),
    ));

    TilesetHandles(material)
}
