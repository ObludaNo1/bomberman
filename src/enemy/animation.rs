use bevy::prelude::*;

use crate::{
    animation::{
        ANIMATION_FRAME_COUNT, AnimationController, AnimationRenderFrame, MovementDirection,
    },
    assets::{
        enemy_tileset::{self, EnemyTileType},
        material::ColouringMaterial,
    },
    enemy::Enemy,
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
    direction: crate::controls::Direction,
) -> &'static [crate::animation::AnimationRenderFrame<EnemyTileType>;
             crate::animation::ANIMATION_FRAME_COUNT] {
    match direction {
        crate::controls::Direction::Up => &ENEMY_ANIMATION_FRAMES_MOVING_UP,
        crate::controls::Direction::Down => &ENEMY_ANIMATION_FRAMES_MOVING_DOWN,
        crate::controls::Direction::Left => &ENEMY_ANIMATION_FRAMES_MOVING_LEFT,
        crate::controls::Direction::Right => &ENEMY_ANIMATION_FRAMES_MOVING_RIGHT,
    }
}

pub fn animate_enemies(
    mut query: Query<
        (
            &mut AnimationController<EnemyTileType>,
            &MovementDirection,
            &MeshMaterial2d<ColouringMaterial>,
        ),
        With<Enemy>,
    >,
    mut materials: ResMut<Assets<ColouringMaterial>>,
    time: Res<Time>,
) {
    let delta_time = time.delta();
    for (mut animation_controller, movement_direction, material_handle) in query.iter_mut() {
        animation_controller.update(delta_time, *movement_direction);
        let current_frame = animation_controller.current_frame();
        if let Some(material) = materials.get_mut(&material_handle.0) {
            // Material2d carries per-frame UV/flip uniforms, so animation only updates
            // the current atlas rect and mirror flag.
            material.set_uv_rect(enemy_tileset::TILEMAP.sprite_uv_rect(*current_frame.tile()));
            material.set_flip_x(current_frame.flip_x());
        }
    }
}
