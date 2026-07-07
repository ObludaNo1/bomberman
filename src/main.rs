mod assets;
mod bomb;
mod character;
mod controls;
mod map;
mod position;
mod rendering;
mod util;
mod world_entities;

use bevy::{
    prelude::*,
    window::{WindowMode, WindowPlugin},
};

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
        .add_plugins(rendering::RenderingPlugin)
        .add_plugins(util::CameraScalePlugin)
        .add_plugins(map::Map)
        .add_plugins(controls::ControlsPlugin)
        .add_plugins(character::CharacterPlugin)
        .add_plugins(bomb::BombPlugin)
        .run();
}
