use bevy::prelude::*;

use crate::constants::{TOTAL_MAP_HEIGHT, TOTAL_MAP_WIDTH};

/// A component representing a position in the world space. This is independent of the `Transform`
/// component and is used for game logic, such as movement and collision detection. The `Transform`
/// component is used for rendering and may not always reflect the actual position in the game
/// world.
#[repr(transparent)]
#[derive(Component, Debug, Deref, DerefMut, Clone, Copy, PartialEq)]
pub struct WorldPosition(pub Vec2);

impl WorldPosition {
    pub fn to_closest_tile(&self) -> TilePosition {
        let x = (self.x + 0.5 + (TOTAL_MAP_WIDTH - 1) as f32 * 0.5) as u32;
        let y = (self.y + 0.5 + (TOTAL_MAP_HEIGHT - 1) as f32 * 0.5) as u32;
        TilePosition(UVec2 { x, y })
    }
}

impl From<Vec2> for WorldPosition {
    fn from(value: Vec2) -> Self {
        WorldPosition(value)
    }
}

impl From<WorldPosition> for Vec2 {
    fn from(value: WorldPosition) -> Self {
        value.0
    }
}

/// A component precisely representing a position in the tile grid.
#[repr(transparent)]
#[derive(Component, Debug, Deref, DerefMut, Clone, Copy, PartialEq, Eq)]
pub struct TilePosition(pub UVec2);

impl TilePosition {
    pub fn to_world_position(&self) -> WorldPosition {
        WorldPosition(Vec2 {
            x: self.x as f32 - (TOTAL_MAP_WIDTH - 1) as f32 * 0.5,
            y: self.y as f32 - (TOTAL_MAP_HEIGHT - 1) as f32 * 0.5,
        })
    }
}

impl From<UVec2> for TilePosition {
    fn from(value: UVec2) -> Self {
        TilePosition(value)
    }
}

impl From<TilePosition> for UVec2 {
    fn from(value: TilePosition) -> Self {
        value.0
    }
}
