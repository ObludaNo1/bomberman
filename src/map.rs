use bevy::prelude::*;
use rand::{RngExt, SeedableRng, rngs::StdRng};

use crate::{
    assets::{map_tileset, material::ColouringMaterial},
    game_state::GameState,
    position::WorldPosition,
    rendering::MeshHandle,
    world_entities::{InGameEntity, MapTileMarker, SpawnSystemSet},
};

pub const MAP_WIDTH: usize = 19;
pub const MAP_HEIGHT: usize = 15;

const RND_SEED: u64 = 123456789;
const WALL_DENSITY: f64 = 0.60;

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct WorldMap {
    tiles: Vec<MapTileMarker>,
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

    pub fn set_tile(&mut self, x: usize, y: usize, marker: MapTileMarker) {
        if x >= self.width() as usize || y >= self.height() as usize {
            return;
        }
        let index = y * self.width() as usize + x;
        if let Some(tile) = self.tiles.get_mut(index) {
            *tile = marker;
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
                if marker == MapTileMarker::Empty {
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

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct MapColouringMaterial(pub Handle<ColouringMaterial>);

#[derive(Component)]
pub struct MapTile;

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
    let tilemap_handles = map_tileset::prepare_tilemap_material(&asset_server, &mut material);
    let mut map_tile_markers = Vec::new();

    commands.insert_resource(MapColouringMaterial(tilemap_handles.0.clone()));

    let Some(colouring_material) = material.get(&tilemap_handles.0) else {
        return;
    };

    let mut indestructible_wall_material = colouring_material.clone();
    indestructible_wall_material.set_uv_rect(
        map_tileset::TILEMAP.sprite_uv_rect(map_tileset::MapTileType::IndestructibleWall),
    );
    let mut floor_material = colouring_material.clone();
    floor_material
        .set_uv_rect(map_tileset::TILEMAP.sprite_uv_rect(map_tileset::MapTileType::Floor));

    let mut wall_material = colouring_material.clone();
    wall_material.set_uv_rect(map_tileset::TILEMAP.sprite_uv_rect(map_tileset::MapTileType::Wall));

    let mut rng_gen = StdRng::seed_from_u64(RND_SEED);

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
                Tile::IndestructibleWall => MapTileMarker::IndestructibleWall,
                Tile::Wall => MapTileMarker::Wall,
                Tile::Floor => MapTileMarker::Empty,
            });

            commands.spawn((
                MapTile,
                InGameEntity,
                Mesh2d(mesh_handle.0.clone()),
                MeshMaterial2d(match tile_marker {
                    Tile::IndestructibleWall => material.add(indestructible_wall_material.clone()),
                    Tile::Wall => material.add(wall_material.clone()),
                    Tile::Floor => material.add(floor_material.clone()),
                }),
                WorldPosition(Vec2 {
                    x: x as f32 - ((MAP_WIDTH - 1) as f32) * 0.5,
                    y: y as f32 - ((MAP_HEIGHT - 1) as f32) * 0.5,
                }),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
        }
    }

    commands.insert_resource(WorldMap {
        tiles: map_tile_markers,
    });
}

pub struct Map;

impl Plugin for Map {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            setup_map.in_set(SpawnSystemSet::CreateMap),
        );
    }
}
