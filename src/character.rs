mod movement;
mod spawn;

use bevy::prelude::*;

use crate::character::{movement::move_character, spawn::spawn_character};

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_character)
            .add_systems(Update, move_character);
    }
}
