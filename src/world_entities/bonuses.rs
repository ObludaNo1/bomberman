use std::time::Duration;

use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BombRange(pub u32);

impl Default for BombRange {
    fn default() -> Self {
        BombRange(1)
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BombCount {
    pub current: u32,
    pub max: u32,
}

impl Default for BombCount {
    fn default() -> Self {
        BombCount { current: 0, max: 1 }
    }
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct MovementMultiplier {
    pub timer: Timer,
    pub multiplier: f32,
}

impl MovementMultiplier {
    pub fn new(duration: Duration, multiplier: f32) -> Self {
        MovementMultiplier {
            timer: Timer::new(duration, TimerMode::Once),
            multiplier,
        }
    }
}

#[derive(Event)]
pub struct SpeedUpEnemies;
