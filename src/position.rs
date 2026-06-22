use bevy::prelude::*;

/// A component representing a position in the world space. This is independent of the `Transform`
/// component and is used for game logic, such as movement and collision detection. The `Transform`
/// component is used for rendering and may not always reflect the actual position in the game
/// world.
#[repr(transparent)]
#[derive(Component, Debug, Deref, DerefMut, Clone, Copy, PartialEq)]
pub struct WorldPosition(pub Vec2);

impl From<Vec2> for WorldPosition {
    fn from(value: Vec2) -> Self {
        WorldPosition(value)
    }
}
