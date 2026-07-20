mod animation;
mod movement;

use std::time::Duration;

use bevy::prelude::*;
use rand::{SeedableRng, rngs::StdRng, seq::SliceRandom};

use crate::{
    animation::{AnimationController, MovementDirection},
    assets::{
        enemy_tileset::{self, EnemyTileType},
        material::ColouringMaterial,
    },
    constants::ENEMIES_SPAWNED,
    enemy::{
        animation::{animate_enemies, get_enemy_animation_frames},
        movement::{EnemyMovement, move_enemies, tick_enemy_temporal_bonuses},
    },
    game_state::GameState,
    map::WorldMap,
    rendering::MeshHandle,
    world_entities::{
        ActorState, Enemy, GameplaySet, InGameEntity, Killable, MovementMultiplier, MovementSpeed,
        SpawnSystemSet, SpeedUpEnemies,
    },
};

const ENEMY_RNG_SEED: u64 = 123456789;

#[derive(Resource, Deref, DerefMut)]
struct EnemyRngGen(pub StdRng);

fn spawn_enemies(
    mut commands: Commands,
    world_map: Res<WorldMap>,
    asset_server: Res<AssetServer>,
    mut material: ResMut<Assets<ColouringMaterial>>,
    mesh_handle: Res<MeshHandle>,
    mut enemy_rng_gen: ResMut<EnemyRngGen>,
) {
    let enemy_tileset_material =
        enemy_tileset::prepare_tilemap_material(&asset_server, &mut material);
    let Some(enemy_material) = material.get(&enemy_tileset_material.0).cloned() else {
        return;
    };

    let mut free_locations = world_map
        .iter_empty_non_starting_tiles()
        .collect::<Vec<_>>();
    free_locations.shuffle(&mut enemy_rng_gen);

    for tile in free_locations.into_iter().take(ENEMIES_SPAWNED) {
        commands.spawn((
            Enemy,
            Killable,
            InGameEntity,
            ActorState::Alive,
            tile.to_world_position(),
            Mesh2d(mesh_handle.0.clone()),
            MeshMaterial2d(material.add(enemy_material.clone())),
            Transform::from_translation(Vec3::new(0.0, 0.0, 2.5)),
            AnimationController::<EnemyTileType>::new(get_enemy_animation_frames),
            EnemyMovement::new(tile),
            MovementDirection(None),
            MovementSpeed(1.5),
        ));
    }
}

fn on_enemy_speed_up(
    _trigger: On<SpeedUpEnemies>,
    mut commands: Commands,
    enemies: Query<Entity, With<Enemy>>,
) {
    for entity in enemies {
        commands
            .entity(entity)
            .insert(MovementMultiplier::new(Duration::from_secs(10), 2.0));
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyRngGen(StdRng::seed_from_u64(ENEMY_RNG_SEED)))
            .add_systems(
                OnEnter(GameState::Playing),
                spawn_enemies.in_set(SpawnSystemSet::SpawnEnemies),
            )
            .add_observer(on_enemy_speed_up)
            .add_systems(
                FixedUpdate,
                (move_enemies, tick_enemy_temporal_bonuses).in_set(GameplaySet::Movement),
            )
            .add_systems(
                Update,
                animate_enemies.in_set(GameplaySet::AnimationAndSound),
            );
    }
}
