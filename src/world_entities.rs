use bevy::prelude::*;

#[derive(Component)]
pub struct Character;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapTileMarker {
    Empty,
    Wall,
    IndestructibleWall,
    Bomb,
    Explosion,
}

impl MapTileMarker {
    pub fn is_walkable(&self) -> bool {
        use MapTileMarker as M;
        match self {
            M::Empty => true,
            M::Explosion => true,
            M::Bomb => false,
            M::Wall => false,
            M::IndestructibleWall => false,
        }
    }

    pub fn is_obstacle(&self) -> bool {
        !self.is_walkable()
    }
}

#[derive(Component)]
pub struct Bomb;

#[derive(Component)]
pub struct Explosion;
