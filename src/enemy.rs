mod animation;
mod movement;

use std::time::Duration;

use bevy::prelude::*;
use rand::{SeedableRng, rngs::StdRng, seq::SliceRandom};

use crate::{
    animation::{AnimationController, MovementDirection},
    assets::{
        ImageAssets, TilesetHandles,
        enemy_tileset::{self, EnemyTileType},
        material::ColouringMaterial,
    },
    constants::{
        GHOST_SPEED, GHOSTS_SPAWNED, HOODIE_SPEED, HOODIES_SPAWNED, ZOMBIE_SPEED, ZOMBIES_SPAWNED,
    },
    enemy::{
        animation::{
            animate_enemies, get_ghost_animation_frames, get_hoodie_animation_frames,
            get_zombie_animation_frames,
        },
        movement::{EnemyMovement, move_enemies, tick_enemy_temporal_bonuses},
    },
    game_state::GameState,
    map::WorldMap,
    position::TilePosition,
    rendering::MeshHandle,
    world_entities::{
        ActorState, DespawnOnMainMenu, Enemy, GameplaySet, Killable, MovementMultiplier,
        MovementSpeed, SpawnEnemiesMessage, SpawnSystemSet, SpeedUpEnemies,
    },
};

const ENEMY_RNG_SEED: u64 = 123456789;

#[derive(Resource, Deref, DerefMut)]
struct EnemyRngGen(pub StdRng);

#[derive(Resource, Debug, Deref, DerefMut)]
struct EnemyTilesetMaterial(TilesetHandles<ColouringMaterial>);

#[derive(Component)]
struct PlayerChasingEnemy;

fn prepare_enemy_material(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut material: ResMut<Assets<ColouringMaterial>>,
) {
    let enemy_tileset_material =
        enemy_tileset::prepare_tilemap_material(&image_assets, &mut material);
    commands.insert_resource(EnemyTilesetMaterial(enemy_tileset_material));
}

fn spawn_single_enemy(
    enemy: Enemy,
    position: TilePosition,
    commands: &mut Commands,
    colouring_assets_storage: &mut Assets<ColouringMaterial>,
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<ColouringMaterial>,
) {
    let Some(material) = colouring_assets_storage.get(&material_handle).cloned() else {
        return;
    };

    let animation_function = match enemy {
        Enemy::Zombie => get_zombie_animation_frames,
        Enemy::Ghost => get_ghost_animation_frames,
        Enemy::Hoodie => get_hoodie_animation_frames,
    };

    let movement_speed = match enemy {
        Enemy::Zombie => ZOMBIE_SPEED,
        Enemy::Ghost => GHOST_SPEED,
        Enemy::Hoodie => HOODIE_SPEED,
    };

    commands
        .spawn((
            enemy,
            Killable,
            DespawnOnMainMenu,
            ActorState::Alive,
            position.to_world_position(),
            Mesh2d(mesh_handle),
            MeshMaterial2d(colouring_assets_storage.add(material.clone())),
            Transform::from_translation(Vec3::new(0.0, 0.0, 2.5)),
            AnimationController::<EnemyTileType>::new(animation_function),
            EnemyMovement::new(position),
            MovementDirection(None),
            MovementSpeed(movement_speed),
        ))
        .insert_if(PlayerChasingEnemy, || enemy == Enemy::Hoodie);
}

fn setup_spawn_enemies(
    mut commands: Commands,
    world_map: Res<WorldMap>,
    mut enemy_rng_gen: ResMut<EnemyRngGen>,
    mut material_assets: ResMut<Assets<ColouringMaterial>>,
    mesh_handle: Res<MeshHandle>,
    enemy_material: Res<EnemyTilesetMaterial>,
) {
    let mut free_locations = world_map
        .iter_empty_non_starting_tiles()
        .collect::<Vec<_>>();
    free_locations.shuffle(&mut enemy_rng_gen);

    for (tile, enemy) in free_locations.into_iter().zip(
        (0..ZOMBIES_SPAWNED)
            .map(|_| Enemy::Zombie)
            .chain((0..GHOSTS_SPAWNED).map(|_| Enemy::Ghost))
            .chain((0..HOODIES_SPAWNED).map(|_| Enemy::Hoodie)),
    ) {
        spawn_single_enemy(
            enemy,
            tile,
            &mut commands,
            &mut material_assets,
            mesh_handle.0.clone(),
            enemy_material.0.clone().0.clone(),
        );
    }
}

#[derive(Component, Deref, DerefMut)]
struct SpawnEnemiesTimer(Timer);

fn process_spawn_enemies_messages(
    mut commands: Commands,
    mut spawn_event: MessageReader<SpawnEnemiesMessage>,
) {
    if !spawn_event.is_empty() {
        for message in spawn_event.read() {
            commands.spawn((message.tile, SpawnEnemiesTimer(message.timer.clone())));
        }
        spawn_event.clear();
    }
}

fn spawn_delayed_enemies(
    mut commands: Commands,
    query: Query<(Entity, &TilePosition, &mut SpawnEnemiesTimer)>,
    time: Res<Time<Fixed>>,
    mut material_assets: ResMut<Assets<ColouringMaterial>>,
    mesh_handle: Res<MeshHandle>,
    enemy_material: Res<EnemyTilesetMaterial>,
) {
    let delta = time.delta();
    for (entity, position, mut timer) in query {
        timer.tick(delta);
        if timer.is_finished() {
            commands.entity(entity).despawn();
            for enemy in [Enemy::Zombie, Enemy::Zombie, Enemy::Ghost, Enemy::Ghost] {
                spawn_single_enemy(
                    enemy,
                    *position,
                    &mut commands,
                    &mut material_assets,
                    mesh_handle.0.clone(),
                    enemy_material.0.clone().0.clone(),
                );
            }
        }
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
        app.init_resource::<ImageAssets>()
            .insert_resource(EnemyRngGen(StdRng::seed_from_u64(ENEMY_RNG_SEED)))
            .add_systems(Startup, prepare_enemy_material)
            .add_systems(
                OnEnter(GameState::Playing),
                setup_spawn_enemies.in_set(SpawnSystemSet::SpawnUnits),
            )
            .add_observer(on_enemy_speed_up)
            .add_systems(
                FixedUpdate,
                (move_enemies, tick_enemy_temporal_bonuses)
                    .chain()
                    .in_set(GameplaySet::Movement),
            )
            .add_systems(
                FixedUpdate,
                (process_spawn_enemies_messages, spawn_delayed_enemies)
                    .chain()
                    .in_set(GameplaySet::EnemySpawning),
            )
            .add_systems(
                Update,
                animate_enemies.in_set(GameplaySet::AnimationAndSound),
            );
    }
}
