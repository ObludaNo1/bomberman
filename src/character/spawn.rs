use bevy::prelude::*;

use crate::{
    assets::character_tileset::{CharacterTileType, prepare_character_tileset_handles},
    character::movement::CharacterMovement,
    position::WorldPosition,
};

#[derive(Component)]
pub struct Character;

pub fn spawn_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let character_tileset_handles =
        prepare_character_tileset_handles(&asset_server, &mut atlas_layouts);

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
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        CharacterMovement::default(),
    ));
}
