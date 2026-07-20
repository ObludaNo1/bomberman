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
    world_entities::{ActorState, Direction, Enemy},
};

const ZOMBIE_ANIMATION_FRAMES_MOVING_DOWN: [AnimationRenderFrame<EnemyTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(EnemyTileType::ZombieDown2, false),
    AnimationRenderFrame::new(EnemyTileType::ZombieDown1, false),
    AnimationRenderFrame::new(EnemyTileType::ZombieDown2, false),
    AnimationRenderFrame::new(EnemyTileType::ZombieDown3, false),
];

const ZOMBIE_ANIMATION_FRAMES_MOVING_UP: [AnimationRenderFrame<EnemyTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(EnemyTileType::ZombieUp2, false),
    AnimationRenderFrame::new(EnemyTileType::ZombieUp1, false),
    AnimationRenderFrame::new(EnemyTileType::ZombieUp2, false),
    AnimationRenderFrame::new(EnemyTileType::ZombieUp3, false),
];

const ZOMBIE_ANIMATION_FRAMES_MOVING_LEFT: [AnimationRenderFrame<EnemyTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(EnemyTileType::ZombieLeft2, false),
    AnimationRenderFrame::new(EnemyTileType::ZombieLeft1, false),
    AnimationRenderFrame::new(EnemyTileType::ZombieLeft2, false),
    AnimationRenderFrame::new(EnemyTileType::ZombieLeft3, false),
];

const ZOMBIE_ANIMATION_FRAMES_MOVING_RIGHT: [AnimationRenderFrame<EnemyTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(EnemyTileType::ZombieLeft2, true),
    AnimationRenderFrame::new(EnemyTileType::ZombieLeft1, true),
    AnimationRenderFrame::new(EnemyTileType::ZombieLeft2, true),
    AnimationRenderFrame::new(EnemyTileType::ZombieLeft3, true),
];

const GHOST_ANIMATION_FRAMES_MOVING_DOWN: [AnimationRenderFrame<EnemyTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(EnemyTileType::GhostDown2, false),
    AnimationRenderFrame::new(EnemyTileType::GhostDown1, false),
    AnimationRenderFrame::new(EnemyTileType::GhostDown2, false),
    AnimationRenderFrame::new(EnemyTileType::GhostDown3, false),
];

const GHOST_ANIMATION_FRAMES_MOVING_UP: [AnimationRenderFrame<EnemyTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(EnemyTileType::GhostUp2, false),
    AnimationRenderFrame::new(EnemyTileType::GhostUp1, false),
    AnimationRenderFrame::new(EnemyTileType::GhostUp2, false),
    AnimationRenderFrame::new(EnemyTileType::GhostUp3, false),
];

const GHOST_ANIMATION_FRAMES_MOVING_LEFT: [AnimationRenderFrame<EnemyTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(EnemyTileType::GhostLeft2, false),
    AnimationRenderFrame::new(EnemyTileType::GhostLeft1, false),
    AnimationRenderFrame::new(EnemyTileType::GhostLeft2, false),
    AnimationRenderFrame::new(EnemyTileType::GhostLeft3, false),
];

const GHOST_ANIMATION_FRAMES_MOVING_RIGHT: [AnimationRenderFrame<EnemyTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(EnemyTileType::GhostLeft2, true),
    AnimationRenderFrame::new(EnemyTileType::GhostLeft1, true),
    AnimationRenderFrame::new(EnemyTileType::GhostLeft2, true),
    AnimationRenderFrame::new(EnemyTileType::GhostLeft3, true),
];

const HOODIE_ANIMATION_FRAMES_MOVING_DOWN: [AnimationRenderFrame<EnemyTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(EnemyTileType::HoodieDown2, false),
    AnimationRenderFrame::new(EnemyTileType::HoodieDown1, false),
    AnimationRenderFrame::new(EnemyTileType::HoodieDown2, false),
    AnimationRenderFrame::new(EnemyTileType::HoodieDown3, false),
];

const HOODIE_ANIMATION_FRAMES_MOVING_UP: [AnimationRenderFrame<EnemyTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(EnemyTileType::HoodieUp2, false),
    AnimationRenderFrame::new(EnemyTileType::HoodieUp1, false),
    AnimationRenderFrame::new(EnemyTileType::HoodieUp2, false),
    AnimationRenderFrame::new(EnemyTileType::HoodieUp3, false),
];

const HOODIE_ANIMATION_FRAMES_MOVING_LEFT: [AnimationRenderFrame<EnemyTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(EnemyTileType::HoodieLeft2, false),
    AnimationRenderFrame::new(EnemyTileType::HoodieLeft1, false),
    AnimationRenderFrame::new(EnemyTileType::HoodieLeft2, false),
    AnimationRenderFrame::new(EnemyTileType::HoodieLeft3, false),
];

const HOODIE_ANIMATION_FRAMES_MOVING_RIGHT: [AnimationRenderFrame<EnemyTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(EnemyTileType::HoodieLeft2, true),
    AnimationRenderFrame::new(EnemyTileType::HoodieLeft1, true),
    AnimationRenderFrame::new(EnemyTileType::HoodieLeft2, true),
    AnimationRenderFrame::new(EnemyTileType::HoodieLeft3, true),
];

