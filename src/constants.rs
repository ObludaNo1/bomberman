use std::time::Duration;

pub const TOTAL_MAP_WIDTH: usize = 19;
pub const TOTAL_MAP_HEIGHT: usize = 15;
pub const WALL_DENSITY: f32 = 0.50;

pub const BOMB_DURATION: Duration = Duration::from_millis(4500);
pub const BOMB_EXPLOSION_DURATION: Duration = Duration::from_millis(1000);
pub const WALL_BREAK_DURATION: Duration = BOMB_EXPLOSION_DURATION;

pub const ZOMBIES_SPAWNED: usize = 4;
pub const ZOMBIE_SPEED: f32 = 1.2;
pub const GHOSTS_SPAWNED: usize = 2;
pub const GHOST_SPEED: f32 = 2.4;
pub const HOODIES_SPAWNED: usize = 2;
pub const HOODIE_SPEED: f32 = 1.8;

pub const TOP_MENU_BAR_HEIGHT: f32 = 100.0;
