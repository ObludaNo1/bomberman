mod animation;
mod movement;
mod spawn;

use bevy::prelude::*;

use crate::{
    character::{animation::animate_character, movement::move_character, spawn::spawn_character},
    controls::Direction,
};

#[derive(Component, Deref, DerefMut, Debug, Clone, Copy, PartialEq, Eq)]
struct MovementDirection(pub Option<Direction>);

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_character)
            .add_systems(Update, (move_character, animate_character).chain());
    }
}
