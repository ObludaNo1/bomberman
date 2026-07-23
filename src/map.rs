mod explosion;
mod generation;
mod map_tile;
mod neighbours;

use std::time::Duration;

use bevy::prelude::*;
pub use map_tile::*;
pub use neighbours::*;
use rand::{SeedableRng, rngs::StdRng};

use crate::{
    assets::{
        ImageAssets,
        map_tileset::{self, MapTilesetHandles, PowerUpTileType},
        material::ColouringMaterial,
    },
    constants::{TOTAL_MAP_HEIGHT, TOTAL_MAP_WIDTH},
    game_state::STARTS_PLAYING,
    map::explosion::{ExplosionResult, ExplosionVisual},
    position::TilePosition,
    rendering::MeshHandle,
    sound::EffectKind,
    world_entities::{
        AllEnemiesKilledEvent, Bomb, Bonus, BonusType, DespawnOnMainMenu, DestructibleWall,
        Explosion, ExplosionNeedsSetup, GameplaySet, MarkToDespawn, SpawnEnemiesMessage,
        SpawnSystemSet,
    },
};

const RND_SEED: u64 = 123456789;

fn map_power_up(power_up_tile_type: BonusType) -> PowerUpTileType {
    match power_up_tile_type {
        BonusType::Range => PowerUpTileType::Range,
        BonusType::BombCount => PowerUpTileType::BombCount,
        BonusType::Negative => PowerUpTileType::Negative,
        BonusType::ExtraLife => PowerUpTileType::ExtraLife,
        BonusType::Hook => PowerUpTileType::Hook,
        BonusType::BombKick => PowerUpTileType::BombKick,
        BonusType::Detonator => PowerUpTileType::Detonator,
        BonusType::Turbo => PowerUpTileType::Turbo,
        BonusType::LineBomb => PowerUpTileType::LineBomb,
        BonusType::DoubleBomb => PowerUpTileType::DoubleBomb,
        BonusType::Max => PowerUpTileType::Max,
    }
}

#[derive(Resource, Debug, Clone)]
pub struct WorldMap {
    exit_gate_index: usize,
    tiles: Vec<MapTile>,
}

impl WorldMap {
    fn pos_to_index(pos: TilePosition) -> Option<usize> {
        let pos_x = pos.x as usize;
        let pos_y = pos.y as usize;
        if pos_x >= TOTAL_MAP_WIDTH || pos_y >= TOTAL_MAP_HEIGHT {
            return None;
        }
        Some(pos_y * TOTAL_MAP_WIDTH + pos_x)
    }

    pub fn get_tile(&self, pos: TilePosition) -> Option<&MapTile> {
        self.tiles.get(WorldMap::pos_to_index(pos)?)
    }

    fn get_tile_mut(&mut self, pos: TilePosition) -> Option<&mut MapTile> {
        self.tiles.get_mut(WorldMap::pos_to_index(pos)?)
    }

    pub fn index_to_tile_pos(index: usize) -> TilePosition {
        TilePosition(UVec2 {
            x: (index % TOTAL_MAP_WIDTH) as u32,
            y: (index / TOTAL_MAP_WIDTH) as u32,
        })
    }

