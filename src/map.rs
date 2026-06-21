use bevy::prelude::*;

use crate::{
    assets::{
        TILESET_TILE_SIZE,
        map_tileset::{MapTileType, prepare_tilemap_handles},
    },
    util::CameraScale,
};

pub const MAP_WIDTH: i32 = 19;
pub const MAP_HEIGHT: i32 = 15;

fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    camera_scale: Res<CameraScale>,
) {
    let tilemap_handles = prepare_tilemap_handles(&asset_server, &mut atlas_layouts);

    let scale = camera_scale.0;

    let scaled_tile_width = (TILESET_TILE_SIZE.x as f32 * scale).floor();
    let scaled_tile_height = (TILESET_TILE_SIZE.y as f32 * scale).floor();

    let start_x = -((MAP_WIDTH as f32 - 1.0) * scaled_tile_width) * 0.5;
    let start_y = -((MAP_HEIGHT as f32 - 1.0) * scaled_tile_height) * 0.5;

    let floor_index = MapTileType::Floor.index();
    let indestructible_wall_index = MapTileType::IndestructibleWall.index();

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let is_indestructible_wall = x == 0
                || x == MAP_WIDTH - 1
                || y == 0
                || y == MAP_HEIGHT - 1
                || (x % 2 == 0 && y % 2 == 0);

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
                Transform::from_xyz(
                    start_x + x as f32 * scaled_tile_width,
                    start_y + y as f32 * scaled_tile_height,
                    0.0,
                )
                .with_scale(Vec3::splat(scale)),
            ));
        }
    }
}

pub struct Map;

impl Plugin for Map {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_map);
    }
}
