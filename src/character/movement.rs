use bevy::prelude::*;

use crate::{character::spawn::Character, controls::Controls, position::WorldPosition};

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

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CharacterMovement {
    pub vertical: MovementDirection,
    pub horizontal: MovementDirection,
}

pub fn move_character(
    mut query: Query<(&mut WorldPosition, &mut CharacterMovement), With<Character>>,
    controls: Res<Controls>,
    time: Res<Time>,
) {
    let elapsed = time.delta_secs();
    for (mut world_position, mut movement) in query.iter_mut() {
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

        let direction = if movement.horizontal.is_zero() && movement.vertical.is_zero() {
            Vec2::ZERO
        } else {
            Vec2::new(movement.horizontal.as_f32(), movement.vertical.as_f32()).normalize()
                * CHARACTER_SPEED
                * elapsed
        };
        world_position.x += direction.x;
        world_position.y += direction.y;
    }
}