    pub fn get_player_spawning_location() -> TilePosition {
        TilePosition(UVec2 {
            x: 1,
            y: (TOTAL_MAP_HEIGHT - 2) as u32,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (TilePosition, &MapTile)> {
        self.tiles
            .iter()
            .enumerate()
            .map(|(index, tile)| (Self::index_to_tile_pos(index), tile))
    }

    pub fn iter_empty_non_starting_tiles(&self) -> impl Iterator<Item = TilePosition> + '_ {
        let starting_area_indices = Self::get_starting_area_indices();
        self.tiles.iter().enumerate().filter_map(move |(i, t)| {
            (t.is_ai_walkable() && !starting_area_indices.contains(&i))
                .then(|| Self::index_to_tile_pos(i))
        })
    }

    pub fn process_tick(&mut self, delta: Duration) {
        for tile in self.tiles.iter_mut() {
            tile.tick(delta);
        }
    }

    pub fn remove_bonus(&mut self, pos: TilePosition) -> Option<BonusType> {
        if let Some(tile) = self.get_tile_mut(pos) {
            tile.remove_bonus()
        } else {
            None
        }
    }

    pub fn try_add_bomb(&mut self, pos: TilePosition, bomb_tile: BombTile) -> bool {
        if let Some(tile) = self.get_tile_mut(pos) {
            tile.try_add_bomb(bomb_tile)
        } else {
            false
        }
    }

    pub fn open_exit(&mut self) {
        self.tiles[self.exit_gate_index].open_exit();
    }

    pub fn open_exit_position(&self) -> Option<TilePosition> {
        self.tiles[self.exit_gate_index].special().and_then(|s| {
            s.is_open_exit()
                .then(|| Self::index_to_tile_pos(self.exit_gate_index))
        })
    }
}

#[derive(Component)]
pub struct MapTileComponent;

#[derive(Component)]
struct ClosedGateTile;

fn setup_map(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut material: ResMut<Assets<ColouringMaterial>>,
    mesh_handle: Res<MeshHandle>,
) {
    // Generate map
    let mut rng_gen = StdRng::seed_from_u64(RND_SEED);
    let map = WorldMap::new_random_default(&mut rng_gen);

    // Prepare materials and rendering stuff
    let MapTilesetHandles {
        floor: floor_tilemap_handles,
        basic: basic_tilemap_handles,
        non_standard: non_standard_tilemap_handles,
        bonuses: bonuses_tilemap_handles,
    } = map_tileset::prepare_tilemap_material(&image_assets, &mut material);
    let Some(floor_colouring_material) = material.get(&floor_tilemap_handles.0) else {
        return;
    };
    let Some(basic_colouring_material) = material.get(&basic_tilemap_handles.0) else {
        return;
    };
    let Some(non_standard_colouring_material) = material.get(&non_standard_tilemap_handles.0)
    else {
        return;
    };
    let Some(bonuses_colouring_material) = material.get(&bonuses_tilemap_handles.0) else {
        return;
    };

    let mut indestructible_wall_material = basic_colouring_material.clone();
    indestructible_wall_material.set_uv_rect(
        map_tileset::BASIC_TILEMAP.sprite_uv_rect(map_tileset::MapTileType::IndestructibleWall),
    );
    let mut floor_material = floor_colouring_material.clone();
    floor_material
        .set_uv_rect(map_tileset::BASIC_TILEMAP.sprite_uv_rect(map_tileset::MapTileType::Floor));

    let mut wall_material = basic_colouring_material.clone();
    wall_material
        .set_uv_rect(map_tileset::BASIC_TILEMAP.sprite_uv_rect(map_tileset::MapTileType::Wall));
    let mut closed_gate_material = non_standard_colouring_material.clone();
    closed_gate_material.set_uv_rect(
        map_tileset::NON_STANDARD_TILEMAP.sprite_uv_rect(map_tileset::MapGateTileType::Closed),
    );
    let mut open_gate_material = non_standard_colouring_material.clone();
    open_gate_material.set_uv_rect(
        map_tileset::NON_STANDARD_TILEMAP.sprite_uv_rect(map_tileset::MapGateTileType::Open),
    );
    let mut bonuses_material = bonuses_colouring_material.clone();
    bonuses_material.set_uv_rect(
        map_tileset::BONUSES_TILEMAP.sprite_uv_rect(map_tileset::PowerUpTileType::Range),
    );

    // These are never changed
    let floor_material = material.add(floor_material);
    let indestructible_wall_material = material.add(indestructible_wall_material);

    for (position, tile) in map.iter() {
        match tile.base_type() {
            BaseTile::Floor => {
                commands.spawn((
                    MapTileComponent,
                    DespawnOnMainMenu,
                    Mesh2d(mesh_handle.0.clone()),
                    MeshMaterial2d(floor_material.clone()),
                    position,
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ));
            }
            BaseTile::IndestructibleWall => {
                commands.spawn((
                    MapTileComponent,
                    DespawnOnMainMenu,
                    Mesh2d(mesh_handle.0.clone()),
                    MeshMaterial2d(indestructible_wall_material.clone()),
                    position,
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ));
            }
            BaseTile::BasicWall | BaseTile::BreakingWall(_) => {
                // Every wall is handled by spawning both floor and wall. Wall can be destroyed but
                // floor remains.
                commands.spawn((
                    MapTileComponent,
                    DespawnOnMainMenu,
                    Mesh2d(mesh_handle.0.clone()),
                    MeshMaterial2d(floor_material.clone()),
                    position,
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ));
                commands.spawn((
                    MapTileComponent,
                    DespawnOnMainMenu,
                    DestructibleWall,
                    Mesh2d(mesh_handle.0.clone()),
                    MeshMaterial2d(material.add(wall_material.clone())),
                    position,
                    Transform::from_xyz(0.0, 0.0, 0.1),
                ));
            }
        };
        match tile.special() {
            Some(SpecialTile::ClosedExit) | Some(SpecialTile::OpenExit) => {
                // Open exit should never happen since creating map never creates it but is here for
                // completeness
                commands.spawn((
                    MapTileComponent,
                    DespawnOnMainMenu,
                    ClosedGateTile,
                    Mesh2d(mesh_handle.0.clone()),
                    MeshMaterial2d(material.add(closed_gate_material.clone())),
                    position,
                    Transform::from_xyz(0.0, 0.0, 0.04),
                ));
                commands.spawn((
                    MapTileComponent,
                    DespawnOnMainMenu,
                    Mesh2d(mesh_handle.0.clone()),
                    MeshMaterial2d(material.add(open_gate_material.clone())),
                    position,
                    Transform::from_xyz(0.0, 0.0, 0.02),
                ));
            }
            Some(SpecialTile::Bonus(bonus)) => {
                let mut bonuses_material = bonuses_material.clone();
                bonuses_material
                    .set_uv_rect(map_tileset::BONUSES_TILEMAP.sprite_uv_rect(map_power_up(*bonus)));
                commands.spawn((
                    MapTileComponent,
                    DespawnOnMainMenu,
                    Bonus,
                    *bonus,
                    Mesh2d(mesh_handle.0.clone()),
                    MeshMaterial2d(material.add(bonuses_material)),
                    position,
                    Transform::from_xyz(0.0, 0.0, 0.02),
                ));
            }
            None => {}
        };
    }

    commands.insert_resource(map);
}

fn on_all_enemies_killed(
    _: On<AllEnemiesKilledEvent>,
    mut commands: Commands,
    closed_gate_tiles: Query<Entity, With<ClosedGateTile>>,
    mut map: ResMut<WorldMap>,
) {
    for entity in closed_gate_tiles {
        commands.entity(entity).despawn();
    }
    map.open_exit();
}

fn process_map_in_tick(
    mut commands: Commands,
    mut spawn_event: MessageWriter<SpawnEnemiesMessage>,
    mut map: ResMut<WorldMap>,
    time: Res<Time<Fixed>>,
) {
    map.process_tick(time.delta());

    let ExplosionResult {
        visuals,
        punish_tiles,
    } = map.explode_bombs();
    for ExplosionVisual { variant, pos } in visuals {
        commands.spawn((Explosion, variant, pos, ExplosionNeedsSetup));
        commands.trigger(EffectKind::Explosion);
    }
    for message in punish_tiles {
        spawn_event.write(message);
    }

    for tile in map.tiles.iter_mut() {
        tile.convert_expired_entities();
    }
}

fn clean_map_removed_entities(
    mut commands: Commands,
    map: Res<WorldMap>,
    bombs: Query<(Entity, &TilePosition), (With<Bomb>, Without<Bonus>)>,
    bonuses: Query<(Entity, &TilePosition), (With<Bonus>, Without<Bomb>)>,
) {
    for (entity, position) in bombs {
        if let Some(tile) = map.get_tile(*position)
            && tile
                .bomb_or_explosion()
                .map(|v| v.is_explosion())
                .unwrap_or(true)
        {
            commands.entity(entity).insert(MarkToDespawn);
        }
    }
    for (entity, position) in bonuses {
        if let Some(tile) = map.get_tile(*position)
            && tile.special().is_none()
        {
            commands.entity(entity).despawn();
        }
    }
}

pub struct Map;

impl Plugin for Map {
    fn build(&self, app: &mut App) {
        app.init_resource::<ImageAssets>()
            .add_observer(on_all_enemies_killed)
            .add_systems(STARTS_PLAYING, setup_map.in_set(SpawnSystemSet::CreateMap))
            .add_systems(
                FixedUpdate,
                process_map_in_tick.in_set(GameplaySet::MapTickUpdate),
            )
            .add_systems(
                Update,
                clean_map_removed_entities.in_set(GameplaySet::MapToVisualsSync),
            );
    }
}
