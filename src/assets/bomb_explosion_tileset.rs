use bevy::prelude::*;

use crate::{
    assets::{CHARACTER_TEXTURE_PATH, TILESET_TILE_SIZE, TilesetHandles},
    tileset_enum,
};

tileset_enum!(
    BombExplosionTileType,
    ExplosionCenter1 => 19, 170,
    ExplosionStraight1 => 36, 153,
    ExplosionEnd1 => 2, 153,
    ExplosionCenter2 => 72, 170,
    ExplosionStraight2 => 89, 153,
    ExplosionEnd2 => 55, 153,
    ExplosionCenter3 => 125, 170,
    ExplosionStraight3 => 142, 153,
    ExplosionEnd3 => 108, 153,
    ExplosionCenter4 => 178, 170,
    ExplosionStraight4 => 195, 153,
    ExplosionEnd4 => 161, 153,
);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BombExplosionTileset {
    pub atlas_size: UVec2,
    /// The position of the top-left corner of each sprite in the atlas, in pixels.
    sprites: [UVec2; BombExplosionTileType::COUNT],
}

impl BombExplosionTileset {
    pub fn sprite_topleft_position(&self, sprite_type: BombExplosionTileType) -> UVec2 {
        self.sprites[sprite_type as usize]
    }

    pub fn sprite_rect(&self, sprite_type: BombExplosionTileType) -> URect {
        let topleft = self.sprite_topleft_position(sprite_type);
        URect::from_corners(topleft, topleft + TILESET_TILE_SIZE.x)
    }

    pub fn sprites_iter(&self) -> impl Iterator<Item = (BombExplosionTileType, URect)> {
        BombExplosionTileType::VARIANTS
            .iter()
            .map(|v| (*v, self.sprite_rect(*v)))
    }
}

pub const BOMB_TILESET: BombExplosionTileset = BombExplosionTileset {
    atlas_size: UVec2::new(222, 324),
    sprites: SPRITES,
};

pub fn prepare_bomb_explosion_tileset_handles(
    asset_server: &AssetServer,
    atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> TilesetHandles {
    let image = asset_server.load::<Image>(CHARACTER_TEXTURE_PATH);
    let mut layout = TextureAtlasLayout::new_empty(BOMB_TILESET.atlas_size);
    for (_tile_type, rect) in BOMB_TILESET.sprites_iter() {
        layout.add_texture(rect);
    }
    let layout = atlas_layouts.add(layout);

    TilesetHandles { image, layout }
}
