use std::time::Duration;

use bevy::prelude::*;

use crate::{
    animation::MovementDirection,
    character::movement::CHARACTER_SPEED,
    death::DeathTimer,
    game_state::GameState,
    position::WorldPosition,
    util::RenderScale,
    world_entities::{Character, Direction, ExitGate},
};

const WINNING_TRIGGER_DISTANCE: f32 = 0.5;
const VICTORY_ANIMATION_DURATION: Duration = Duration::from_secs(3);

fn manhattan_distance(pos1: &WorldPosition, pos2: &WorldPosition) -> f32 {
    (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()
}

#[derive(Component, Deref, DerefMut)]
pub struct VictoryTimer(pub Timer);

pub fn check_for_win(
    mut commands: Commands,
    characters: Query<(Entity, &WorldPosition), (With<Character>, Without<VictoryTimer>)>,
    exit_gates: Query<&WorldPosition, With<ExitGate>>,
) {
    for (entity, player_pos) in characters {
        for gate_pos in exit_gates {
            if manhattan_distance(player_pos, gate_pos) < WINNING_TRIGGER_DISTANCE {
                commands.entity(entity).insert((
                    VictoryTimer(Timer::new(VICTORY_ANIMATION_DURATION, TimerMode::Once)),
                    RenderScale(1.0),
                ));
                break;
            }
        }
    }
}

pub fn victory_ending(
    mut next_state: ResMut<NextState<GameState>>,
    mut characters: Query<
        (
            &mut WorldPosition,
            &mut MovementDirection,
            &mut VictoryTimer,
            &mut RenderScale,
        ),
        (With<Character>, Without<DeathTimer>),
    >,
    gate: Query<&WorldPosition, (Without<Character>, With<ExitGate>)>,
    time: Res<Time<Fixed>>,
) {
    let delta_time = time.delta();

    let Ok(gate_pos) = gate.single() else {
        return;
    };

    for (mut world_position, mut animation_dir, mut victory_timer, mut render_scale) in
        characters.iter_mut()
    {
        let dir = gate_pos.0 - world_position.0;
        let len = dir.length();
        let step_distance = delta_time.as_secs_f32() * CHARACTER_SPEED;
        if step_distance <= len {
            let norm = dir.normalize_or_zero();
            world_position.0 += norm * step_distance;
            animation_dir.0 = Direction::from_vec2(norm);
        } else {
            world_position.0 = gate_pos.0;
            let remaining_step_duration = delta_time.as_secs_f32() * (1.0 - len / step_distance);
            victory_timer.tick(Duration::from_secs_f32(remaining_step_duration));
            animation_dir.0 = Some(Direction::Up);
            render_scale.0 = 1.0 - victory_timer.fraction();
            if victory_timer.is_finished() {
                next_state.set(GameState::Win);
            }
        }
    }
}
