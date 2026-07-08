use std::time::Duration;

use bevy::{
    camera::visibility::RenderLayers,
    platform::collections::{HashMap, HashSet},
    prelude::*,
};

mod animation;

use crate::{
    assets::{
        TilesetHandles, bomb_explosion_tileset,
        bomb_tileset::{self, BombTileType},
        map_tileset::{self, MapTileType},
        material::{ColouringMaterial, ExplosionMaterial},
    },
    bomb::animation::{animate_bomb, animate_exploding_walls, animate_explosion},
    controls::Controls,
    game_state::GameState,
    map::{CollisionMapTile, MapTile, WorldMap},
    position::WorldPosition,
    rendering::MeshHandle,
    util::RenderScale,
    world_entities::{Bomb, Character, Explosion, InGameEntity, MapTileMarker},
};

const BOMB_TICKS: u32 = 6;
const BOMB_TICK_DURATION: f32 = 0.75;

const EXPLOSION_TICKS: u32 = 9;
const EXPLOSION_TICK_DURATION: f32 = 0.125;

const WALL_EXPLOSION_TICKS: u32 = 4;
// match the duration of the explosion animation to the duration of the wall explosion animation
const WALL_EXPLOSION_TICK_DURATION: f32 =
    EXPLOSION_TICK_DURATION * EXPLOSION_TICKS as f32 / WALL_EXPLOSION_TICKS as f32;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
struct BombTiming {
    timer: Timer,
    pub ticks: u32,
    max_ticks: u32,
}

impl BombTiming {
    pub fn new(max_ticks: u32, interval: f32) -> Self {
        Self {
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
            ticks: 0,
            max_ticks,
        }
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.timer.tick(delta_time);

        let times_finished = self.timer.times_finished_this_tick();
        self.ticks += times_finished as u32;
    }

    pub fn is_on_final_tick(&self) -> bool {
        self.ticks == self.max_ticks - 1
    }

