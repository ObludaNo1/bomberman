use bevy::prelude::*;

use crate::{
    assets::{
        TILESET_TILE_SIZE,
        character_tileset::{CharacterTileType, prepare_character_tileset_handles},
    },
    position::WorldPosition,
    util::CameraScale,
};

#[derive(Component)]
pub struct Character;

pub fn spawn_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    camera_scale: Res<CameraScale>,
) {
    let character_tileset_handles =
        prepare_character_tileset_handles(&asset_server, &mut atlas_layouts);

    let scale = camera_scale.0;
    let spawn_position = Vec2::new(-8.0, 6.0);
    commands.spawn((
        Character,
        WorldPosition(spawn_position),
        Sprite::from_atlas_image(
            character_tileset_handles.image.clone(),
            TextureAtlas {
                layout: character_tileset_handles.layout.clone(),
                index: CharacterTileType::Standing.index(),
            },
        ),
        Transform::from_xyz(
            spawn_position.x * TILESET_TILE_SIZE.x as f32 * scale,
            spawn_position.y * TILESET_TILE_SIZE.y as f32 * scale,
            1.0,
        )
        .with_scale(Vec3::splat(scale)),
    ));
}
