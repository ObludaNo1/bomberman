use bevy::prelude::*;

use crate::{
    assets::TILESET_TILE_SIZE, character::spawn::Character, controls::Controls,
    position::WorldPosition, util::CameraScale,
};

pub const CHARACTER_SPEED: f32 = 2.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MovementDirection(i8);

impl MovementDirection {
    pub fn new(direction: impl Into<i8>) -> Self {
        Self(direction.into())
    }

    pub fn as_f32(&self) -> f32 {
        self.0 as f32
    }
}

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CharacterMovement {
    pub vertical: MovementDirection,
    pub horizontal: MovementDirection,
}

pub fn move_character(
    mut query: Query<(&mut Transform, &mut WorldPosition, &mut CharacterMovement), With<Character>>,
    controls: Res<Controls>,
    time: Res<Time>,
    scale: Res<CameraScale>,
) {
    let scale = scale.0;
    let elapsed = time.delta_secs();
    for (mut transform, mut world_position, mut movement) in query.iter_mut() {
        movement.vertical = MovementDirection::new(if controls.up {
            1
        } else if controls.down {
            -1
        } else {
            0
        });
        movement.horizontal = MovementDirection::new(if controls.left {
            -1
        } else if controls.right {
            1
        } else {
            0
        });

        world_position.x += movement.horizontal.as_f32() * CHARACTER_SPEED * elapsed;
        world_position.y += movement.vertical.as_f32() * CHARACTER_SPEED * elapsed;

        *transform = Transform::from_xyz(
            world_position.x * TILESET_TILE_SIZE.x as f32 * scale,
            world_position.y * TILESET_TILE_SIZE.y as f32 * scale,
            1.0,
        )
        .with_scale(Vec3::splat(scale));
    }
}
