use bevy::prelude::*;

use crate::{
    constants::{BOMB_EXPLOSION_DURATION, TOTAL_MAP_WIDTH, WALL_BREAK_DURATION},
    map::{ExplosionTile, WorldMap, map_tile::BaseTile},
    position::TilePosition,
    world_entities::{ExplosionOrientation, ExplosionVariant},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExplosionVisual {
    pub variant: ExplosionVariant,
    pub pos: TilePosition,
}

impl ExplosionVisual {
    fn new(variant: ExplosionVariant, pos: TilePosition) -> Self {
        Self { variant, pos }
    }
}

const INDICES_INCREMENTS: [(isize, ExplosionOrientation); 4] = [
    (-1, ExplosionOrientation::Left),
    (1, ExplosionOrientation::Right),
    (-(TOTAL_MAP_WIDTH as isize), ExplosionOrientation::Down),
    (TOTAL_MAP_WIDTH as isize, ExplosionOrientation::Up),
];

impl WorldMap {
    /// Explodes all bombs on the map and returns a vector of explosion paths that can be used to
    /// spawn explosion entities.
    pub fn explode_bombs(&mut self) -> Vec<ExplosionVisual> {
        let mut exploding_bombs = self
            .tiles
            .iter()
            .enumerate()
            .filter_map(|(index, tile)| {
                tile.bomb_or_explosion().and_then(|v| {
                    v.bomb()
                        .and_then(|b| b.timer.is_finished().then_some((index, b.range)))
                })
            })
            .collect::<Vec<_>>();

        if exploding_bombs.is_empty() {
            return Vec::new();
        }

        let mut explosion_centers = Vec::new();

        let mut new_visuals = Vec::new();

        // Repeat the explosion process until there are no more bombs to explode.
        while !exploding_bombs.is_empty() {
            // Firstly remove all affected bombs from map to prevent double explosions of infinite
            // loops
            while let Some((i, range)) = exploding_bombs.pop() {
                explosion_centers.push((i, range));
                let tile = &mut self.tiles[i];
                tile.set_explosion(ExplosionTile(Timer::new(
                    BOMB_EXPLOSION_DURATION,
                    TimerMode::Once,
                )));
                tile.remove_bonus();
            }

            // Secondly traverse all explosions and search for all chain exploded bombs.
            while let Some((index, range)) = explosion_centers.pop() {
                new_visuals.push(ExplosionVisual::new(
                    ExplosionVariant::Center,
                    Self::index_to_tile_pos(index),
                ));
                for (increment, orientation) in INDICES_INCREMENTS {
                    for mult in 1..=range {
                        let i = (index as isize + increment * (mult as isize)) as usize;
                        let tile = &mut self.tiles[i];
                        match tile.base_type() {
                            // For floor tiles propagate the explosion and mark all the bombs for
                            // chain explosions.
                            BaseTile::Floor => {
                                let path = if mult == range {
                                    ExplosionVariant::End(orientation)
                                } else {
                                    ExplosionVariant::Straight(orientation)
                                };
                                new_visuals
                                    .push(ExplosionVisual::new(path, Self::index_to_tile_pos(i)));
                                if let Some(range) = tile
                                    .bomb_or_explosion()
                                    .as_ref()
                                    .and_then(|v| v.bomb().map(|b| b.range))
                                {
                                    exploding_bombs.push((i, range));
                                }
                                // There is a change we set the explosion multiple times since it is
                                // already set for explosion center tile some explosions may
                                // actually overlap. However setting is again is desired behaviour
                                // since overriding it with the new timer means the explosion will
                                // linger for the longest duration.
                                tile.set_explosion(ExplosionTile(Timer::new(
                                    BOMB_EXPLOSION_DURATION,
                                    TimerMode::Once,
                                )));
                                tile.remove_bonus();
                            }
                            // Indestructible walls completely stop the explosion propagation.
                            BaseTile::IndestructibleWall => break,
                            // Breaking walls are already in a process of being destroyed so they
                            // only stop the propagation.
                            BaseTile::BreakingWall(_) => {
                                new_visuals.push(ExplosionVisual::new(
                                    ExplosionVariant::End(orientation),
                                    Self::index_to_tile_pos(i),
                                ));
                                break;
                            }
                            // Basic walls are destroyed by the explosion and stop the propagation.
                            BaseTile::BasicWall => {
                                new_visuals.push(ExplosionVisual::new(
                                    ExplosionVariant::End(orientation),
                                    Self::index_to_tile_pos(i),
                                ));
                                tile.break_wall(Timer::new(WALL_BREAK_DURATION, TimerMode::Once));
                                break;
                            }
                        }
                    }
                }
            }
        }

        new_visuals
    }
}
