use bevy::prelude::*;

use crate::{
    assets::map_tileset::{MapTileType, prepare_tilemap_handles},
    position::WorldPosition,
};

pub const MAP_WIDTH: i32 = 19;
pub const MAP_HEIGHT: i32 = 15;

fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let tilemap_handles = prepare_tilemap_handles(&asset_server, &mut atlas_layouts);

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
                WorldPosition(Vec2 {
                    x: x as f32 - ((MAP_WIDTH - 1) as f32) * 0.5,
                    y: y as f32 - ((MAP_HEIGHT - 1) as f32) * 0.5,
                }),
                Transform::from_xyz(0.0, 0.0, 0.0),
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
