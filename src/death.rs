use bevy::prelude::*;

use crate::{
    game_state::GameState,
    map::WorldMap,
    position::WorldPosition,
    world_entities::{Character, Enemy, GameplaySet, Killable, MapTileMarker},
};

const KILL_DISTANCE_THRESHOLD: f32 = 0.75;

fn manhattan_distance(pos1: &Vec2, pos2: &Vec2) -> f32 {
    (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()
}

fn check_kill_from_explosion(world_position: &WorldPosition, world_map: &WorldMap) -> bool {
    let nearest_explosion_distance = [
        Vec2::new(-0.5, 0.5),
        Vec2::new(0.5, 0.5),
        Vec2::new(-0.5, -0.5),
        Vec2::new(0.5, -0.5),
    ]
    .map(|offset| {
        world_map
            .get_tile_at_position(&WorldPosition(world_position.0 + offset))
            .and_then(|tile| {
                (tile.marker == MapTileMarker::Explosion)
                    .then(|| manhattan_distance(&tile.world_pos().0, &world_position.0))
            })
    })
    .iter()
    .fold(f32::INFINITY, |acc, x| acc.min(x.unwrap_or(f32::INFINITY)));

    nearest_explosion_distance < KILL_DISTANCE_THRESHOLD
}

fn check_explosion_entity_kills(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    non_characters: Query<(Entity, &WorldPosition), (With<Killable>, Without<Character>)>,
    character: Query<&WorldPosition, (With<Killable>, With<Character>)>,
    world_map: Res<WorldMap>,
) {
    for (entity, world_position) in non_characters {
        if check_kill_from_explosion(world_position, &world_map) {
            commands.entity(entity).despawn();
        }
    }
    for world_position in character {
        if check_kill_from_explosion(world_position, &world_map) {
            println!("Character killed!");
            game_state.set(GameState::MainMenu);
        }
    }
}

const ENEMY_KILL_DISTANCE: f32 = 0.75;

pub fn kill_character_near_enemy(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    enemies: Query<&WorldPosition, With<Enemy>>,
    characters: Query<(Entity, &WorldPosition), With<Character>>,
) {
    for enemy_pos in enemies {
        // Only one character is expected at a time. No optimization needed
        for (entity, character_pos) in characters {
            if enemy_pos.0.distance(character_pos.0) < ENEMY_KILL_DISTANCE {
                commands.entity(entity).despawn();
                println!("Character killed!");
                game_state.set(GameState::MainMenu);
            }
        }
    }
}

pub struct DeathPlugin;

impl Plugin for DeathPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (check_explosion_entity_kills, kill_character_near_enemy).in_set(GameplaySet::Death),
        );
    }
}
