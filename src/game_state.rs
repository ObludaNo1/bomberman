mod level_selection_menu;
mod main_menu;
mod pause_menu;
mod winning_screen;

use bevy::prelude::*;

use crate::{
    game_state::{
        level_selection_menu::{
            despawn_level_selection_menu, handle_level_selection_buttons,
            handle_level_selection_hover, spawn_level_selection_menu,
        },
        main_menu::{
            clear_in_game_entities, despawn_main_menu, handle_main_menu_buttons,
            handle_main_menu_hover, spawn_main_menu,
        },
        pause_menu::{despawn_pause_menu, pause_on_esc, resume_on_esc, spawn_pause_menu},
        winning_screen::{
            despawn_winning_screen, handle_win_screen_buttons, handle_win_screen_hover,
            spawn_winning_screen,
        },
    },
    sound::EffectKind,
    world_entities::{FontHandle, GamePlayTimer},
};

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
}

#[derive(SubStates, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[source(GameState = GameState::MainMenu)]
pub enum MenuState {
    #[default]
    Main,
    Levels,
}

#[derive(SubStates, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[source(GameState = GameState::Playing)]
pub enum PlayingState {
    #[default]
    Playing,
    Pause,
    Win,
}

fn setup_game_play_timer(mut commands: Commands) {
    commands.insert_resource(GamePlayTimer::new());
}

fn tick_game_play_timer(
    mut commands: Commands,
    mut game_play_timer: ResMut<GamePlayTimer>,
    time: Res<Time<Fixed>>,
) {
    game_play_timer.tick(time.delta());
    if game_play_timer.turned_overtime_this_tick() {
        commands.trigger(EffectKind::Overtime);
    }
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::MainMenu)
            .add_sub_state::<PlayingState>()
            .add_sub_state::<MenuState>()
            .init_resource::<FontHandle>()
            .insert_resource(GamePlayTimer::new())
            .add_systems(OnEnter(GameState::MainMenu), clear_in_game_entities)
            .add_systems(OnEnter(MenuState::Main), spawn_main_menu)
            .add_systems(OnExit(MenuState::Main), despawn_main_menu)
            .add_systems(
                Update,
                (handle_main_menu_buttons, handle_main_menu_hover)
                    .run_if(in_state(MenuState::Main)),
            )
            .add_systems(OnEnter(MenuState::Levels), spawn_level_selection_menu)
            .add_systems(OnExit(MenuState::Levels), despawn_level_selection_menu)
            .add_systems(
                Update,
                (handle_level_selection_buttons, handle_level_selection_hover)
                    .run_if(in_state(MenuState::Levels)),
            )
            .add_systems(OnEnter(GameState::Playing), setup_game_play_timer)
            .add_systems(
                FixedUpdate,
                tick_game_play_timer.run_if(in_state(PlayingState::Playing)),
            )
            .add_systems(Update, pause_on_esc.run_if(in_state(PlayingState::Playing)))
            .add_systems(OnEnter(PlayingState::Pause), spawn_pause_menu)
            .add_systems(Update, resume_on_esc.run_if(in_state(PlayingState::Pause)))
            .add_systems(OnExit(PlayingState::Pause), despawn_pause_menu)
            .add_systems(OnEnter(PlayingState::Win), spawn_winning_screen)
            .add_systems(OnExit(PlayingState::Win), despawn_winning_screen)
            .add_systems(
                Update,
                (handle_win_screen_buttons, handle_win_screen_hover)
                    .run_if(in_state(PlayingState::Win)),
            );
    }
}