pub fn get_zombie_animation_frames(
    direction: Direction,
) -> &'static [AnimationRenderFrame<EnemyTileType>; ANIMATION_FRAME_COUNT] {
    match direction {
        Direction::Up => &ZOMBIE_ANIMATION_FRAMES_MOVING_UP,
        Direction::Down => &ZOMBIE_ANIMATION_FRAMES_MOVING_DOWN,
        Direction::Left => &ZOMBIE_ANIMATION_FRAMES_MOVING_LEFT,
        Direction::Right => &ZOMBIE_ANIMATION_FRAMES_MOVING_RIGHT,
    }
}

pub fn get_ghost_animation_frames(
    direction: Direction,
) -> &'static [AnimationRenderFrame<EnemyTileType>; ANIMATION_FRAME_COUNT] {
    match direction {
        Direction::Up => &GHOST_ANIMATION_FRAMES_MOVING_UP,
        Direction::Down => &GHOST_ANIMATION_FRAMES_MOVING_DOWN,
        Direction::Left => &GHOST_ANIMATION_FRAMES_MOVING_LEFT,
        Direction::Right => &GHOST_ANIMATION_FRAMES_MOVING_RIGHT,
    }
}

pub fn get_hoodie_animation_frames(
    direction: Direction,
) -> &'static [AnimationRenderFrame<EnemyTileType>; ANIMATION_FRAME_COUNT] {
    match direction {
        Direction::Up => &HOODIE_ANIMATION_FRAMES_MOVING_UP,
        Direction::Down => &HOODIE_ANIMATION_FRAMES_MOVING_DOWN,
        Direction::Left => &HOODIE_ANIMATION_FRAMES_MOVING_LEFT,
        Direction::Right => &HOODIE_ANIMATION_FRAMES_MOVING_RIGHT,
    }
}

const ZOMBIE_DEATH_ANIMATION_FRAME_COUNT: usize = 5;

/// Animation frame and its weight for death animation.
const ZOMBIE_DEATH_ANIMATION_FRAMES: [(AnimationRenderFrame<EnemyTileType>, u32);
    ZOMBIE_DEATH_ANIMATION_FRAME_COUNT] = [
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

const GHOST_DEATH_ANIMATION_FRAME_COUNT: usize = 5;
const GHOST_DEATH_ANIMATION_FRAMES: [(AnimationRenderFrame<EnemyTileType>, u32);
    GHOST_DEATH_ANIMATION_FRAME_COUNT] = [
    (
        AnimationRenderFrame::new(EnemyTileType::GhostDown2, false),
        2,
    ),
    (
        AnimationRenderFrame::new(EnemyTileType::GhostDeath1, false),
        2,
    ),
    (
        AnimationRenderFrame::new(EnemyTileType::GhostDeath2, false),
        1,
    ),
    (
        AnimationRenderFrame::new(EnemyTileType::GhostDeath3, false),
        1,
    ),
    (
        AnimationRenderFrame::new(EnemyTileType::GhostDeath4, false),
        1,
    ),
];

const HOODIE_DEATH_ANIMATION_FRAME_COUNT: usize = 4;
const HOODIE_DEATH_ANIMATION_FRAMES: [(AnimationRenderFrame<EnemyTileType>, u32);
    HOODIE_DEATH_ANIMATION_FRAME_COUNT] = [
    (
        AnimationRenderFrame::new(EnemyTileType::HoodieDown2, false),
        2,
    ),
    (
        AnimationRenderFrame::new(EnemyTileType::HoodieDeath1, false),
        2,
    ),
    (
        AnimationRenderFrame::new(EnemyTileType::HoodieDeath2, false),
        1,
    ),
    (
        AnimationRenderFrame::new(EnemyTileType::HoodieDeath3, false),
        1,
    ),
];

pub fn animate_enemies(
    mut query: Query<(
        &Enemy,
        &mut AnimationController<EnemyTileType>,
        &MovementDirection,
        &MeshMaterial2d<ColouringMaterial>,
        &ActorState,
    )>,
    mut materials: ResMut<Assets<ColouringMaterial>>,
    time: Res<Time>,
) {
    let delta_time = time.delta();
    for (enemy, mut animation_controller, movement_direction, material_handle, death_timer) in
        query.iter_mut()
    {
        let frame = if let ActorState::Dying(death_timer) = death_timer {
            let death_animation_frames: &'static [(AnimationRenderFrame<EnemyTileType>, u32)] =
                match enemy {
                    Enemy::Zombie => &ZOMBIE_DEATH_ANIMATION_FRAMES,
                    Enemy::Ghost => &GHOST_DEATH_ANIMATION_FRAMES,
                    Enemy::Hoodie => &HOODIE_DEATH_ANIMATION_FRAMES,
                };
            get_death_frame(death_timer, death_animation_frames)
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
