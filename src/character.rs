mod spawn;

use bevy::prelude::*;

use crate::character::spawn::spawn_character;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_character);
    }
}
