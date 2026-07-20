use std::time::Duration;

use bevy::prelude::*;

use crate::{
    game_state::GameState,
    map::WorldMap,
    position::WorldPosition,
    sound::EffectKind,
    world_entities::{
        ActorState, AllEnemiesKilled, AllEnemiesKilledEvent, Character, Enemy, GameplaySet,
        Killable,
    },
};

const KILL_DISTANCE_THRESHOLD: f32 = 0.75;

const ENEMY_KILL_DISTANCE: f32 = 0.75;

const ENEMY_DEATH_DURATION: Duration = Duration::from_secs(1);
const CHARACTER_DEATH_DURATION: Duration = Duration::from_secs(3);

fn manhattan_distance(pos1: Vec2, pos2: Vec2) -> f32 {
    (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()
}

fn check_kill_from_explosion(world_position: WorldPosition, world_map: &WorldMap) -> bool {
    world_map
        .world_position_neighbours(world_position)
        .map(|n| {
            n.iter()
                .map(|n| {
                    n.tile
                        .bomb_or_explosion()
                        .is_some_and(|v| v.is_explosion())
                        .then(|| manhattan_distance(n.pos.to_world_position().0, world_position.0))
                })
                .fold(f32::INFINITY, |acc, x| acc.min(x.unwrap_or(f32::INFINITY)))
                < KILL_DISTANCE_THRESHOLD
        })
        .unwrap_or(false)
}

fn check_explosion_entity_kills(
    mut commands: Commands,
    non_characters: Query<(&WorldPosition, &mut ActorState), (With<Killable>, Without<Character>)>,
    character: Query<(&WorldPosition, &mut ActorState), (With<Killable>, With<Character>)>,
    world_map: Res<WorldMap>,
) {
    for (world_position, mut state) in non_characters {
        if matches!(state.as_ref(), ActorState::Alive) {
            if check_kill_from_explosion(*world_position, &world_map) {
                *state = ActorState::Dying(Timer::new(ENEMY_DEATH_DURATION, TimerMode::Once));
                commands.trigger(EffectKind::EnemyDeath);
            }
        }
    }
    for (world_position, mut state) in character {
        if matches!(state.as_ref(), ActorState::Alive) {
            if check_kill_from_explosion(*world_position, &world_map) {
                *state = ActorState::Dying(Timer::new(CHARACTER_DEATH_DURATION, TimerMode::Once));
                commands.trigger(EffectKind::CharacterDeath);
            }
        }
    }
}

fn kill_character_near_enemy(
    mut commands: Commands,
    enemies: Query<&WorldPosition, With<Enemy>>,
    mut characters: Query<(&WorldPosition, &mut ActorState), With<Character>>,
) {
    for enemy_pos in enemies {
        // Only one character is expected at a time. No optimization needed
        for (character_pos, mut state) in characters.iter_mut() {
            if matches!(state.as_ref(), ActorState::Alive) {
                if enemy_pos.0.distance(character_pos.0) < ENEMY_KILL_DISTANCE {
                    *state =
                        ActorState::Dying(Timer::new(CHARACTER_DEATH_DURATION, TimerMode::Once));
                    commands.trigger(EffectKind::CharacterDeath);
                }
            }
        }
    }
}

fn advance_death_timers(
    non_characters: Query<&mut ActorState, Without<Character>>,
    characters: Query<&mut ActorState, With<Character>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let time_delta = time.delta();
    for mut state in non_characters {
        if let ActorState::Dying(timer) = state.as_mut() {
            timer.tick(time_delta);
        }
    }

    for mut state in characters {
        if let ActorState::Dying(timer) = state.as_mut() {
            timer.tick(time_delta);
            if timer.is_finished() {
                next_state.set(GameState::MainMenu);
            }
        }
    }
}

fn end_game_on_death(
    characters: Query<&ActorState, With<Character>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for death_timer in characters {
        if let ActorState::Dying(timer) = death_timer {
            if timer.is_finished() {
                next_state.set(GameState::MainMenu);
            }
        }
    }
}

fn despawn_killed_enemies(
    mut commands: Commands,
    enemies: Query<(Entity, &ActorState), With<Enemy>>,
) {
    for (entity, death_timer) in enemies.iter() {
        if let ActorState::Dying(timer) = death_timer {
            if timer.is_finished() {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn check_alive_enemies(
    mut commands: Commands,
    all_enemies_killed: Option<Res<AllEnemiesKilled>>,
    enemies: Query<(), With<Enemy>>,
) {
    if all_enemies_killed.is_none() && enemies.is_empty() {
        commands.trigger(AllEnemiesKilledEvent);
        commands.insert_resource(AllEnemiesKilled);
    }
}

pub struct DeathPlugin;

impl Plugin for DeathPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                check_explosion_entity_kills,
                kill_character_near_enemy,
                (
                    advance_death_timers,
                    (end_game_on_death, despawn_killed_enemies),
                    check_alive_enemies,
                )
                    .chain(),
            )
                .in_set(GameplaySet::DeathAndVictory),
        );
    }
}
