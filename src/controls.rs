use bevy::prelude::*;

#[derive(Resource, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Controls {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub place_bomb: bool,
}

fn map_controls_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut controls: ResMut<Controls>) {
    controls.up = keyboard_input.pressed(KeyCode::KeyW);
    controls.down = keyboard_input.pressed(KeyCode::KeyS);
    controls.left = keyboard_input.pressed(KeyCode::KeyA);
    controls.right = keyboard_input.pressed(KeyCode::KeyD);
    controls.place_bomb = keyboard_input.just_pressed(KeyCode::Space);
}

fn insert_default_controls(mut commands: Commands) {
    commands.insert_resource(Controls::default());
}

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, insert_default_controls)
            .add_systems(Update, map_controls_input);
    }
}
