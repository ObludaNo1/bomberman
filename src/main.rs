mod camera;

use bevy::{
    prelude::*,
    window::{WindowMode, WindowPlugin},
};

use crate::camera::MainCameraPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bomberman".to_string(),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(MainCameraPlugin)
        .run();
}
