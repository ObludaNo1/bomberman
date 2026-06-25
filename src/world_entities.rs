use bevy::prelude::*;

#[derive(Component)]
pub struct Character;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapTileMarker {
    Obstacle,
    Walkable,
}

#[derive(Component)]
pub struct Bomb;
