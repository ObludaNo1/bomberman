use std::time::Duration;

use bevy::prelude::*;

use crate::world_entities::Direction;

const ANIMATION_FRAME_DURATION: f32 = 0.1;

#[derive(Component, Deref, DerefMut, Debug, Clone, Copy, PartialEq, Eq)]
pub struct MovementDirection(pub Option<Direction>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnimationRenderFrame<TT> {
    tile: TT,
    flip_x: bool,
}

impl<TT> AnimationRenderFrame<TT> {
    pub const fn new(tile: TT, flip_x: bool) -> Self {
        Self { tile, flip_x }
    }

    pub fn tile(&self) -> &TT {
        &self.tile
    }

    pub fn flip_x(&self) -> bool {
        self.flip_x
    }
}

pub const ANIMATION_FRAME_COUNT: usize = 4;

#[derive(Component, Debug, Clone)]
pub struct AnimationController<TT: 'static> {
    timer: Timer,
    get_frames: fn(Direction) -> &'static [AnimationRenderFrame<TT>; ANIMATION_FRAME_COUNT],
    current_animation_frames: &'static [AnimationRenderFrame<TT>; ANIMATION_FRAME_COUNT],
    current_frame_index: usize,
    last_moving: MovementDirection,
}

impl<TT> AnimationController<TT> {
    pub fn new(
        get_frames: fn(Direction) -> &'static [AnimationRenderFrame<TT>; ANIMATION_FRAME_COUNT],
    ) -> Self {
        Self {
            timer: Timer::from_seconds(ANIMATION_FRAME_DURATION, TimerMode::Repeating),
            get_frames,
            current_animation_frames: get_frames(Direction::Down),
            current_frame_index: 0,
            last_moving: MovementDirection(None),
        }
    }

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

            self.current_animation_frames = (self.get_frames)(direction);
        } else {
            self.current_frame_index = 0;
            self.timer.reset();
            self.last_moving = MovementDirection(None);
        }
    }

    pub fn current_frame(&self) -> &AnimationRenderFrame<TT> {
        &self.current_animation_frames[self.current_frame_index]
    }
}
