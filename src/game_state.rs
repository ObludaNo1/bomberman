mod main_menu;
mod winning_screen;

use bevy::prelude::*;

use crate::game_state::{
    main_menu::{
        clear_in_game_entities, despawn_main_menu, handle_main_menu_buttons,
        handle_main_menu_hover, spawn_main_menu,
    },
    winning_screen::{
        despawn_winning_screen, handle_win_screen_buttons, handle_win_screen_hover,
        spawn_winning_screen,
    },
};

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Win,
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
            )
            .add_systems(OnEnter(GameState::Win), spawn_winning_screen)
            .add_systems(OnExit(GameState::Win), despawn_winning_screen)
            .add_systems(
                Update,
                (handle_win_screen_buttons, handle_win_screen_hover)
                    .run_if(in_state(GameState::Win)),
            );
    }
}
