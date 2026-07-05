pub mod bomb_explosion_tileset;
pub mod bomb_tileset;
pub mod character_tileset;
pub mod map_tileset;
pub mod material;
mod tileset_enum;

use bevy::prelude::*;

use crate::assets::material::ColouringMaterial;

pub const TILESET_TILE_SIZE: UVec2 = UVec2::new(16, 16);

const CHARACTER_TEXTURE_PATH: &str = "BombermanGB2-Bomberman.gif";
const ENEMIES_TEXTURE_PATH: &str = "BombermanGB2-enemies.gif";
const TILEMAP_TEXTURE_PATH: &str = "BombermanGB2-tiles.png";
const COLOURING_SHADER_PATH: &str = "colouring.wgsl";

#[derive(Resource, Clone)]
pub struct TilesetHandles {
    pub colouring: Handle<ColouringMaterial>,
}
