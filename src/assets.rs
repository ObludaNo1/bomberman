pub mod bomb_explosion_tileset;
pub mod bomb_tileset;
pub mod character_tileset;
pub mod map_tileset;
mod tileset_enum;

use bevy::prelude::*;

pub const TILESET_TILE_SIZE: UVec2 = UVec2::new(16, 16);

const CHARACTER_TEXTURE_PATH: &str = "BombermanGB2-Bomberman.gif";
const ENEMIES_TEXTURE_PATH: &str = "BombermanGB2-enemies.gif";
const TILEMAP_TEXTURE_PATH: &str = "BombermanGB2-tiles.png";

#[derive(Resource, Clone)]
pub struct TilesetHandles {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}
