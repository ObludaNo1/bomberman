use std::time::Duration;

use bevy::prelude::*;

use crate::{
    assets::character_tileset::CharacterTileType, character::MovementDirection,
    controls::Direction, world_entities::Character,
};

const ANIMATION_FRAME_DURATION: f32 = 0.1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterRenderFrame {
    tile: CharacterTileType,
    flip_x: bool,
}

const ANIMATION_FRAME_COUNT: usize = 4;

const CHARACTER_ANIMATION_FRAMES_MOVING_DOWN: [CharacterRenderFrame; ANIMATION_FRAME_COUNT] = [
    CharacterRenderFrame {
        tile: CharacterTileType::StandingDown,
        flip_x: false,
    },
    CharacterRenderFrame {
        tile: CharacterTileType::MovingDown,
        flip_x: false,
    },
    CharacterRenderFrame {
        tile: CharacterTileType::StandingDown,
        flip_x: false,
    },
    CharacterRenderFrame {
        tile: CharacterTileType::MovingDown,
        flip_x: true,
    },
];

const CHARACTER_ANIMATION_FRAMES_MOVING_UP: [CharacterRenderFrame; ANIMATION_FRAME_COUNT] = [
    CharacterRenderFrame {
        tile: CharacterTileType::StandingUp,
        flip_x: false,
    },
    CharacterRenderFrame {
        tile: CharacterTileType::MovingUp,
        flip_x: false,
    },
    CharacterRenderFrame {
        tile: CharacterTileType::StandingUp,
        flip_x: false,
    },
    CharacterRenderFrame {
        tile: CharacterTileType::MovingUp,
        flip_x: true,
    },
];

const CHARACTER_ANIMATION_FRAMES_MOVING_RIGHT: [CharacterRenderFrame; ANIMATION_FRAME_COUNT] = [
    CharacterRenderFrame {
        tile: CharacterTileType::StandingRight,
        flip_x: false,
    },
    CharacterRenderFrame {
        tile: CharacterTileType::MovingRight2,
        flip_x: false,
    },
    CharacterRenderFrame {
        tile: CharacterTileType::MovingRight1,
        flip_x: false,
    },
    CharacterRenderFrame {
        tile: CharacterTileType::MovingRight2,
        flip_x: false,
    },
];

const CHARACTER_ANIMATION_FRAMES_MOVING_LEFT: [CharacterRenderFrame; ANIMATION_FRAME_COUNT] = [
    CharacterRenderFrame {
        tile: CharacterTileType::StandingRight,
        flip_x: true,
    },
    CharacterRenderFrame {
        tile: CharacterTileType::MovingRight2,
        flip_x: true,
    },
    CharacterRenderFrame {
        tile: CharacterTileType::MovingRight1,
        flip_x: true,
    },
    CharacterRenderFrame {
        tile: CharacterTileType::MovingRight2,
        flip_x: true,
    },
];

fn get_animation_frames_for_direction(
    direction: Direction,
) -> &'static [CharacterRenderFrame; ANIMATION_FRAME_COUNT] {
    match direction {
        Direction::Up => &CHARACTER_ANIMATION_FRAMES_MOVING_UP,
        Direction::Down => &CHARACTER_ANIMATION_FRAMES_MOVING_DOWN,
        Direction::Left => &CHARACTER_ANIMATION_FRAMES_MOVING_LEFT,
        Direction::Right => &CHARACTER_ANIMATION_FRAMES_MOVING_RIGHT,
    }
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct CharacterAnimationController {
    timer: Timer,
    current_animation_frames: &'static [CharacterRenderFrame; ANIMATION_FRAME_COUNT],
    current_frame_index: usize,
    last_moving: MovementDirection,
}

impl Default for CharacterAnimationController {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ANIMATION_FRAME_DURATION, TimerMode::Repeating),
            current_animation_frames: &CHARACTER_ANIMATION_FRAMES_MOVING_DOWN,
            current_frame_index: 0,
            last_moving: MovementDirection(None),
        }
    }
}

impl CharacterAnimationController {
    pub fn update(&mut self, delta_time: Duration, direction: MovementDirection) {
        if let Some(direction) = direction.0 {
            if self.last_moving.is_none() {
                // Advance animation so we always start by character taking a step as first
                // animation.
                self.current_frame_index = 1;
            }
            self.last_moving = MovementDirection(Some(direction));

            self.timer.tick(delta_time);
            let times_finished = self.timer.times_finished_this_tick();
            if times_finished > 0 {
                self.current_frame_index =
                    (self.current_frame_index + times_finished as usize) % ANIMATION_FRAME_COUNT;
            }

            self.current_animation_frames = get_animation_frames_for_direction(direction);
        } else {
            self.current_frame_index = 0;
            self.timer.reset();
            self.last_moving = MovementDirection(None);
        }
    }

    pub fn current_frame(&self) -> &CharacterRenderFrame {
        &self.current_animation_frames[self.current_frame_index]
    }
}

pub fn animate_character(
    mut query: Query<
        (
            &mut CharacterAnimationController,
            &mut Sprite,
            &MovementDirection,
        ),
        With<Character>,
    >,
    time: Res<Time>,
) {
    let delta_time = time.delta();
    for (mut animation_controller, mut sprite, movement_direction) in query.iter_mut() {
        animation_controller.update(delta_time, *movement_direction);
        let current_frame = animation_controller.current_frame();
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = current_frame.tile as usize;
        }
        sprite.flip_x = current_frame.flip_x;
    }
}
