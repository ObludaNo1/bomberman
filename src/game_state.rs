mod main_menu;

use bevy::prelude::*;

use crate::game_state::main_menu::{
    clear_in_game_entities, despawn_main_menu, handle_main_menu_buttons, handle_main_menu_hover,
    spawn_main_menu,
};

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::MainMenu)
            .add_systems(
                OnEnter(GameState::MainMenu),
                (spawn_main_menu, clear_in_game_entities),
            )
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
            .add_systems(
                Update,
                (handle_main_menu_buttons, handle_main_menu_hover)
                    .run_if(in_state(GameState::MainMenu)),
            );
    }
}