    pub fn is_finished(&self) -> bool {
        self.ticks >= self.max_ticks
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
struct ExplosionRadius(pub u32);

impl Default for ExplosionRadius {
    fn default() -> Self {
        Self(2)
    }
}

#[derive(Component)]
struct ExplodingWall;

fn spawn_bomb_when_requested(
    mut commands: Commands,
    characters: Query<&WorldPosition, With<Character>>,
    mut world_map: ResMut<WorldMap>,
    controls: Res<Controls>,
    bomb_assets: Res<BombAssets>,
    mesh_handle: Res<MeshHandle>,
    mut materials: ResMut<Assets<ColouringMaterial>>,
) {
    if !controls.place_bomb {
        return;
    }

    let Some(mut bomb_material) = materials.get(&bomb_assets.bomb_handles.0).cloned() else {
        return;
    };
    bomb_material.set_uv_rect(bomb_tileset::TILEMAP.sprite_uv_rect(BombTileType::Bomb));
    bomb_material.set_flip_x(false);
    let bomb_material = materials.add(bomb_material);

    for character_position in characters.iter() {
        if let Some(tile) = world_map.get_tile_at_position(character_position)
            && tile.marker.is_walkable()
        {
            commands.spawn((
                Bomb,
                InGameEntity,
                tile.world_pos(),
                Mesh2d(mesh_handle.0.clone()),
                MeshMaterial2d(bomb_material.clone()),
                Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                BombTiming::new(BOMB_TICKS, BOMB_TICK_DURATION),
                ExplosionRadius::default(),
                RenderScale(1.0),
            ));
            world_map.set_tile(tile.x, tile.y, MapTileMarker::Bomb);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExplosionPathType {
    Center,
    Straight,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExplosionOrientation {
    Up,
    Down,
    Left,
    Right,
}

impl ExplosionOrientation {
    fn to_vec2(&self) -> Vec2 {
        match self {
            ExplosionOrientation::Up => Vec2::Y,
            ExplosionOrientation::Down => -Vec2::Y,
            ExplosionOrientation::Left => -Vec2::X,
            ExplosionOrientation::Right => Vec2::X,
        }
    }

    fn variants() -> [ExplosionOrientation; 4] {
        [
            ExplosionOrientation::Up,
            ExplosionOrientation::Down,
            ExplosionOrientation::Left,
            ExplosionOrientation::Right,
        ]
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
struct ExplosionTileVariant {
    pub kind: ExplosionPathType,
    pub orientation: ExplosionOrientation,
}

fn get_explosion_tiles(
    world_map: &WorldMap,
    bomb_position: &WorldPosition,
    explosion_radius: u32,
) -> (
    Vec<(CollisionMapTile, ExplosionTileVariant)>,
    Vec<CollisionMapTile>,
    Vec<CollisionMapTile>,
) {
    let mut explosion_tiles = Vec::new();
    let mut chained_bombs = Vec::new();
    let mut destroyed_walls = Vec::new();

    if let Some(center_tile) = world_map.get_tile_at_position(bomb_position) {
        explosion_tiles.push((
            center_tile,
            ExplosionTileVariant {
                kind: ExplosionPathType::Center,
                orientation: ExplosionOrientation::Up,
            },
        ));

        for dir in ExplosionOrientation::variants().iter() {
            let dir_vec = dir.to_vec2();
            for i in 1..=explosion_radius {
                let offset = dir_vec * i as f32;
                let tile_pos = bomb_position.0 + offset;
                if let Some(tile) = world_map.get_tile_at_position(&tile_pos.into()) {
                    match tile.marker {
                        MapTileMarker::Empty | MapTileMarker::Explosion => {
                            let variant = if i == explosion_radius {
                                ExplosionPathType::End
                            } else {
                                ExplosionPathType::Straight
                            };
                            explosion_tiles.push((
                                tile,
                                ExplosionTileVariant {
                                    kind: variant,
                                    orientation: *dir,
                                },
                            ));
                        }
                        MapTileMarker::Bomb => {
                            chained_bombs.push(tile);
                            break;
                        }
                        MapTileMarker::IndestructibleWall => {
                            break;
                        }
                        MapTileMarker::Wall => {
                            destroyed_walls.push(tile);
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }

    (explosion_tiles, chained_bombs, destroyed_walls)
}

fn explode_expired_bombs(
    mut commands: Commands,
    mut world_map: ResMut<WorldMap>,
    mut explosion_materials: ResMut<Assets<ExplosionMaterial>>,
    mesh_handle: Res<MeshHandle>,
    bomb_assets: Res<BombAssets>,
    mut query: Query<
        (Entity, &WorldPosition, &mut BombTiming, &ExplosionRadius),
        (With<Bomb>, Without<MapTile>),
    >,
    map_tiles: Query<(Entity, &WorldPosition), (With<MapTile>, Without<Bomb>)>,
    time: Res<Time>,
) {
    let delta_time = time.delta();

    for (_, _, mut bomb_timing, _) in query.iter_mut() {
        bomb_timing.update(delta_time);
    }

    let query = query.as_readonly();

    // Hold all bombs in a map for quick lookup
    let mut bombs_map = HashMap::new();
    // Hold all bombs that explode this tick
    let mut bombs_to_explode_vec = Vec::new();
    for (entity, world_pos, bomb_timing, explosion_radius) in query {
        // Collect all bombs since they may chain-explode
        bombs_map.insert(
            world_map.get_position_from_world(world_pos),
            (entity, world_pos, explosion_radius),
        );
        if bomb_timing.is_finished() {
            // Collect all bombs that explode this tick
            bombs_to_explode_vec.push((entity, world_pos, explosion_radius));
        }
    }

    let mut walls_to_destroy = HashSet::new();

    let mut i = 0;
    while let Some((entity, world_pos, explosion_radius)) = bombs_to_explode_vec.get(i) {
        i += 1;

        commands.entity(*entity).despawn();
        let (explosions, bombs_to_explode, destroyed_walls) =
            get_explosion_tiles(&world_map, world_pos, explosion_radius.0);

        walls_to_destroy.extend(destroyed_walls.iter().map(|tile| (tile.x, tile.y)));

        for (tile, variant) in explosions {
            world_map.set_tile(tile.x, tile.y, MapTileMarker::Explosion);

            let Some(mut explosion_material) = explosion_materials
                .get(&bomb_assets.bomb_explosion_handles.0)
                .cloned()
            else {
                continue;
            };
            explosion_material.set_uv_rect(Rect::default());
            explosion_material.set_flip_x(false);
            let explosion_material = explosion_materials.add(explosion_material);
            commands.spawn((
                Explosion,
                InGameEntity,
                tile.world_pos(),
                Mesh2d(mesh_handle.0.clone()),
                MeshMaterial2d(explosion_material),
                Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                BombTiming::new(EXPLOSION_TICKS, EXPLOSION_TICK_DURATION),
                variant,
                RenderLayers::layer(1),
            ));
        }

        for bomb_to_explode in bombs_to_explode {
            if let Some((entity, world_pos, explosion_radius)) =
                bombs_map.remove(&(bomb_to_explode.x, bomb_to_explode.y))
            {
                bombs_to_explode_vec.push((entity, world_pos, explosion_radius));
            }
        }
    }

    for (entity, world_pos) in map_tiles {
        let (x, y) = world_map.get_position_from_world(world_pos);
        if walls_to_destroy.contains(&(x, y)) {
            commands.entity(entity).insert((
                ExplodingWall,
                BombTiming::new(WALL_EXPLOSION_TICKS, WALL_EXPLOSION_TICK_DURATION),
            ));
        }
    }
}

fn remove_expired_explosions(
    mut commands: Commands,
    mut world_map: ResMut<WorldMap>,
    mut query: Query<(Entity, &WorldPosition, &mut BombTiming), With<Explosion>>,
    time: Res<Time>,
) {
    let delta_time = time.delta();
    for (entity, position, mut bomb_timing) in query.iter_mut() {
        bomb_timing.update(delta_time);
        if bomb_timing.is_finished() {
            commands.entity(entity).despawn();
            if let Some(tile) = world_map.get_tile_at_position(position) {
                world_map.set_tile(tile.x, tile.y, MapTileMarker::Empty);
            }
        }
    }
}

#[derive(Resource)]
struct BombAssets {
    pub bomb_handles: TilesetHandles<ColouringMaterial>,
    pub bomb_explosion_handles: TilesetHandles<ExplosionMaterial>,
}

fn prepare_bomb_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut colouring_material: ResMut<Assets<ColouringMaterial>>,
    mut explostion_material: ResMut<Assets<ExplosionMaterial>>,
) {
    let bomb_handles =
        bomb_tileset::prepare_tilemap_material(&asset_server, &mut colouring_material);
    let Some(bomb_colouring) = colouring_material.get_mut(&bomb_handles.0) else {
        return;
    };
    bomb_colouring.set_uv_rect(bomb_tileset::TILEMAP.sprite_uv_rect(BombTileType::Bomb));

    let bomb_explosion_handles =
        bomb_explosion_tileset::prepare_tilemap_material(&asset_server, &mut explostion_material);
    commands.insert_resource(BombAssets {
        bomb_handles,
        bomb_explosion_handles,
    });
}

fn advance_exploding_walls(
    mut commands: Commands,
    mut world_map: ResMut<WorldMap>,
    mut query: Query<
        (
            Entity,
            &WorldPosition,
            &mut BombTiming,
            &MeshMaterial2d<ColouringMaterial>,
        ),
        With<ExplodingWall>,
    >,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColouringMaterial>>,
) {
    let delta_time = time.delta();
    for (entity, position, mut bomb_timing, material_handle) in query.iter_mut() {
        bomb_timing.update(delta_time);
        if bomb_timing.is_finished() {
            commands
                .entity(entity)
                .remove::<(ExplodingWall, BombTiming)>();

            if let Some(material) = materials.get_mut(&material_handle.0) {
                material.set_uv_rect(map_tileset::TILEMAP.sprite_uv_rect(MapTileType::Floor));
            }
            if let Some(tile) = world_map.get_tile_at_position(position) {
                world_map.set_tile(tile.x, tile.y, MapTileMarker::Empty);
            }
        }
    }
}

pub struct BombPlugin;

impl Plugin for BombPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, prepare_bomb_assets).add_systems(
            Update,
            (
                (
                    spawn_bomb_when_requested,
                    explode_expired_bombs,
                    remove_expired_explosions,
                    advance_exploding_walls,
                ),
                (animate_bomb, animate_explosion, animate_exploding_walls),
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}
