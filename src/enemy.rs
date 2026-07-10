mod animation;

use bevy::prelude::*;
use rand::{RngExt, SeedableRng, rngs::StdRng};

use crate::{
    animation::{AnimationController, MovementDirection},
    assets::{
        enemy_tileset::{self, EnemyTileType},
        material::ColouringMaterial,
    },
    enemy::animation::{animate_enemies, get_enemy_animation_frames},
    game_state::GameState,
    map::WorldMap,
    rendering::MeshHandle,
    world_entities::{GameplaySet, InGameEntity, Killable, SpawnSystemSet},
};

const ENEMY_SPAWN_SEED: u64 = 1234567890;
const ENEMIES_SPAWNED: usize = 1;

#[derive(Component)]
struct Enemy;

fn spawn_enemies(
    mut commands: Commands,
    world_map: Res<WorldMap>,
    asset_server: Res<AssetServer>,
    mut material: ResMut<Assets<ColouringMaterial>>,
    mesh_handle: Res<MeshHandle>,
) {
    let enemy_tileset_material =
        enemy_tileset::prepare_tilemap_material(&asset_server, &mut material);
    let Some(enemy_material) = material.get(&enemy_tileset_material.0).cloned() else {
        return;
    };

    let free_locations = world_map
        .get_empty_tiles_non_starting_area()
        .collect::<Vec<_>>();

    let mut rng_gen = StdRng::seed_from_u64(ENEMY_SPAWN_SEED);

    for _ in 0..ENEMIES_SPAWNED {
        let index = rng_gen.random_range(0..free_locations.len());
        let tile = free_locations[index];
        let enemy_material = material.add(enemy_material.clone());
        commands.spawn((
            Enemy,
            Killable,
            InGameEntity,
            tile.world_pos(),
            Mesh2d(mesh_handle.0.clone()),
            MeshMaterial2d(enemy_material),
            Transform::from_translation(Vec3::new(0.0, 0.0, 2.5)),
            AnimationController::<EnemyTileType>::new(get_enemy_animation_frames),
            MovementDirection(None),
        ));
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            spawn_enemies.in_set(SpawnSystemSet::SpawnEnemies),
        )
        .add_systems(Update, animate_enemies.in_set(GameplaySet::Animation));
    }
}
