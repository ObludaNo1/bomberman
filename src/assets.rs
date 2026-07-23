pub mod audio;
pub mod bomb_explosion_tileset;
pub mod bomb_tileset;
pub mod character_tileset;
pub mod enemy_tileset;
pub mod map_tileset;
pub mod material;
mod tileset_enum;

use bevy::{image::ImageLoaderSettings, prelude::*, sprite_render::Material2d};

pub const TILESET_TILE_SIZE: UVec2 = UVec2::new(16, 16);

const CHARACTER_TEXTURE_PATH: &str = "BombermanGB2-Bomberman.gif";
const ENEMIES_TEXTURE_PATH: &str = "BombermanGB2-enemies.gif";
const TILEMAP_TEXTURE_PATH: &str = "BombermanGB2-tiles.png";

const COLOURING_SHADER_PATH: &str = "colouring.wgsl";
const EXPLOSIONS_SHADER_PATH: &str = "explosions.wgsl";
const SECOND_PASS_SHADER_PATH: &str = "second_pass.wgsl";

#[derive(Debug, Resource, Clone, Deref, DerefMut)]
pub struct TilesetHandles<M: Material2d>(pub Handle<M>);

#[derive(Resource, Debug, Clone)]
pub struct ImageAssets {
    pub character: Handle<Image>,
    pub enemies: Handle<Image>,
    pub tilemap: Handle<Image>,
}

impl FromWorld for ImageAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        ImageAssets {
            character: asset_server.load_with_settings::<Image, ImageLoaderSettings>(
                CHARACTER_TEXTURE_PATH,
                |settings| {
                    settings.is_srgb = false;
                },
            ),
            enemies: asset_server.load_with_settings::<Image, ImageLoaderSettings>(
                ENEMIES_TEXTURE_PATH,
                |settings| {
                    settings.is_srgb = false;
                },
            ),
            tilemap: asset_server.load_with_settings::<Image, ImageLoaderSettings>(
                TILEMAP_TEXTURE_PATH,
                |settings| {
                    settings.is_srgb = false;
                },
            ),
        }
    }
}
