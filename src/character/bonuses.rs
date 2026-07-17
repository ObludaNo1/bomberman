use bevy::prelude::*;

use crate::{
    map::{MapTileSetter, WorldMap},
    position::WorldPosition,
    world_entities::{BombCount, BombRange, BonusType, Character},
};

const PICKUP_DISTANCE: f32 = 0.75;

pub fn pick_up_bonuses(
    mut commands: Commands,
    mut players: Query<(&WorldPosition, &mut BombRange, &mut BombCount), With<Character>>,
    bonuses: Query<(Entity, &WorldPosition, &BonusType), Without<Character>>,
    mut map: ResMut<WorldMap>,
) {
    for (pos, mut bomb_range, mut max_bomb_count) in players.iter_mut() {
        // There should not be that many bonuses where linearly searching is expensive
        for (entity, bonus_pos, bonus_type) in bonuses {
            if pos.0.distance(bonus_pos.0) < PICKUP_DISTANCE {
                match bonus_type {
                    BonusType::Range => bomb_range.0 += 1,
                    BonusType::BombCount => max_bomb_count.max += 1,
                    _ => {}
                }
                commands.entity(entity).despawn();
                let pos = map.get_position_from_world(bonus_pos);
                map.set_tile(pos.0, pos.1, MapTileSetter::PickupBonus);
            }
        }
    }
}
