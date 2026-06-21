use bevy::prelude::*;

use crate::{
    assets::{TILESET_TILE_SIZE, TilesetHandles},
    tileset_enum,
};

tileset_enum!(
    MapTileType,
    Floor => 86, 417,
    Wall => 1, 417,
    IndestructibleWall => 120, 417,
);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapTileset {
    pub atlas_size: UVec2,
    /// The position of the top-left corner of each sprite in the atlas, in pixels.
    sprites: [UVec2; MapTileType::COUNT],
}

impl MapTileset {
    pub fn sprite_topleft_position(&self, sprite_type: MapTileType) -> UVec2 {
        self.sprites[sprite_type as usize]
    }

    pub fn sprite_rect(&self, sprite_type: MapTileType) -> URect {
        let topleft = self.sprite_topleft_position(sprite_type);
        URect::from_corners(topleft, topleft + TILESET_TILE_SIZE)
    }

    pub fn sprites_iter(&self) -> impl Iterator<Item = (MapTileType, URect)> {
        MapTileType::VARIANTS
            .iter()
            .map(|v| (*v, self.sprite_rect(*v)))
    }
}

pub const TILEMAP: MapTileset = MapTileset {
    atlas_size: UVec2::new(255, 434),
    sprites: SPRITES,
};

pub const TILEMAP_TEXTURE_PATH: &str = "BombermanGB2-tiles.png";

pub fn prepare_tilemap_handles(
    asset_server: &AssetServer,
    atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> TilesetHandles {
    let image = asset_server.load::<Image>(TILEMAP_TEXTURE_PATH);
    let mut layout = TextureAtlasLayout::new_empty(TILEMAP.atlas_size);
    for (_tile_type, rect) in TILEMAP.sprites_iter() {
        layout.add_texture(rect);
    }
    let layout = atlas_layouts.add(layout);

    TilesetHandles { image, layout }
}
