use bevy::prelude::*;

use crate::{assets::TILESET_TILE_SIZE, bomb::BombTiming, world_entities::Bomb};

pub fn animate_bomb(mut query: Query<(&BombTiming, &mut Sprite), With<Bomb>>) {
    for (bomb_timing, mut sprite) in query.iter_mut() {
        let scale: f32 = if bomb_timing.is_on_final_tick() {
            1.1
        } else if bomb_timing.bomb_ticks % 2 == 0 {
            1.0
        } else {
            0.9
        };
        let scale = Vec2::new(
            TILESET_TILE_SIZE.x as f32 * scale,
            TILESET_TILE_SIZE.x as f32 * scale,
        );

        sprite.custom_size = Some(scale);
    }
}
