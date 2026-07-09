use bevy::prelude::*;

use crate::world_entities::GameplaySet;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Resource, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Controls {
    movement_directions: Vec<Direction>,
    pub place_bomb: bool,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            movement_directions: Vec::with_capacity(4),
            place_bomb: false,
        }
    }
}

impl Controls {
    fn update_direction(&mut self, direction: Direction, is_pressed: bool) {
        if is_pressed {
            if !self.movement_directions.contains(&direction) {
                self.movement_directions.push(direction);
            }
        } else {
            self.movement_directions.retain(|&d| d != direction);
        }
    }

    pub fn into_movement(&self) -> Option<Direction> {
        self.movement_directions.last().copied()
    }
}

fn map_controls_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut controls: ResMut<Controls>) {
    controls.update_direction(Direction::Up, keyboard_input.pressed(KeyCode::KeyW));
    controls.update_direction(Direction::Down, keyboard_input.pressed(KeyCode::KeyS));
    controls.update_direction(Direction::Left, keyboard_input.pressed(KeyCode::KeyA));
    controls.update_direction(Direction::Right, keyboard_input.pressed(KeyCode::KeyD));
    controls.place_bomb = keyboard_input.just_pressed(KeyCode::Space);
}

fn insert_default_controls(mut commands: Commands) {
    commands.insert_resource(Controls::default());
}

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, insert_default_controls)
            .add_systems(Update, map_controls_input.in_set(GameplaySet::Controls));
    }
}
