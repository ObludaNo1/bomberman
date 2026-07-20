use bevy::prelude::*;

mod animation;

use crate::{
    assets::{
        TilesetHandles, bomb_explosion_tileset,
        bomb_tileset::{self, BombTileType},
        material::{ColouringMaterial, ExplosionMaterial},
    },
    bomb::animation::{
        advance_and_despawn_explosions, animate_bomb, animate_exploding_walls, animate_explosion,
        spawn_explosion_visuals,
    },
    constants::BOMB_DURATION,
    controls::Controls,
    map::{BombTile, WorldMap},
    position::WorldPosition,
    rendering::MeshHandle,
    util::RenderScale,
    world_entities::{
        Bomb, BombCount, BombRange, Character, GameplaySet, InGameEntity, MarkToDespawn,
    },
};

const BOMB_TICKS: u32 = 6;

const EXPLOSION_TICKS: u32 = 9;

fn spawn_bomb_when_requested(
    mut commands: Commands,
    mut characters: Query<(&WorldPosition, &BombRange, &mut BombCount), With<Character>>,
    mut world_map: ResMut<WorldMap>,
    controls: Res<Controls>,
    bomb_assets: Res<BombAssets>,
    mesh_handle: Res<MeshHandle>,
    mut materials: ResMut<Assets<ColouringMaterial>>,
) {
    if !controls.place_bomb {
        return;
    }

    let Some(mut bomb_material) = materials.get(&bomb_assets.bomb_handles.0).cloned() else {
        return;
    };
    bomb_material.set_uv_rect(bomb_tileset::TILEMAP.sprite_uv_rect(BombTileType::Bomb));
    bomb_material.set_flip_x(false);
    let bomb_material = materials.add(bomb_material);

    for (character_position, bomb_range, mut bomb_count) in characters.iter_mut() {
        let bomb_position = character_position.to_closest_tile();
        if let Some(tile) = world_map.get_tile(bomb_position)
            && tile.is_walkable()
            && tile
                .bomb_or_explosion()
                .map(|t| !t.is_bomb())
                .unwrap_or(true)
            && bomb_count.current < bomb_count.max
        {
            if world_map.try_add_bomb(
                bomb_position,
                BombTile {
                    range: bomb_range.0,
                    timer: Timer::new(BOMB_DURATION, TimerMode::Once),
                },
            ) {
                bomb_count.current += 1;
                commands.spawn((
                    Bomb,
                    InGameEntity,
                    bomb_position,
                    Mesh2d(mesh_handle.0.clone()),
                    MeshMaterial2d(bomb_material.clone()),
                    Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    RenderScale(1.0),
                ));
            }
        }
    }
}

fn despawn_exploded_bombs(
    mut commands: Commands,
    query: Query<Entity, (With<MarkToDespawn>, With<Bomb>)>,
    mut players: Query<&mut BombCount, (With<Character>, Without<Bomb>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
        for mut bomb_count in players.iter_mut() {
            bomb_count.current -= 1;
        }
    }
}

#[derive(Resource)]
struct BombAssets {
    pub bomb_handles: TilesetHandles<ColouringMaterial>,
    pub bomb_explosion_handles: TilesetHandles<ExplosionMaterial>,
}

fn prepare_bomb_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut colouring_material: ResMut<Assets<ColouringMaterial>>,
    mut explostion_material: ResMut<Assets<ExplosionMaterial>>,
) {
    let bomb_handles =
        bomb_tileset::prepare_tilemap_material(&asset_server, &mut colouring_material);
    let Some(bomb_colouring) = colouring_material.get_mut(&bomb_handles.0) else {
        return;
    };
    bomb_colouring.set_uv_rect(bomb_tileset::TILEMAP.sprite_uv_rect(BombTileType::Bomb));

    let bomb_explosion_handles =
        bomb_explosion_tileset::prepare_tilemap_material(&asset_server, &mut explostion_material);
    commands.insert_resource(BombAssets {
        bomb_handles,
        bomb_explosion_handles,
    });
}

pub struct BombPlugin;

impl Plugin for BombPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, prepare_bomb_assets)
            .add_systems(
                FixedUpdate,
                (
                    spawn_bomb_when_requested.in_set(GameplaySet::BombPlacement),
                    despawn_exploded_bombs.in_set(GameplaySet::MapTickUpdate),
                ),
            )
            .add_systems(
                PostUpdate,
                (
                    spawn_explosion_visuals,
                    advance_and_despawn_explosions,
                    animate_bomb,
                    animate_explosion,
                    animate_exploding_walls,
                )
                    .in_set(GameplaySet::AnimationAndSound),
            );
    }
}
