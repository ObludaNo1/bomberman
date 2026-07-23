use bevy::prelude::*;

use crate::{
    animation::{AnimationController, MovementDirection},
    assets::{
        ImageAssets,
        character_tileset::{self, CharacterTileType},
        material::ColouringMaterial,
    },
    character::animation::get_character_animation_frames,
    map::WorldMap,
    rendering::MeshHandle,
    util::EntityScale,
    world_entities::{
        ActorState, BombCount, BombRange, Character, InGameEntity, Killable, MovementSpeed,
    },
};

const CHARACTER_SPEED: f32 = 2.0;

pub fn spawn_character(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut material: ResMut<Assets<ColouringMaterial>>,
    mesh_handle: Res<MeshHandle>,
) {
    let character_tileset_material =
        character_tileset::prepare_tilemap_material(&image_assets, &mut material);

    // Clone the shared template so this character can mutate uv_min/uv_max/flip_x without affecting
    // other entities.
    let Some(character_material) = material.get(&character_tileset_material.0).cloned() else {
        return;
    };
    let character_material = material.add(character_material);

    commands.spawn((
        Character,
        Killable,
        InGameEntity,
        ActorState::Alive,
        WorldMap::get_player_spawning_location().to_world_position(),
        Mesh2d(mesh_handle.0.clone()),
        MeshMaterial2d(character_material),
        Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
        AnimationController::<CharacterTileType>::new(get_character_animation_frames),
        MovementDirection(None),
        MovementSpeed(CHARACTER_SPEED),
        BombRange::default(),
        BombCount::default(),
        EntityScale(1.0),
    ));
}
