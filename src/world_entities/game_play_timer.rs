use std::time::Duration;

use bevy::prelude::*;

use crate::{
    constants::{
        DEFAULT_GAMEPLAY_TIMER, GAMEPLAY_OVERTIME_TIMER, OVERTIME_PENALTY_SPEED_MULTIPLIER,
    },
    world_entities::GamePlayTimer,
};

impl GamePlayTimer {
    pub fn new() -> Self {
        GamePlayTimer(GamePlayTimerInner {
            kind: GamePlayTimerType::Basic,
            timer: Timer::new(DEFAULT_GAMEPLAY_TIMER, TimerMode::Once),
            overtimes: 0,
            overtime_this_tick: false,
        })
    }

    pub fn tick(&mut self, delta: Duration) {
        self.0.timer.tick(delta);
        self.0.overtime_this_tick = false;
        let finished_times = self.0.timer.times_finished_this_tick();
        if finished_times > 0 && self.0.kind == GamePlayTimerType::Basic {
            self.0.kind = GamePlayTimerType::Overtime;
            self.0.timer = Timer::new(GAMEPLAY_OVERTIME_TIMER, TimerMode::Repeating);
            self.0.overtime_this_tick = true;
        }
        self.0.overtimes += finished_times;
    }

    pub fn text(&self) -> String {
        match self.0.kind {
            GamePlayTimerType::Overtime => "Overtime! ".to_string(),
            GamePlayTimerType::Basic => {
                let remaining_time = self.0.timer.remaining_secs();
                let minutes = (remaining_time / 60.0).floor() as u32;
                let seconds = (remaining_time % 60.0).floor() as u32;
                format!("{:02}:{:02}", minutes, seconds)
            }
        }
    }

    pub fn enemy_speed_multiplier(&self) -> f32 {
        1.0 + self.0.overtimes as f32 * OVERTIME_PENALTY_SPEED_MULTIPLIER
    }

    pub fn overtime_duration(&self) -> Duration {
        if self.0.kind == GamePlayTimerType::Overtime {
            self.0.timer.elapsed() + self.0.timer.duration() * self.0.overtimes
        } else {
            Duration::ZERO
        }
    }

    pub fn turned_overtime_this_tick(&self) -> bool {
        self.0.overtime_this_tick
    }
}

impl Default for GamePlayTimer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GamePlayTimerType {
    Basic,
    Overtime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GamePlayTimerInner {
    kind: GamePlayTimerType,
    timer: Timer,
    overtimes: u32,
    overtime_this_tick: bool,
}
