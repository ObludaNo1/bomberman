use bevy::prelude::*;

use crate::{
    assets::{map_tileset, material::ColouringMaterial},
    position::WorldPosition,
    rendering::MeshHandle,
    world_entities::MapTileMarker,
};

pub const MAP_WIDTH: i32 = 19;
pub const MAP_HEIGHT: i32 = 15;

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct WorldMap {
    tiles: Vec<MapTileMarker>,
}

impl WorldMap {
    pub fn width(&self) -> i32 {
        MAP_WIDTH
    }

    pub fn height(&self) -> i32 {
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut material: ResMut<Assets<ColouringMaterial>>,
    mesh_handle: Res<MeshHandle>,
) {
    let tilemap_handles = map_tileset::prepare_tilemap_material(&asset_server, &mut material);
    let mut map_tile_markers = Vec::new();

    let Some(colouring_material) = material.get(&tilemap_handles.0) else {
        return;
    };

    let mut wall_material = colouring_material.clone();
    wall_material.set_uv_rect(
        map_tileset::TILEMAP.sprite_uv_rect(map_tileset::MapTileType::IndestructibleWall),
    );
    let mut floor_material = colouring_material.clone();
    floor_material
        .set_uv_rect(map_tileset::TILEMAP.sprite_uv_rect(map_tileset::MapTileType::Floor));

    let wall_material = material.add(wall_material);
    let floor_material = material.add(floor_material);

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let is_indestructible_wall = x == 0
                || x == MAP_WIDTH - 1
                || y == 0
                || y == MAP_HEIGHT - 1
                || (x % 2 == 0 && y % 2 == 0);

            map_tile_markers.push(if is_indestructible_wall {
                MapTileMarker::Wall
            } else {
                MapTileMarker::Empty
            });

            commands.spawn((
                Mesh2d(mesh_handle.0.clone()),
                MeshMaterial2d(if is_indestructible_wall {
                    wall_material.clone()
                } else {
                    floor_material.clone()
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
        app.add_systems(Startup, setup_map);
    }
}
