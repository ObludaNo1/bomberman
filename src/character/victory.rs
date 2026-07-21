use std::time::Duration;

use bevy::prelude::*;

use crate::{
    animation::MovementDirection,
    game_state::GameState,
    map::WorldMap,
    position::WorldPosition,
    sound::EffectKind,
    util::EntityScale,
    world_entities::{ActorState, Character, Direction, MovementSpeed},
};

const WINNING_TRIGGER_DISTANCE: f32 = 0.5;
const VICTORY_ANIMATION_DURATION: Duration = Duration::from_secs(3);

fn manhattan_distance(pos1: WorldPosition, pos2: WorldPosition) -> f32 {
    (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()
}

pub fn check_for_win(
    characters: Query<(&WorldPosition, &mut ActorState), With<Character>>,
    world_map: Res<WorldMap>,
) {
    for (player_pos, mut state) in characters {
        if matches!(state.as_ref(), ActorState::Alive) {
            if let Some(open_exit_position) = world_map.open_exit_position() {
                if manhattan_distance(*player_pos, open_exit_position.to_world_position())
                    < WINNING_TRIGGER_DISTANCE
                {
                    *state = ActorState::Victory(Timer::new(
                        VICTORY_ANIMATION_DURATION,
                        TimerMode::Once,
                    ));
                    break;
                }
            }
        }
    }
}

pub fn victory_ending(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut characters: Query<
        (
            &mut WorldPosition,
            &mut MovementDirection,
            &mut ActorState,
            &mut EntityScale,
            &MovementSpeed,
        ),
        With<Character>,
    >,
    world_map: Res<WorldMap>,
    time: Res<Time<Fixed>>,
) {
    let delta_time = time.delta();

    let Some(gate_pos) = world_map.open_exit_position() else {
        return;
    };
    let gate_pos = gate_pos.to_world_position();

    for (mut world_position, mut animation_dir, mut state, mut render_scale, movement_speed) in
        characters.iter_mut()
    {
        let ActorState::Victory(victory_timer) = state.as_mut() else {
            continue;
        };

        let dir = gate_pos.0 - world_position.0;
        let len = dir.length();
        let step_distance = delta_time.as_secs_f32() * movement_speed.0;
        if step_distance <= len {
            let norm = dir.normalize_or_zero();
            world_position.0 += norm * step_distance;
            animation_dir.0 = Direction::from_vec2(norm);
        } else {
            world_position.0 = gate_pos.0;
            let remaining_step_duration = delta_time.as_secs_f32() * (1.0 - len / step_distance);
            if victory_timer.elapsed() == Duration::ZERO {
                commands.trigger(EffectKind::Victory);
            }
            victory_timer.tick(Duration::from_secs_f32(remaining_step_duration));
            animation_dir.0 = Some(Direction::Up);
            render_scale.0 = 1.0 - victory_timer.fraction();
            if victory_timer.is_finished() {
                next_state.set(GameState::Win);
            }
        }
    }
}
