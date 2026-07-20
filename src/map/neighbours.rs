use bevy::prelude::*;

use crate::{
    constants::TOTAL_MAP_WIDTH,
    map::{WorldMap, map_tile::MapTile},
    position::{TilePosition, WorldPosition},
};

#[derive(Debug, Clone)]
pub struct NeighbourTile<'world> {
    pub pos: TilePosition,
    pub tile: &'world MapTile,
}

#[derive(Debug, Clone)]
pub struct NeighbourTiles<'world> {
    pub top_left: NeighbourTile<'world>,
    pub top_right: NeighbourTile<'world>,
    pub bottom_left: NeighbourTile<'world>,
    pub bottom_right: NeighbourTile<'world>,
}

impl<'world> NeighbourTiles<'world> {
    pub fn iter(&'world self) -> impl Iterator<Item = &'world NeighbourTile<'world>> {
        [
            &self.top_left, &self.top_right, &self.bottom_left, &self.bottom_right,
        ]
        .into_iter()
    }
}

impl<'world> WorldMap {
    /// Let's assume that the world position is always within the map bounds. Otherwise it makes no
    /// sense.
    pub fn world_position_neighbours(
        &'world self,
        pos: WorldPosition,
    ) -> Option<NeighbourTiles<'world>> {
        let top_left = WorldPosition(pos.0 + Vec2::new(-0.5, 0.5)).to_closest_tile();
        let top_right = WorldPosition(pos.0 + Vec2::new(0.5, 0.5)).to_closest_tile();
        let bottom_left = WorldPosition(pos.0 + Vec2::new(-0.5, -0.5)).to_closest_tile();
        let bottom_right = WorldPosition(pos.0 + Vec2::new(0.5, -0.5)).to_closest_tile();

        Some(NeighbourTiles {
            top_left: NeighbourTile {
                pos: top_left,
                tile: self.get_tile(top_left)?,
            },
            top_right: NeighbourTile {
                pos: top_right,
                tile: self.get_tile(top_right)?,
            },
            bottom_left: NeighbourTile {
                pos: bottom_left,
                tile: self.get_tile(bottom_left)?,
            },
            bottom_right: NeighbourTile {
                pos: bottom_right,
                tile: self.get_tile(bottom_right)?,
            },
        })
    }
}

const INDICES_INCREMENTS: [isize; 4] =
    [-1, 1, -(TOTAL_MAP_WIDTH as isize), TOTAL_MAP_WIDTH as isize];

impl<'world> WorldMap {
    pub fn tile_neighbours(
        &'world self,
        pos: TilePosition,
    ) -> impl Iterator<Item = (TilePosition, &'world MapTile)> {
        INDICES_INCREMENTS.clone().into_iter().filter_map(move |i| {
            let base_index = Self::pos_to_index(pos)?;
            let i = (base_index as isize + i) as usize;
            let pos = Self::index_to_tile_pos(i);
            let tile = self.get_tile(pos)?;

            Some((pos, tile))
        })
    }
}
