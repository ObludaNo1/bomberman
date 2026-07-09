mod animation;
mod movement;
mod spawn;

use bevy::prelude::*;

use crate::{
    character::{animation::animate_character, movement::move_character, spawn::spawn_character},
    controls::Direction,
    game_state::GameState,
    world_entities::GameplaySet,
};

#[derive(Component, Deref, DerefMut, Debug, Clone, Copy, PartialEq, Eq)]
struct MovementDirection(pub Option<Direction>);

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_character)
            .add_systems(
                Update,
                (
                    move_character.in_set(GameplaySet::Movement),
                    animate_character.in_set(GameplaySet::Animation),
                ),
            );
    }
}
