use std::time::Duration;

use bevy::prelude::*;

use crate::{
    assets::character_tileset::CharacterTileType, character::MovementDirection,
    world_entities::Character,
};

const ANIMATION_FRAME_DURATION: f32 = 0.1;

pub struct CharacterRenderFrame {
    tile: CharacterTileType,
    flip_x: bool,
}

const CHARACTER_ANIMATION_FRAMES: [CharacterRenderFrame; 4] = [
    CharacterRenderFrame {
        tile: CharacterTileType::Standing,
        flip_x: false,
    },
    CharacterRenderFrame {
        tile: CharacterTileType::Moving,
        flip_x: false,
    },
    CharacterRenderFrame {
        tile: CharacterTileType::Standing,
        flip_x: false,
    },
    CharacterRenderFrame {
        tile: CharacterTileType::Moving,
        flip_x: true,
    },
];

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct AnimationController {
    timer: Timer,
    current_frame_index: usize,
    moving: bool,
    was_moving: bool,
}

impl Default for AnimationController {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ANIMATION_FRAME_DURATION, TimerMode::Repeating),
            current_frame_index: 0,
            moving: false,
            was_moving: false,
        }
    }
}

impl AnimationController {
    pub fn update(&mut self, delta_time: Duration, direction: MovementDirection) {
        self.moving = direction.is_some();
        if self.moving {
            if !self.was_moving {
                self.current_frame_index = 1;
                self.was_moving = true;
            }

            self.timer.tick(delta_time);
            let times_finished = self.timer.times_finished_this_tick();
            if times_finished > 0 {
                self.current_frame_index = (self.current_frame_index + times_finished as usize)
                    % CHARACTER_ANIMATION_FRAMES.len();
            }
        } else {
            self.current_frame_index = 0;
            self.timer.reset();
            self.was_moving = false;
        }
    }

    pub fn current_frame(&self) -> &CharacterRenderFrame {
        &CHARACTER_ANIMATION_FRAMES[self.current_frame_index]
    }
}

pub fn animate_character(
    mut query: Query<(&mut AnimationController, &mut Sprite, &MovementDirection), With<Character>>,
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
