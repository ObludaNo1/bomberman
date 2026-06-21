use bevy::{
    prelude::*,
    window::{Monitor, PrimaryMonitor, PrimaryWindow, Window, WindowMode},
};

use crate::assets::{
    TILEMAP_TEXTURE_PATH,
    map_tileset::{MapTileType, TILEMAP, prepare_tilemap_handles},
};

const MAP_WIDTH: i32 = 19;
const MAP_HEIGHT: i32 = 15;

fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    monitor: Query<&Monitor, With<PrimaryMonitor>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let (target_width, target_height) = match window.mode {
        WindowMode::Fullscreen(..) | WindowMode::BorderlessFullscreen(_) => {
            if let Ok(monitor) = monitor.single() {
                (
                    monitor.physical_width as f32,
                    monitor.physical_height as f32,
                )
            } else {
                (window.width(), window.height())
            }
        }
        WindowMode::Windowed => (window.width(), window.height()),
    };

    let tilemap_handles =
        prepare_tilemap_handles(&asset_server, &mut atlas_layouts, TILEMAP_TEXTURE_PATH);

    let tile_width = TILEMAP.tile_size.x as f32;
    let tile_height = TILEMAP.tile_size.y as f32;

    let map_width_px = MAP_WIDTH as f32 * tile_width;
    let map_height_px = MAP_HEIGHT as f32 * tile_height;
    let scale = (target_width / map_width_px).min(target_height / map_height_px);
    let scaled_tile_width = (tile_width * scale).floor();
    let scaled_tile_height = (tile_height * scale).floor();

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
