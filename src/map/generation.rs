use std::{iter, ops::IndexMut};

use bevy::prelude::*;
use rand::{Rng, seq::SliceRandom};

use crate::{
    constants::WALL_DENSITY,
    map::{
        TOTAL_MAP_HEIGHT, TOTAL_MAP_WIDTH, WorldMap,
        map_tile::{BaseTile, MapTile, SpecialTile},
    },
    world_entities::BonusType,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BaseTileBuilder {
    Floor,
    BasicWall,
    IndestructibleWall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpecialTileBuilder {
    Exit,
    Bonus(BonusType),
}

#[derive(Debug, Clone, Copy)]
struct TileBuilder {
    base_tile: BaseTileBuilder,
    special_tile: Option<SpecialTileBuilder>,
}

impl WorldMap {
    pub fn get_starting_area_indices() -> [usize; 3] {
        let width = TOTAL_MAP_WIDTH;
        let height = TOTAL_MAP_HEIGHT;
        [
            width * (height - 2) + 1,
            width * (height - 2) + 2,
            width * (height - 3) + 1,
        ]
    }

    pub fn new_random_default(rng_gen: &mut impl Rng) -> Self {
        let height = TOTAL_MAP_HEIGHT;
        let width = TOTAL_MAP_WIDTH;

        let mut tiles = (0..height)
            .flat_map(|y| {
                (0..width).map(move |x| TileBuilder {
                    base_tile: if x == 0
                        || x == width - 1
                        || y == 0
                        || y == height - 1
                        || (x % 2 == 0 && y % 2 == 0)
                    {
                        BaseTileBuilder::IndestructibleWall
                    } else {
                        BaseTileBuilder::Floor
                    },
                    special_tile: None,
                })
            })
            .collect::<Vec<_>>();

        // Map is generated from bottom left, starting area is top left
        let starting_area_indices = Self::get_starting_area_indices();
        let mut empty_tiles_indices = tiles
            .iter()
            .enumerate()
            .filter(|(i, _)| !starting_area_indices.contains(i))
            .filter_map(|(i, tile)| (tile.base_tile == BaseTileBuilder::Floor).then_some(i))
            .collect::<Vec<_>>();
        empty_tiles_indices.shuffle(rng_gen);

        let empty_tiles_count = empty_tiles_indices.len();
        // There has to be at least one wall to place gate somewhere
        let wall_count =
            ((empty_tiles_count as f32 * WALL_DENSITY) as usize).clamp(1, empty_tiles_count - 1);

        let bonuses_count = [
            (BonusType::Range, 5),
            (BonusType::BombCount, 5),
            (BonusType::Negative, 5),
            (BonusType::ExtraLife, 0),
            (BonusType::Hook, 0),
            (BonusType::BombKick, 0),
            (BonusType::Detonator, 0),
            (BonusType::Turbo, 0),
            (BonusType::LineBomb, 0),
            (BonusType::DoubleBomb, 0),
            (BonusType::Max, 0),
        ];

        let mut all_bonuses = bonuses_count
            .iter()
            .copied()
            .flat_map(|(bonus, count)| (0..count).map(move |_| bonus))
            .collect::<Vec<_>>();
        all_bonuses.shuffle(rng_gen);

        let bonuses_and_exit = iter::once(SpecialTileBuilder::Exit)
            .chain(
                all_bonuses
                    .into_iter()
                    .map(|b| SpecialTileBuilder::Bonus(b)),
            )
            .take(wall_count);

        for index in empty_tiles_indices.iter().copied().take(wall_count) {
            tiles.index_mut(index).base_tile = BaseTileBuilder::BasicWall;
        }
        for (index, special) in empty_tiles_indices
            .iter()
            .copied()
            .zip(bonuses_and_exit)
            .take(wall_count)
        {
            tiles.index_mut(index).special_tile = Some(special)
        }
        // There exits at least one empty tile
        let exit_gate_index = empty_tiles_indices[0];

        let tiles = tiles
            .into_iter()
            .map(|tb| {
                let base_tile = match tb.base_tile {
                    BaseTileBuilder::Floor => BaseTile::Floor,
                    BaseTileBuilder::BasicWall => BaseTile::BasicWall,
                    BaseTileBuilder::IndestructibleWall => BaseTile::IndestructibleWall,
                };
                let special = match tb.special_tile {
                    Some(SpecialTileBuilder::Exit) => Some(SpecialTile::ClosedExit),
                    Some(SpecialTileBuilder::Bonus(b)) => Some(SpecialTile::Bonus(b)),
                    None => None,
                };
                MapTile::new(base_tile, special)
            })
            .collect::<Vec<_>>();

        Self {
            tiles,
            exit_gate_index,
        }
    }
}
