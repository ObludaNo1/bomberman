use bevy::prelude::*;

use crate::{
    assets::{TILEMAP_TEXTURE_PATH, TILESET_TILE_SIZE, material::ColouringMaterial},
    tileset_enum,
};

const C1: f32 = 0.0 / 255.0;
const C2: f32 = 96.0 / 255.0;
const C3: f32 = 168.0 / 255.0;

tileset_enum!(
    Map,
    TILESET_TILE_SIZE,
    (255, 434),
    TILEMAP_TEXTURE_PATH,
    ColouringMaterial,
    Color::srgba(C1, C1, C1, 1.0),
    Color::srgba(C2, C2, C2, 1.0),
    Color::srgba(C3, C3, C3, 1.0),
    Color::srgba(0.85, 0.95, 0.75, 1.0),
    Floor => (86, 417),
    // Wall => (1, 417),
    IndestructibleWall => (120, 417),
);
