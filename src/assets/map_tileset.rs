use bevy::prelude::*;

use crate::tileset_enum;

tileset_enum!(
    MapTileType,
    Floor => 86, 417,
    Wall => 1, 417,
    IndestructibleWall => 120, 417,
);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapTileset {
    pub atlas_size: UVec2,
    pub tile_size: UVec2,
    /// The position of the top-left corner of each sprite in the atlas, in pixels.
    sprites: [UVec2; MapTileType::COUNT],
}

impl MapTileset {
    pub fn sprite_topleft_position(&self, sprite_type: MapTileType) -> UVec2 {
        self.sprites[sprite_type as usize]
    }

    pub fn sprite_rect(&self, sprite_type: MapTileType) -> URect {
        let topleft = self.sprite_topleft_position(sprite_type);
        URect::from_corners(topleft, topleft + self.tile_size)
    }

    pub fn sprites_iter(&self) -> impl Iterator<Item = (MapTileType, URect)> {
        MapTileType::VARIANTS
            .iter()
            .map(|v| (*v, self.sprite_rect(*v)))
    }
}

pub const TILEMAP: MapTileset = MapTileset {
    atlas_size: UVec2::new(255, 434),
    tile_size: UVec2::new(16, 16),
    sprites: SPRITES,
};
