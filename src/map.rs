use bevy::prelude::*;
use rand::{RngExt, SeedableRng, rngs::StdRng};

use crate::{
    assets::{
        map_tileset::{self, MapTilesetHandles},
        material::ColouringMaterial,
    },
    game_state::GameState,
    position::WorldPosition,
    rendering::MeshHandle,
    world_entities::{
        AllEnemiesKilledEvent, DestructibleWall, ExitGate, InGameEntity, MapTileMarker,
        SpawnSystemSet,
    },
};

pub const MAP_WIDTH: usize = 19;
pub const MAP_HEIGHT: usize = 15;

const RND_SEED: u64 = 123456789;
const WALL_DENSITY: f64 = 0.60;

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct WorldMap {
    tiles: Vec<MapTileMarker>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapTileSetter {
    Clear,
    Bomb,
    Explosion,
}

impl WorldMap {
    pub fn width(&self) -> usize {
        MAP_WIDTH
    }

    pub fn height(&self) -> usize {
        MAP_HEIGHT
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<CollisionMapTile> {
        if x >= self.width() as usize || y >= self.height() as usize {
            return None;
        }
        let index = y * self.width() as usize + x;
        self.tiles
            .get(index)
            .copied()
            .map(|marker| CollisionMapTile { x, y, marker })
    }

    pub fn get_tile_at_position(&self, position: &WorldPosition) -> Option<CollisionMapTile> {
        let x = (position.x + 0.5 + (MAP_WIDTH - 1) as f32 * 0.5) as usize;
        let y = (position.y + 0.5 + (MAP_HEIGHT - 1) as f32 * 0.5) as usize;
        self.get_tile(x, y)
    }

    pub fn get_position_from_world(&self, position: &WorldPosition) -> (usize, usize) {
        let x = (position.x + 0.5 + (MAP_WIDTH - 1) as f32 * 0.5) as usize;
        let y = (position.y + 0.5 + (MAP_HEIGHT - 1) as f32 * 0.5) as usize;
        (x, y)
    }

    pub fn set_tile(&mut self, x: usize, y: usize, value: MapTileSetter) {
        if x >= self.width() as usize || y >= self.height() as usize {
            return;
        }
        let index = y * self.width() as usize + x;
        if let Some(tile) = self.tiles.get_mut(index) {
            match value {
                MapTileSetter::Explosion => tile.set_explosion(true),
                MapTileSetter::Bomb => tile.set_bomb(true),
                MapTileSetter::Clear => tile.set_explosion(false).set_bomb(false).remove_wall(),
            };
        }
    }

    pub fn is_starting_area(x: usize, y: usize) -> bool {
        x <= 2 && y >= MAP_HEIGHT - 2 - 1
    }

    pub fn get_empty_tiles_non_starting_area(&self) -> impl Iterator<Item = CollisionMapTile> {
        self.tiles
            .iter()
            .enumerate()
            .filter_map(move |(index, &marker)| {
                if marker.is_floor() {
                    let x = index % self.width() as usize;
                    let y = index / self.width() as usize;
                    (!Self::is_starting_area(x, y)).then(|| CollisionMapTile { x, y, marker })
                } else {
                    None
                }
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollisionMapTile {
    pub x: usize,
    pub y: usize,
    pub marker: MapTileMarker,
}

impl CollisionMapTile {
    pub fn world_pos(&self) -> WorldPosition {
        Vec2::new(
            self.x as f32 - (MAP_WIDTH - 1) as f32 * 0.5,
            self.y as f32 - (MAP_HEIGHT - 1) as f32 * 0.5,
        )
        .into()
    }
}

#[derive(Component)]
pub struct MapTile;

#[derive(Component)]
struct ClosedGateTile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    IndestructibleWall,
    Wall,
    Floor,
}

fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut material: ResMut<Assets<ColouringMaterial>>,
    mesh_handle: Res<MeshHandle>,
) {
    let MapTilesetHandles {
        floor: floor_tilemap_handles,
        basic: basic_tilemap_handles,
        non_standard: non_standard_tilemap_handles,
    } = map_tileset::prepare_tilemap_material(&asset_server, &mut material);
    let mut map_tile_markers = Vec::new();

    let Some(floor_colouring_material) = material.get(&floor_tilemap_handles.0) else {
        return;
    };
    let Some(basic_colouring_material) = material.get(&basic_tilemap_handles.0) else {
        return;
    };
    let Some(non_standard_colouring_material) = material.get(&non_standard_tilemap_handles.0)
    else {
        return;
    };

    let mut indestructible_wall_material = basic_colouring_material.clone();
    indestructible_wall_material.set_uv_rect(
        map_tileset::BASIC_TILEMAP.sprite_uv_rect(map_tileset::MapTileType::IndestructibleWall),
    );
    let mut floor_material = floor_colouring_material.clone();
    floor_material
        .set_uv_rect(map_tileset::BASIC_TILEMAP.sprite_uv_rect(map_tileset::MapTileType::Floor));

    let mut wall_material = basic_colouring_material.clone();
    wall_material
        .set_uv_rect(map_tileset::BASIC_TILEMAP.sprite_uv_rect(map_tileset::MapTileType::Wall));
    let mut closed_gate_material = non_standard_colouring_material.clone();
    closed_gate_material.set_uv_rect(
        map_tileset::NON_STANDARD_TILEMAP.sprite_uv_rect(map_tileset::MapGateTileType::Closed),
    );
    let mut open_gate_material = non_standard_colouring_material.clone();
    open_gate_material.set_uv_rect(
        map_tileset::NON_STANDARD_TILEMAP.sprite_uv_rect(map_tileset::MapGateTileType::Open),
    );

    let mut rng_gen = StdRng::seed_from_u64(RND_SEED);

    // These are never changed
    let floor_material = material.add(floor_material);
    let indestructible_wall_material = material.add(indestructible_wall_material);

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let tile_marker = if x == 0
                || x == MAP_WIDTH - 1
                || y == 0
                || y == MAP_HEIGHT - 1
                || (x % 2 == 0 && y % 2 == 0)
            {
                Tile::IndestructibleWall
            } else if WorldMap::is_starting_area(x, y) {
                Tile::Floor
            } else if rng_gen.random_bool(WALL_DENSITY) {
                Tile::Wall
            } else {
                Tile::Floor
            };

            map_tile_markers.push(match tile_marker {
                Tile::IndestructibleWall => MapTileMarker::indestructible_wall(),
                Tile::Wall => MapTileMarker::basic_wall(),
                Tile::Floor => MapTileMarker::floor(),
            });

            let world_position = WorldPosition(Vec2 {
                x: x as f32 - ((MAP_WIDTH - 1) as f32) * 0.5,
                y: y as f32 - ((MAP_HEIGHT - 1) as f32) * 0.5,
            });

            match tile_marker {
                Tile::IndestructibleWall | Tile::Floor => {
                    let material = match tile_marker {
                        Tile::IndestructibleWall => indestructible_wall_material.clone(),
                        Tile::Floor => floor_material.clone(),
                        _ => unreachable!(),
                    };
                    commands.spawn((
                        MapTile,
                        InGameEntity,
                        Mesh2d(mesh_handle.0.clone()),
                        MeshMaterial2d(material.clone()),
                        world_position,
                        Transform::from_xyz(0.0, 0.0, 0.0),
                    ));
                }
                Tile::Wall => {
                    // Every wall is handled by spawning both floor and wall. Wall can be destroyed
                    // but floor remains.
                    commands.spawn((
                        MapTile,
                        InGameEntity,
                        Mesh2d(mesh_handle.0.clone()),
                        MeshMaterial2d(floor_material.clone()),
                        world_position,
                        Transform::from_xyz(0.0, 0.0, 0.0),
                    ));
                    commands.spawn((
                        MapTile,
                        InGameEntity,
                        DestructibleWall,
                        Mesh2d(mesh_handle.0.clone()),
                        MeshMaterial2d(material.add(wall_material.clone())),
                        world_position,
                        Transform::from_xyz(0.0, 0.0, 0.1),
                    ));
                }
            }
        }
    }

    let wall_indices = map_tile_markers
        .iter()
        .enumerate()
        .filter_map(|(i, tile)| tile.is_basic_wall().then_some(i))
        .collect::<Vec<_>>();
    let gate_index = if wall_indices.len() > 0 {
        wall_indices
            .get(rng_gen.random_range(0..wall_indices.len()))
            .copied()
    } else {
        None
    };
    if let Some(gate_index) = gate_index {
        if let Some(wall) = map_tile_markers.get_mut(gate_index) {
            *wall = wall.with_exit()
        };
        let world_position = WorldPosition(Vec2 {
            x: (gate_index % MAP_WIDTH) as f32 - ((MAP_WIDTH - 1) as f32) * 0.5,
            y: (gate_index / MAP_WIDTH) as f32 - ((MAP_HEIGHT - 1) as f32) * 0.5,
        });
        commands.spawn((
            MapTile,
            InGameEntity,
            ClosedGateTile,
            Mesh2d(mesh_handle.0.clone()),
            MeshMaterial2d(material.add(closed_gate_material)),
            world_position,
            Transform::from_xyz(0.0, 0.0, 0.04),
        ));
        commands.spawn((
            MapTile,
            InGameEntity,
            ExitGate,
            Mesh2d(mesh_handle.0.clone()),
            MeshMaterial2d(material.add(open_gate_material)),
            world_position,
            Transform::from_xyz(0.0, 0.0, 0.02),
        ));
    };

    commands.insert_resource(WorldMap {
        tiles: map_tile_markers,
    });
}

fn on_all_enemies_killed(
    _: On<AllEnemiesKilledEvent>,
    mut commands: Commands,
    closed_gate_tiles: Query<Entity, With<ClosedGateTile>>,
) {
    for entity in closed_gate_tiles {
        commands.entity(entity).despawn();
    }
}

pub struct Map;

impl Plugin for Map {
    fn build(&self, app: &mut App) {
        app.add_observer(on_all_enemies_killed).add_systems(
            OnEnter(GameState::Playing),
            setup_map.in_set(SpawnSystemSet::CreateMap),
        );
    }
}
