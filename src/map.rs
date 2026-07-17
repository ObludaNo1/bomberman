use std::ops::IndexMut;

use bevy::prelude::*;
use rand::{RngExt, SeedableRng, rngs::StdRng, seq::SliceRandom};

use crate::{
    assets::{
        map_tileset::{self, MapTilesetHandles, PowerUpTileType},
        material::ColouringMaterial,
    },
    game_state::GameState,
    position::WorldPosition,
    rendering::MeshHandle,
    world_entities::{
        AllEnemiesKilledEvent, BonusType, DestructibleWall, ExitGate, InGameEntity, MapTileMarker,
        SpawnSystemSet,
    },
};

pub const MAP_WIDTH: usize = 19;
pub const MAP_HEIGHT: usize = 15;

const RND_SEED: u64 = 123456789;
const WALL_DENSITY: f64 = 0.60;

const BONUSES: [(PowerUpTileType, u8); PowerUpTileType::COUNT as usize] = [
    (PowerUpTileType::Range, 5),
    (PowerUpTileType::BombCount, 5),
    (PowerUpTileType::Negative, 5),
    (PowerUpTileType::ExtraLife, 0),
    (PowerUpTileType::Hook, 0),
    (PowerUpTileType::BombKick, 0),
    (PowerUpTileType::Detonator, 0),
    (PowerUpTileType::Turbo, 0),
    (PowerUpTileType::LineBomb, 0),
    (PowerUpTileType::DoubleBomb, 0),
    (PowerUpTileType::Max, 0),
];

fn map_power_up(power_up_tile_type: PowerUpTileType) -> BonusType {
    match power_up_tile_type {
        PowerUpTileType::Range => BonusType::Range,
        PowerUpTileType::BombCount => BonusType::BombCount,
        PowerUpTileType::Negative => BonusType::Negative,
        PowerUpTileType::ExtraLife => BonusType::ExtraLife,
        PowerUpTileType::Hook => BonusType::Hook,
        PowerUpTileType::BombKick => BonusType::BombKick,
        PowerUpTileType::Detonator => BonusType::Detonator,
        PowerUpTileType::Turbo => BonusType::Turbo,
        PowerUpTileType::LineBomb => BonusType::LineBomb,
        PowerUpTileType::DoubleBomb => BonusType::DoubleBomb,
        PowerUpTileType::Max => BonusType::Max,
    }
}

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct WorldMap {
    tiles: Vec<MapTileMarker>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapTileSetter {
    Clear,
    Bomb,
    Explosion,
    PickupBonus,
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
                MapTileSetter::Clear => tile
                    .set_explosion(false)
                    .set_bomb(false)
                    .remove_wall()
                    .clear_bonus(),
                MapTileSetter::PickupBonus => tile.clear_bonus(),
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

fn index_to_world_position(index: usize) -> WorldPosition {
    WorldPosition(Vec2 {
        x: (index % MAP_WIDTH) as f32 - ((MAP_WIDTH - 1) as f32) * 0.5,
        y: (index / MAP_WIDTH) as f32 - ((MAP_HEIGHT - 1) as f32) * 0.5,
    })
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
        bonuses: bonuses_tilemap_handles,
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
    let Some(bonuses_colouring_material) = material.get(&bonuses_tilemap_handles.0) else {
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
    let mut bonuses_material = bonuses_colouring_material.clone();
    bonuses_material.set_uv_rect(
        map_tileset::BONUSES_TILEMAP.sprite_uv_rect(map_tileset::PowerUpTileType::Range),
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

    // Insert all bonuses behind walls
    let mut wall_indices = map_tile_markers
        .iter()
        .enumerate()
        .filter_map(|(i, tile)| tile.is_basic_wall().then_some(i))
        .collect::<Vec<_>>();
    wall_indices.shuffle(&mut rng_gen);

    if wall_indices.len() > 0 {
        let index = wall_indices[0];
        map_tile_markers.index_mut(index).with_exit();
        let world_position = index_to_world_position(index);
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

    let mut bonuses = Vec::new();
    for (bonus_type, count) in BONUSES {
        for _ in 0..count {
            bonuses.push(bonus_type);
        }
    }
    bonuses.shuffle(&mut rng_gen);
    for (i, bonus_type) in wall_indices.into_iter().skip(1).zip(bonuses) {
        let world_position = index_to_world_position(i);
        let bonus_type_component = map_power_up(bonus_type);
        map_tile_markers
            .index_mut(i)
            .with_bonus(bonus_type_component);
        let mut bonuses_material = bonuses_material.clone();
        bonuses_material.set_uv_rect(map_tileset::BONUSES_TILEMAP.sprite_uv_rect(bonus_type));
        commands.spawn((
            MapTile,
            InGameEntity,
            bonus_type_component,
            Mesh2d(mesh_handle.0.clone()),
            MeshMaterial2d(material.add(bonuses_material)),
            world_position,
            Transform::from_xyz(0.0, 0.0, 0.02),
        ));
    }

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
