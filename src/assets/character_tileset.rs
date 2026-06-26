use bevy::prelude::*;

use crate::{
    assets::{CHARACTER_TEXTURE_PATH, TILESET_TILE_SIZE, TilesetHandles},
    tileset_enum,
};

tileset_enum!(
    CharacterTileType,
    StandingDown => 1, 84,
    MovingDown => 18, 84,
    StandingUp => 35, 84,
    MovingUp => 52, 84,
    MovingRight1 => 69, 84,
    StandingRight => 86, 84,
    MovingRight2 => 103, 84,
);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharacterTileset {
    pub atlas_size: UVec2,
    /// The position of the top-left corner of each sprite in the atlas, in pixels.
    sprites: [UVec2; CharacterTileType::COUNT],
}

impl CharacterTileset {
    pub fn sprite_topleft_position(&self, sprite_type: CharacterTileType) -> UVec2 {
        self.sprites[sprite_type as usize]
    }

    pub fn sprite_rect(&self, sprite_type: CharacterTileType) -> URect {
        let topleft = self.sprite_topleft_position(sprite_type);
        URect::from_corners(topleft, topleft + TILESET_TILE_SIZE.x)
    }

    pub fn sprites_iter(&self) -> impl Iterator<Item = (CharacterTileType, URect)> {
        CharacterTileType::VARIANTS
            .iter()
            .map(|v| (*v, self.sprite_rect(*v)))
    }
}

pub const CHARACTER_TILESET: CharacterTileset = CharacterTileset {
    atlas_size: UVec2::new(222, 205),
    sprites: SPRITES,
};

pub fn prepare_character_tileset_handles(
    asset_server: &AssetServer,
    atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> TilesetHandles {
    let image = asset_server.load::<Image>(CHARACTER_TEXTURE_PATH);
    let mut layout = TextureAtlasLayout::new_empty(CHARACTER_TILESET.atlas_size);
    for (_tile_type, rect) in CHARACTER_TILESET.sprites_iter() {
        layout.add_texture(rect);
    }
    let layout = atlas_layouts.add(layout);

    TilesetHandles { image, layout }
}
