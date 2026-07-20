use std::time::Duration;

pub const TOTAL_MAP_WIDTH: usize = 19;
pub const TOTAL_MAP_HEIGHT: usize = 15;
pub const WALL_DENSITY: f32 = 0.60;

pub const BOMB_DURATION: Duration = Duration::from_millis(4500);
pub const BOMB_EXPLOSION_DURATION: Duration = Duration::from_millis(1000);
pub const WALL_BREAK_DURATION: Duration = BOMB_EXPLOSION_DURATION;

pub const ENEMIES_SPAWNED: usize = 5;
