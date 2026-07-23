mod animation;
mod assets;
mod bomb;
mod character;
mod constants;
mod controls;
mod death;
mod enemy;
mod game_state;
mod map;
mod position;
mod rendering;
mod sound;
mod ui;
mod util;
mod world_entities;

use bevy::{
    prelude::*,
    window::{WindowMode, WindowPlugin},
};

use crate::{
    game_state::{GameState, PlayingState, STARTS_PLAYING},
    world_entities::{GameplaySet, SpawnEnemiesMessage, SpawnSystemSet},
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
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(bomb::BombPlugin)
        .add_plugins(game_state::GameStatePlugin)
        .add_plugins(death::DeathPlugin)
        .add_plugins(sound::SoundPlugin)
        .add_plugins(ui::GameUiPlugin)
        .add_message::<SpawnEnemiesMessage>()
        .configure_sets(
            PreUpdate,
            GameplaySet::Controls.run_if(in_state(GameState::Playing)),
        )
        .configure_sets(
            FixedUpdate,
            (
                GameplaySet::EnemySpawning,
                GameplaySet::Movement,
                GameplaySet::BombPlacement,
                GameplaySet::MapTickUpdate,
                GameplaySet::DeathAndVictory,
            )
                .chain()
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(PlayingState::Playing)),
        )
        .configure_sets(
            Update,
            GameplaySet::MapToVisualsSync
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(PlayingState::Playing)),
        )
        .configure_sets(
            PostUpdate,
            GameplaySet::AnimationAndSound.run_if(in_state(GameState::Playing)),
        )
        .configure_sets(
            STARTS_PLAYING,
            (SpawnSystemSet::CreateMap, SpawnSystemSet::SpawnUnits).chain(),
        )
        .run();
}
