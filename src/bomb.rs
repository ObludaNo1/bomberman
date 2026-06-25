use std::time::Duration;

use bevy::prelude::*;

mod animation;

use crate::{
    assets::{
        TilesetHandles,
        bomb_tileset::{BombTileType, prepare_bomb_tileset_handles},
    },
    bomb::animation::animate_bomb,
    controls::Controls,
    map::CollisionMap,
    position::WorldPosition,
    world_entities::{Bomb, Character, MapTileMarker},
};

const BOMB_TICKS: u32 = 6;
const BOMB_TICK_DURATION: f32 = 0.75;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
struct BombTiming {
    timer: Timer,
    pub bomb_ticks: u32,
}

impl Default for BombTiming {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(BOMB_TICK_DURATION, TimerMode::Repeating),
            bomb_ticks: 0,
        }
    }
}

impl BombTiming {
    pub fn update(&mut self, delta_time: Duration) {
        self.timer.tick(delta_time);

        let times_finished = self.timer.times_finished_this_tick();
        self.bomb_ticks += times_finished as u32;
    }

    pub fn is_on_final_tick(&self) -> bool {
        self.bomb_ticks == BOMB_TICKS - 1
    }

    pub fn is_bomb_tick_complete(&self) -> bool {
        self.bomb_ticks >= BOMB_TICKS
    }
}

fn spawn_bomb_when_requested(
    mut commands: Commands,
    characters: Query<&WorldPosition, With<Character>>,
    mut collision_map: ResMut<CollisionMap>,
    controls: Res<Controls>,
    bomb_assets: Res<BombAssets>,
) {
    if !controls.place_bomb {
        return;
    }

    for character_position in characters.iter() {
        if let Some(tile) = collision_map.get_tile_at_position(character_position)
            && tile.marker == MapTileMarker::Walkable
        {
            commands.spawn((
                Bomb,
                tile.world_pos(),
                Sprite::from_atlas_image(
                    bomb_assets.image.clone(),
                    TextureAtlas {
                        layout: bomb_assets.layout.clone(),
                        index: BombTileType::Bomb.index(),
                    },
                ),
                Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                BombTiming::default(),
            ));
            collision_map.set_tile(tile.x, tile.y, MapTileMarker::Obstacle);
        }
    }
}

fn remove_expired_bombs(
    mut commands: Commands,
    mut collision_map: ResMut<CollisionMap>,
    mut query: Query<(Entity, &WorldPosition, &mut BombTiming), With<Bomb>>,
    time: Res<Time>,
) {
    let delta_time = time.delta();
    for (entity, position, mut bomb_timing) in query.iter_mut() {
        bomb_timing.update(delta_time);
        if bomb_timing.is_bomb_tick_complete() {
            commands.entity(entity).despawn();
            if let Some(tile) = collision_map.get_tile_at_position(position) {
                collision_map.set_tile(tile.x, tile.y, MapTileMarker::Walkable);
            }
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
struct BombAssets(pub TilesetHandles);

fn prepare_bomb_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let handles = prepare_bomb_tileset_handles(&asset_server, &mut atlas_layouts);
    commands.insert_resource(BombAssets(handles));
}

pub struct BombPlugin;

impl Plugin for BombPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, prepare_bomb_assets).add_systems(
            Update,
            (
                spawn_bomb_when_requested,
                animate_bomb,
                remove_expired_bombs,
            )
                .chain(),
        );
    }
}
