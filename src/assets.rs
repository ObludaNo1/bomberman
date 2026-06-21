pub mod map_tileset;

use bevy::prelude::*;

use crate::assets::map_tileset::TILEMAP;

pub const TILEMAP_TEXTURE_PATH: &str = "BombermanGB2-tiles.png";

#[derive(Resource, Clone)]
pub struct TilemapHandles {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

pub fn prepare_tilemap_handles(
    asset_server: &AssetServer,
    atlas_layouts: &mut Assets<TextureAtlasLayout>,
    tilemap_path: &'static str,
) -> TilemapHandles {
    let image = asset_server.load::<Image>(tilemap_path);
    let mut layout = TextureAtlasLayout::new_empty(TILEMAP.atlas_size);
    for (_tile_type, rect) in TILEMAP.sprites_iter() {
        layout.add_texture(rect);
    }
    let layout = atlas_layouts.add(layout);

    TilemapHandles { image, layout }
}
