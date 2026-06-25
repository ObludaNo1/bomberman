pub mod bomb_tileset;
pub mod character_tileset;
pub mod map_tileset;
mod tileset_enum;

use bevy::prelude::*;

pub const TILESET_TILE_SIZE: UVec2 = UVec2::new(16, 16);

#[derive(Resource, Clone)]
pub struct TilesetHandles {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}
