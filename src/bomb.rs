use std::time::Duration;

use bevy::prelude::*;

mod animation;

use crate::{
    assets::{
        TilesetHandles,
        bomb_explosion_tileset::prepare_bomb_explosion_tileset_handles,
        bomb_tileset::{BombTileType, prepare_bomb_tileset_handles},
    },
    bomb::animation::{animate_bomb, animate_explosion},
    controls::Controls,
    map::{CollisionMapTile, WorldMap},
    position::WorldPosition,
    world_entities::{Bomb, Character, Explosion, MapTileMarker},
};

const BOMB_TICKS: u32 = 6;
const BOMB_TICK_DURATION: f32 = 0.75;

const EXPLOSION_TICKS: u32 = 9;
const EXPLOSION_TICK_DURATION: f32 = 0.125;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
struct BombTiming {
    timer: Timer,
    pub ticks: u32,
    max_ticks: u32,
}

impl BombTiming {
    pub fn new(max_ticks: u32, interval: f32) -> Self {
        Self {
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
            ticks: 0,
            max_ticks,
        }
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.timer.tick(delta_time);

        let times_finished = self.timer.times_finished_this_tick();
        self.ticks += times_finished as u32;
    }

    pub fn is_on_final_tick(&self) -> bool {
        self.ticks == self.max_ticks - 1
    }

    pub fn is_finished(&self) -> bool {
        self.ticks >= self.max_ticks
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
struct ExplosionRadius(pub u32);

impl Default for ExplosionRadius {
    fn default() -> Self {
        Self(2)
    }
}

fn spawn_bomb_when_requested(
    mut commands: Commands,
    characters: Query<&WorldPosition, With<Character>>,
    mut world_map: ResMut<WorldMap>,
    controls: Res<Controls>,
    bomb_assets: Res<BombAssets>,
) {
    if !controls.place_bomb {
        return;
    }

    for character_position in characters.iter() {
        if let Some(tile) = world_map.get_tile_at_position(character_position)
            && tile.marker.is_walkable()
        {
            commands.spawn((
                Bomb,
                tile.world_pos(),
                Sprite::from_atlas_image(
                    bomb_assets.bomb_handles.image.clone(),
                    TextureAtlas {
                        layout: bomb_assets.bomb_handles.layout.clone(),
                        index: BombTileType::Bomb.index(),
                    },
                ),
                Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                BombTiming::new(BOMB_TICKS, BOMB_TICK_DURATION),
                ExplosionRadius::default(),
            ));
            world_map.set_tile(tile.x, tile.y, MapTileMarker::Bomb);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExplosionPathType {
    Center,
    Straight,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExplosionOrientation {
    Up,
    Down,
    Left,
    Right,
}

impl ExplosionOrientation {
    fn to_vec2(&self) -> Vec2 {
        match self {
            ExplosionOrientation::Up => Vec2::Y,
            ExplosionOrientation::Down => -Vec2::Y,
            ExplosionOrientation::Left => -Vec2::X,
            ExplosionOrientation::Right => Vec2::X,
        }
    }

    fn variants() -> [ExplosionOrientation; 4] {
        [
            ExplosionOrientation::Up,
            ExplosionOrientation::Down,
            ExplosionOrientation::Left,
            ExplosionOrientation::Right,
        ]
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
struct ExplosionTileVariant {
    pub kind: ExplosionPathType,
    pub orientation: ExplosionOrientation,
}

fn get_explosion_tiles(
    world_map: &WorldMap,
    bomb_position: &WorldPosition,
    explosion_radius: u32,
) -> Vec<(CollisionMapTile, ExplosionTileVariant)> {
    let mut explosion_tiles = Vec::new();

    if let Some(center_tile) = world_map.get_tile_at_position(bomb_position) {
        explosion_tiles.push((
            center_tile,
            ExplosionTileVariant {
                kind: ExplosionPathType::Center,
                orientation: ExplosionOrientation::Up,
            },
        ));

        for dir in ExplosionOrientation::variants().iter() {
            let dir_vec = dir.to_vec2();
            for i in 1..=explosion_radius {
                let offset = dir_vec * i as f32;
                let tile_pos = bomb_position.0 + offset;
                if let Some(tile) = world_map.get_tile_at_position(&tile_pos.into()) {
                    if tile.marker.is_walkable() {
                        let variant = if i == explosion_radius {
                            ExplosionPathType::End
                        } else {
                            ExplosionPathType::Straight
                        };
                        explosion_tiles.push((
                            tile,
                            ExplosionTileVariant {
                                kind: variant,
                                orientation: *dir,
                            },
                        ));
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    explosion_tiles
}

fn explode_expired_bombs(
    mut commands: Commands,
    mut world_map: ResMut<WorldMap>,
    mut query: Query<(Entity, &WorldPosition, &mut BombTiming, &ExplosionRadius), With<Bomb>>,
    time: Res<Time>,
) {
    let delta_time = time.delta();
    for (entity, position, mut bomb_timing, explosion_radius) in query.iter_mut() {
        bomb_timing.update(delta_time);
        if bomb_timing.is_finished() {
            commands.entity(entity).despawn();
            for (tile, variant) in get_explosion_tiles(&world_map, position, explosion_radius.0) {
                commands.spawn((
                    Explosion,
                    tile.world_pos(),
                    Sprite::default(),
                    Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    BombTiming::new(EXPLOSION_TICKS, EXPLOSION_TICK_DURATION),
                    variant,
                ));
                world_map.set_tile(tile.x, tile.y, MapTileMarker::Explosion);
            }
        }
    }
}

fn remove_expired_explosions(
    mut commands: Commands,
    mut world_map: ResMut<WorldMap>,
    mut query: Query<(Entity, &WorldPosition, &mut BombTiming), With<Explosion>>,
    time: Res<Time>,
) {
    let delta_time = time.delta();
    for (entity, position, mut bomb_timing) in query.iter_mut() {
        bomb_timing.update(delta_time);
        if bomb_timing.is_finished() {
            commands.entity(entity).despawn();
            if let Some(tile) = world_map.get_tile_at_position(position) {
                world_map.set_tile(tile.x, tile.y, MapTileMarker::Empty);
            }
        }
    }
}

#[derive(Resource)]
struct BombAssets {
    pub bomb_handles: TilesetHandles,
    pub bomb_explosion_handles: TilesetHandles,
}

fn prepare_bomb_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let bomb_handles = prepare_bomb_tileset_handles(&asset_server, &mut atlas_layouts);
    let bomb_explosion_handles =
        prepare_bomb_explosion_tileset_handles(&asset_server, &mut atlas_layouts);
    commands.insert_resource(BombAssets {
        bomb_handles,
        bomb_explosion_handles,
    });
}

pub struct BombPlugin;

impl Plugin for BombPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, prepare_bomb_assets).add_systems(
            Update,
            (
                (
                    spawn_bomb_when_requested,
                    explode_expired_bombs,
                    remove_expired_explosions,
                ),
                (animate_bomb, animate_explosion),
            )
                .chain(),
        );
    }
}
