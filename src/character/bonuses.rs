use std::time::Duration;

use bevy::prelude::*;
use rand::RngExt;

use crate::{
    character::CharacterRng,
    map::{NeighbourTile, WorldMap},
    position::WorldPosition,
    sound::EffectKind,
    world_entities::{
        BombCount, BombRange, BonusType, Character, MovementMultiplier, SpeedUpEnemies,
    },
};

const PICKUP_DISTANCE: f32 = 0.75;

pub fn pick_up_bonuses(
    mut commands: Commands,
    mut players: Query<(Entity, &WorldPosition, &mut BombRange, &mut BombCount), With<Character>>,
    mut map: ResMut<WorldMap>,
    mut rng: ResMut<CharacterRng>,
) {
    for (character_entity, player_pos, mut bomb_range, mut max_bomb_count) in players.iter_mut() {
        let Some(neighbours) = map.world_position_neighbours(*player_pos) else {
            continue;
        };

        // It is needed to collect bonuses to prevent double borrow of map in the loop below
        let bonuses = neighbours
            .iter()
            .filter_map(|NeighbourTile { pos, tile }| {
                tile.special()
                    .and_then(|st| st.bonus())
                    .map(|bonus| (*pos, bonus))
            })
            .collect::<Vec<_>>();
        for (bonus_pos, bonus) in bonuses {
            if player_pos.0.distance(bonus_pos.to_world_position().0) < PICKUP_DISTANCE {
                match bonus {
                    BonusType::Range => bomb_range.0 += 1,
                    BonusType::BombCount => max_bomb_count.max += 1,
                    BonusType::Negative => {
                        if rng.0.random_bool(0.5) {
                            commands
                                .entity(character_entity)
                                .insert(MovementMultiplier::new(Duration::from_secs(10), 0.5));
                        } else {
                            commands.trigger(SpeedUpEnemies);
                        }
                    }
                    _ => {}
                }
                commands.trigger(EffectKind::PickUp);
                map.remove_bonus(bonus_pos);
            }
        }
    }
}

pub fn tick_temporal_bonuses(
    mut commands: Commands,
    mut players: Query<(Entity, &mut MovementMultiplier), With<Character>>,
    time: Res<Time<Fixed>>,
) {
    for (entity, mut movement_multiplier) in players.iter_mut() {
        movement_multiplier.timer.tick(time.delta());
        if movement_multiplier.timer.is_finished() {
            commands.entity(entity).remove::<MovementMultiplier>();
        }
    }
}
