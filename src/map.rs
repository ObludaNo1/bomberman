use bevy::prelude::*;

use crate::assets::{
    TILEMAP_TEXTURE_PATH, TilemapHandles,
    map_tileset::{MapTileType, TILEMAP},
    prepare_tilemap_handles,
};

const MAP_WIDTH: i32 = 10;
const MAP_HEIGHT: i32 = 10;

fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let tilemap_handles: TilemapHandles =
        prepare_tilemap_handles(&asset_server, &mut atlas_layouts, TILEMAP_TEXTURE_PATH);

    let tile_width = TILEMAP.tile_size.x as f32;
    let tile_height = TILEMAP.tile_size.y as f32;
    let start_x = -((MAP_WIDTH as f32 - 1.0) * tile_width) * 0.5;
    let start_y = -((MAP_HEIGHT as f32 - 1.0) * tile_height) * 0.5;
    let floor_index = MapTileType::Floor.index();

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            commands.spawn((
                Sprite::from_atlas_image(
                    tilemap_handles.image.clone(),
                    TextureAtlas {
                        layout: tilemap_handles.layout.clone(),
                        index: floor_index,
                    },
                ),
                Transform::from_xyz(
                    start_x + x as f32 * tile_width,
                    start_y + y as f32 * tile_height,
                    0.0,
                ),
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
