use bevy::prelude::*;

use crate::{
    assets::{CHARACTER_TEXTURE_PATH, TILESET_TILE_SIZE, material::ExplosionMaterial},
    tileset_enum,
};

tileset_enum!(
    BombExplosion,
    TILESET_TILE_SIZE,
    (222, 205),
    CHARACTER_TEXTURE_PATH,
    ExplosionMaterial,
    Color::srgba(0.9, 0.2, 0.05, 1.0),
    Color::srgba(0.9, 0.65, 0.05, 0.75),
    Color::srgba(0.9, 0.9, 0.05, 0.5),
    Color::srgba(1.0, 1.0, 1.0, 0.0),
    ExplosionCenter1 => (19, 170),
    ExplosionStraight1 => (36, 153),
    ExplosionEnd1 => (2, 153),
    ExplosionCenter2 => (72, 170),
    ExplosionStraight2 => (89, 153),
    ExplosionEnd2 => (55, 153),
    ExplosionCenter3 => (125, 170),
    ExplosionStraight3 => (142, 153),
    ExplosionEnd3 => (108, 153),
    ExplosionCenter4 => (178, 170),
    ExplosionStraight4 => (195, 153),
    ExplosionEnd4 => (161, 153),
);
