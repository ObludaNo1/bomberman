use bevy::prelude::*;

use crate::{
    animation::{
        ANIMATION_FRAME_COUNT, AnimationController, AnimationRenderFrame, MovementDirection,
        get_death_frame,
    },
    assets::{
        enemy_tileset::{self, EnemyTileType},
        material::ColouringMaterial,
    },
    death::DeathTimer,
    world_entities::{Direction, Enemy},
};

const ENEMY_ANIMATION_FRAMES_MOVING_DOWN: [AnimationRenderFrame<EnemyTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(EnemyTileType::ZombieDown2, false),
    AnimationRenderFrame::new(EnemyTileType::ZombieDown1, false),
    AnimationRenderFrame::new(EnemyTileType::ZombieDown2, false),
    AnimationRenderFrame::new(EnemyTileType::ZombieDown3, false),
];

const ENEMY_ANIMATION_FRAMES_MOVING_UP: [AnimationRenderFrame<EnemyTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(EnemyTileType::ZombieUp2, false),
    AnimationRenderFrame::new(EnemyTileType::ZombieUp1, false),
    AnimationRenderFrame::new(EnemyTileType::ZombieUp2, false),
    AnimationRenderFrame::new(EnemyTileType::ZombieUp3, false),
];

const ENEMY_ANIMATION_FRAMES_MOVING_LEFT: [AnimationRenderFrame<EnemyTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(EnemyTileType::ZombieLeft2, false),
    AnimationRenderFrame::new(EnemyTileType::ZombieLeft1, false),
    AnimationRenderFrame::new(EnemyTileType::ZombieLeft2, false),
    AnimationRenderFrame::new(EnemyTileType::ZombieLeft3, false),
];

const ENEMY_ANIMATION_FRAMES_MOVING_RIGHT: [AnimationRenderFrame<EnemyTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(EnemyTileType::ZombieLeft2, true),
    AnimationRenderFrame::new(EnemyTileType::ZombieLeft1, true),
    AnimationRenderFrame::new(EnemyTileType::ZombieLeft2, true),
    AnimationRenderFrame::new(EnemyTileType::ZombieLeft3, true),
];

pub fn get_enemy_animation_frames(
    direction: Direction,
) -> &'static [crate::animation::AnimationRenderFrame<EnemyTileType>;
             crate::animation::ANIMATION_FRAME_COUNT] {
    match direction {
        Direction::Up => &ENEMY_ANIMATION_FRAMES_MOVING_UP,
        Direction::Down => &ENEMY_ANIMATION_FRAMES_MOVING_DOWN,
        Direction::Left => &ENEMY_ANIMATION_FRAMES_MOVING_LEFT,
        Direction::Right => &ENEMY_ANIMATION_FRAMES_MOVING_RIGHT,
    }
}

const ENEMY_DEATH_ANIMATION_FRAME_COUNT: usize = 5;

/// Animation frame and its weight for death animation.
const DEATH_ANIMATION_FRAMES: [(AnimationRenderFrame<EnemyTileType>, u32);
    ENEMY_DEATH_ANIMATION_FRAME_COUNT] = [
    (
        AnimationRenderFrame::new(EnemyTileType::ZombieDown2, false),
        2,
    ),
    (
        AnimationRenderFrame::new(EnemyTileType::ZombieDeath1, false),
        2,
    ),
    (
        AnimationRenderFrame::new(EnemyTileType::ZombieDeath2, false),
        1,
    ),
    (
        AnimationRenderFrame::new(EnemyTileType::ZombieDeath3, false),
        1,
    ),
    (
        AnimationRenderFrame::new(EnemyTileType::ZombieDeath4, false),
        1,
    ),
];

pub fn animate_enemies(
    mut query: Query<
        (
            &mut AnimationController<EnemyTileType>,
            &MovementDirection,
            &MeshMaterial2d<ColouringMaterial>,
            Option<&DeathTimer>,
        ),
        With<Enemy>,
    >,
    mut materials: ResMut<Assets<ColouringMaterial>>,
    time: Res<Time>,
) {
    let delta_time = time.delta();
    for (mut animation_controller, movement_direction, material_handle, death_timer) in
        query.iter_mut()
    {
        let frame = if let Some(death_timer) = death_timer {
            get_death_frame(death_timer, &DEATH_ANIMATION_FRAMES)
        } else {
            animation_controller.update(delta_time, *movement_direction);
            animation_controller.current_frame()
        };
        if let Some(material) = materials.get_mut(&material_handle.0) {
            // Material2d carries per-frame UV/flip uniforms, so animation only updates
            // the current atlas rect and mirror flag.
            material.set_uv_rect(enemy_tileset::TILEMAP.sprite_uv_rect(*frame.tile()));
            material.set_flip_x(frame.flip_x());
        }
    }
}
