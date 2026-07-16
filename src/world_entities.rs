use bevy::prelude::*;

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct Enemy;

// TODO this has become insufficient to describe state of tiles. Rewrite this into a struct or bit
// flags.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapTileMarker {
    Empty,
    Wall,
    WallWithExit,
    IndestructibleWall,
    Bomb,
    Explosion,
    ExplosionWithExit,
    Exit,
}

impl MapTileMarker {
    pub fn is_walkable(&self) -> bool {
        use MapTileMarker as M;
        match self {
            M::Empty => true,
            M::Explosion => true,
            M::ExplosionWithExit => true,
            M::Bomb => false,
            M::Wall => false,
            M::WallWithExit => false,
            M::IndestructibleWall => false,
            M::Exit => true,
        }
    }

    pub fn is_obstacle(&self) -> bool {
        !self.is_walkable()
    }
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
pub struct InGameEntity;

#[derive(Component)]
pub struct Killable;

#[derive(Component)]
pub struct DestructibleWall;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameplaySet {
    Controls,
    Movement,
    Bomb,
    Explosion,
    Death,
    Animation,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum SpawnSystemSet {
    CreateMap,
    SpawnEnemies,
}
