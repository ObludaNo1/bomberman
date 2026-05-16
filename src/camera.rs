use bevy::prelude::*;

/// Marker component for the main game camera.
#[derive(Component)]
struct MainCamera;

/// Spawn the main 2D camera.
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d::default(), MainCamera));
}

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}
