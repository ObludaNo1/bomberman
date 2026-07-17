use std::time::Duration;

use bevy::prelude::*;
use rand::RngExt;

use crate::{
    character::CharacterRng,
    map::{MapTileSetter, WorldMap},
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
    bonuses: Query<(Entity, &WorldPosition, &BonusType), Without<Character>>,
    mut map: ResMut<WorldMap>,
    mut rng: ResMut<CharacterRng>,
) {
    for (character_entity, pos, mut bomb_range, mut max_bomb_count) in players.iter_mut() {
        // There should not be that many bonuses where linearly searching is expensive
        for (bonus_entity, bonus_pos, bonus_type) in bonuses {
            if pos.0.distance(bonus_pos.0) < PICKUP_DISTANCE {
                match bonus_type {
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
                commands.entity(bonus_entity).despawn();
                let pos = map.get_position_from_world(bonus_pos);
                map.set_tile(pos.0, pos.1, MapTileSetter::PickupBonus);
                commands.trigger(EffectKind::PickUp);
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
