use bevy::prelude::*;

use crate::{
    animation::{AnimationController, MovementDirection},
    assets::{
        character_tileset::{self, CharacterTileType},
        material::ColouringMaterial,
    },
    character::animation::get_character_animation_frames,
    position::WorldPosition,
    rendering::MeshHandle,
    world_entities::{BombCount, BombRange, Character, InGameEntity, Killable},
};

pub fn spawn_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut material: ResMut<Assets<ColouringMaterial>>,
    mesh_handle: Res<MeshHandle>,
) {
    let character_tileset_material =
        character_tileset::prepare_tilemap_material(&asset_server, &mut material);

    // Clone the shared template so this character can mutate uv_min/uv_max/flip_x without affecting
    // other entities.
    let Some(character_material) = material.get(&character_tileset_material.0).cloned() else {
        return;
    };
    let character_material = material.add(character_material);

    let spawn_position = Vec2::new(-8.0, 6.0);
    commands.spawn((
        Character,
        Killable,
        InGameEntity,
        WorldPosition(spawn_position),
        Mesh2d(mesh_handle.0.clone()),
        MeshMaterial2d(character_material),
        Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
        AnimationController::<CharacterTileType>::new(get_character_animation_frames),
        MovementDirection(None),
        BombRange::default(),
        BombCount::default(),
    ));
}
