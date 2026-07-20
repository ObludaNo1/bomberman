mod bonuses;

use bevy::prelude::*;
pub use bonuses::*;

#[derive(Component)]
pub struct Character;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Enemy {
    Zombie,
    Ghost,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_vec2(&self) -> Vec2 {
        match self {
            Direction::Up => Vec2::new(0.0, 1.0),
            Direction::Down => Vec2::new(0.0, -1.0),
            Direction::Left => Vec2::new(-1.0, 0.0),
            Direction::Right => Vec2::new(1.0, 0.0),
        }
    }

    pub fn from_vec2(v: Vec2) -> Option<Self> {
        if v == Vec2::ZERO {
            None
        } else if v.x.abs() > v.y.abs() {
            if v.x > 0.0 {
                Some(Direction::Right)
            } else {
                Some(Direction::Left)
            }
        } else {
            if v.y > 0.0 {
                Some(Direction::Up)
            } else {
                Some(Direction::Down)
            }
        }
    }
}

#[derive(Component)]
pub struct Bomb;

#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct MarkToDespawn;

#[derive(Component)]
pub struct ExplosionNeedsSetup;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplosionOrientation {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplosionVariant {
    Center,
    Straight(ExplosionOrientation),
    End(ExplosionOrientation),
}

#[derive(Component)]
pub struct Bonus;

#[derive(Component)]
pub struct InGameEntity;

#[derive(Component)]
pub struct Killable;

#[derive(Component)]
pub struct DestructibleWall;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BonusType {
    Range,
    BombCount,
    Negative,
    ExtraLife,
    Hook,
    BombKick,
    Detonator,
    Turbo,
    LineBomb,
    DoubleBomb,
    Max,
}

#[derive(Component, Deref, DerefMut, Debug, Clone, Copy, PartialEq)]
pub struct MovementSpeed(pub f32);

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum ActorState {
    Alive,
    Dying(Timer),
    // Exclusively for players
    Victory(Timer),
}

#[derive(Event)]
pub struct AllEnemiesKilledEvent;

#[derive(Resource)]
pub struct AllEnemiesKilled;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameplaySet {
    Controls,
    Movement,
    BombPlacement,
    MapTickUpdate,
    DeathAndVictory,
    MapToVisualsSync,
    AnimationAndSound,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum SpawnSystemSet {
    CreateMap,
    SpawnEnemies,
}
