use bevy::prelude::*;

use crate::{
    assets::map_tileset::{MapTileType, prepare_tilemap_handles},
    position::WorldPosition,
    world_entities::MapTileMarker,
};

pub const MAP_WIDTH: i32 = 19;
pub const MAP_HEIGHT: i32 = 15;

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct CollisionMap {
    tiles: Vec<MapTileMarker>,
}

impl CollisionMap {
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
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let tilemap_handles = prepare_tilemap_handles(&asset_server, &mut atlas_layouts);

    let floor_index = MapTileType::Floor.index();
    let indestructible_wall_index = MapTileType::IndestructibleWall.index();

    let mut map_tile_markers = Vec::new();

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let is_indestructible_wall = x == 0
                || x == MAP_WIDTH - 1
                || y == 0
                || y == MAP_HEIGHT - 1
                || (x % 2 == 0 && y % 2 == 0);

            map_tile_markers.push(if is_indestructible_wall {
                MapTileMarker::Obstacle
            } else {
                MapTileMarker::Walkable
            });

            commands.spawn((
                Sprite::from_atlas_image(
                    tilemap_handles.image.clone(),
                    TextureAtlas {
                        layout: tilemap_handles.layout.clone(),
                        index: if is_indestructible_wall {
                            indestructible_wall_index
                        } else {
                            floor_index
                        },
                    },
                ),
                WorldPosition(Vec2 {
                    x: x as f32 - ((MAP_WIDTH - 1) as f32) * 0.5,
                    y: y as f32 - ((MAP_HEIGHT - 1) as f32) * 0.5,
                }),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
        }
    }

    commands.insert_resource(CollisionMap {
        tiles: map_tile_markers,
    });
}

pub struct Map;

impl Plugin for Map {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_map);
    }
}
