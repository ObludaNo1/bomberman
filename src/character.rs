mod animation;
mod movement;
mod spawn;

use bevy::prelude::*;

use crate::{
    character::{animation::animate_character, movement::move_character, spawn::spawn_character},
    game_state::GameState,
    world_entities::GameplaySet,
};

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_character)
            .add_systems(FixedUpdate, move_character.in_set(GameplaySet::Movement))
            .add_systems(PostUpdate, animate_character.in_set(GameplaySet::Animation));
    }
}
