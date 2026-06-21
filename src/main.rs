mod camera;

use bevy::{
    prelude::*,
    window::{WindowMode, WindowPlugin},
};

use crate::camera::MainCameraPlugin;

fn get_assets_path() -> String {
    // TODO make it dependent on build directory
    "assets".to_string()
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: get_assets_path().into(),
                    ..default()
                })
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
