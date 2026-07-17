mod animation;
mod bonuses;
mod movement;
mod spawn;
mod victory;

use bevy::prelude::*;

use crate::{
    character::{
        animation::animate_character,
        bonuses::pick_up_bonuses,
        movement::move_character,
        spawn::spawn_character,
        victory::{check_for_win, victory_ending},
    },
    game_state::GameState,
    world_entities::{AllEnemiesKilled, GameplaySet},
};

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_character)
            .add_systems(
                FixedUpdate,
                (move_character, pick_up_bonuses)
                    .chain()
                    .in_set(GameplaySet::Movement),
            )
            .add_systems(
                FixedUpdate,
                (check_for_win, victory_ending)
                    .in_set(GameplaySet::DeathAndVictory)
                    .run_if(resource_exists::<AllEnemiesKilled>),
            )
            .add_systems(PostUpdate, animate_character.in_set(GameplaySet::Animation));
    }
}
