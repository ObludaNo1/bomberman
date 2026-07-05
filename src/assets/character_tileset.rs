use bevy::prelude::*;

use crate::{
    assets::{CHARACTER_TEXTURE_PATH, TILESET_TILE_SIZE, material::ColouringMaterial},
    tileset_enum,
};

tileset_enum!(
    Character,
    TILESET_TILE_SIZE,
    (222, 205),
    CHARACTER_TEXTURE_PATH,
    ColouringMaterial,
    Color::srgba(0.0, 0.0, 0.0, 1.0),
    Color::srgba(0.3, 0.3, 0.3, 1.0),
    Color::srgba(0.6, 0.6, 0.6, 1.0),
    Color::srgba(1.0, 1.0, 1.0, 0.0),
    StandingDown => (1, 84),
    MovingDown => (18, 84),
    StandingUp => (35, 84),
    MovingUp => (52, 84),
    MovingRight1 => (69, 84),
    StandingRight => (86, 84),
    MovingRight2 => (103, 84),
);
