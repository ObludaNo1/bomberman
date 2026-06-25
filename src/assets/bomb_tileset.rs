use bevy::prelude::*;

use crate::{
    assets::{TILESET_TILE_SIZE, TilesetHandles},
    tileset_enum,
};

tileset_enum!(
    BombTileType,
    Bomb => 185, 154,
);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BombTileset {
    pub atlas_size: UVec2,
    /// The position of the top-left corner of each sprite in the atlas, in pixels.
    sprites: [UVec2; BombTileType::COUNT],
}

impl BombTileset {
    pub fn sprite_topleft_position(&self, sprite_type: BombTileType) -> UVec2 {
        self.sprites[sprite_type as usize]
    }

    pub fn sprite_rect(&self, sprite_type: BombTileType) -> URect {
        let topleft = self.sprite_topleft_position(sprite_type);
        URect::from_corners(topleft, topleft + TILESET_TILE_SIZE.x)
    }

    pub fn sprites_iter(&self) -> impl Iterator<Item = (BombTileType, URect)> {
        BombTileType::VARIANTS
            .iter()
            .map(|v| (*v, self.sprite_rect(*v)))
    }
}

pub const BOMB_TILESET: BombTileset = BombTileset {
    atlas_size: UVec2::new(222, 324),
    sprites: SPRITES,
};

pub const ENEMIES_TEXTURE_PATH: &str = "BombermanGB2-enemies.gif";

pub fn prepare_bomb_tileset_handles(
    asset_server: &AssetServer,
    atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> TilesetHandles {
    let image = asset_server.load::<Image>(ENEMIES_TEXTURE_PATH);
    let mut layout = TextureAtlasLayout::new_empty(BOMB_TILESET.atlas_size);
    for (_tile_type, rect) in BOMB_TILESET.sprites_iter() {
        layout.add_texture(rect);
    }
    let layout = atlas_layouts.add(layout);

    TilesetHandles { image, layout }
}
