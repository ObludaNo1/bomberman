pub mod bomb_explosion_tileset;
pub mod bomb_tileset;
pub mod character_tileset;
pub mod map_tileset;
pub mod material;
mod tileset_enum;

use bevy::{prelude::*, sprite_render::Material2d};

pub const TILESET_TILE_SIZE: UVec2 = UVec2::new(16, 16);

const CHARACTER_TEXTURE_PATH: &str = "BombermanGB2-Bomberman.gif";
const ENEMIES_TEXTURE_PATH: &str = "BombermanGB2-enemies.gif";
const TILEMAP_TEXTURE_PATH: &str = "BombermanGB2-tiles.png";

const COLOURING_SHADER_PATH: &str = "colouring.wgsl";
const EXPLOSIONS_SHADER_PATH: &str = "explosions.wgsl";
const SECOND_PASS_SHADER_PATH: &str = "second_pass.wgsl";

#[derive(Debug, Resource, Clone, Deref, DerefMut)]
pub struct TilesetHandles<M: Material2d + Clone>(pub Handle<M>);
