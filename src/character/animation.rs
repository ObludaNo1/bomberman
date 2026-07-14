use bevy::prelude::*;

use crate::{
    animation::{
        ANIMATION_FRAME_COUNT, AnimationController, AnimationRenderFrame, MovementDirection,
        get_death_frame,
    },
    assets::{
        character_tileset::{self, CharacterTileType},
        material::ColouringMaterial,
    },
    death::DeathTimer,
    world_entities::{Character, Direction},
};

const CHARACTER_ANIMATION_FRAMES_MOVING_DOWN: [AnimationRenderFrame<CharacterTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(CharacterTileType::StandingDown, false),
    AnimationRenderFrame::new(CharacterTileType::MovingDown, false),
    AnimationRenderFrame::new(CharacterTileType::StandingDown, false),
    AnimationRenderFrame::new(CharacterTileType::MovingDown, true),
];

const CHARACTER_ANIMATION_FRAMES_MOVING_UP: [AnimationRenderFrame<CharacterTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(CharacterTileType::StandingUp, false),
    AnimationRenderFrame::new(CharacterTileType::MovingUp, false),
    AnimationRenderFrame::new(CharacterTileType::StandingUp, false),
    AnimationRenderFrame::new(CharacterTileType::MovingUp, true),
];

const CHARACTER_ANIMATION_FRAMES_MOVING_RIGHT: [AnimationRenderFrame<CharacterTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(CharacterTileType::StandingRight, false),
    AnimationRenderFrame::new(CharacterTileType::MovingRight2, false),
    AnimationRenderFrame::new(CharacterTileType::MovingRight1, false),
    AnimationRenderFrame::new(CharacterTileType::MovingRight2, false),
];

const CHARACTER_ANIMATION_FRAMES_MOVING_LEFT: [AnimationRenderFrame<CharacterTileType>;
    ANIMATION_FRAME_COUNT] = [
    AnimationRenderFrame::new(CharacterTileType::StandingRight, true),
    AnimationRenderFrame::new(CharacterTileType::MovingRight2, true),
    AnimationRenderFrame::new(CharacterTileType::MovingRight1, true),
    AnimationRenderFrame::new(CharacterTileType::MovingRight2, true),
];

pub fn get_character_animation_frames(
    direction: Direction,
) -> &'static [AnimationRenderFrame<CharacterTileType>; ANIMATION_FRAME_COUNT] {
    match direction {
        Direction::Up => &CHARACTER_ANIMATION_FRAMES_MOVING_UP,
        Direction::Down => &CHARACTER_ANIMATION_FRAMES_MOVING_DOWN,
        Direction::Left => &CHARACTER_ANIMATION_FRAMES_MOVING_LEFT,
        Direction::Right => &CHARACTER_ANIMATION_FRAMES_MOVING_RIGHT,
    }
}

const PLAYER_DEATH_ANIMATION_FRAME_COUNT: usize = 6;

/// Animation frame and its weight for death animation.
const DEATH_ANIMATION_FRAMES: [(AnimationRenderFrame<CharacterTileType>, u32);
    PLAYER_DEATH_ANIMATION_FRAME_COUNT] = [
    (
        AnimationRenderFrame::new(CharacterTileType::StandingDown, false),
        4,
    ),
    (
        AnimationRenderFrame::new(CharacterTileType::Death1, false),
        1,
    ),
    (
        AnimationRenderFrame::new(CharacterTileType::Death2, false),
        1,
    ),
    (
        AnimationRenderFrame::new(CharacterTileType::Death3, false),
        1,
    ),
    (
        AnimationRenderFrame::new(CharacterTileType::Death4, false),
        1,
    ),
    (
        AnimationRenderFrame::new(CharacterTileType::Empty, false),
        3,
    ),
];

pub fn animate_character(
    mut query: Query<
        (
            &mut AnimationController<CharacterTileType>,
            &MovementDirection,
            &MeshMaterial2d<ColouringMaterial>,
            Option<&DeathTimer>,
        ),
        With<Character>,
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

            material.set_uv_rect(character_tileset::TILEMAP.sprite_uv_rect(*frame.tile()));
            material.set_flip_x(frame.flip_x());
        }
    }
}
