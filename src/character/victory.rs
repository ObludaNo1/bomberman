use std::time::Duration;

use bevy::prelude::*;

use crate::{
    animation::MovementDirection,
    death::DeathTimer,
    game_state::GameState,
    map::WorldMap,
    position::WorldPosition,
    sound::EffectKind,
    util::RenderScale,
    world_entities::{Character, Direction, MovementSpeed},
};

const WINNING_TRIGGER_DISTANCE: f32 = 0.5;
const VICTORY_ANIMATION_DURATION: Duration = Duration::from_secs(3);

fn manhattan_distance(pos1: WorldPosition, pos2: WorldPosition) -> f32 {
    (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()
}

#[derive(Component, Deref, DerefMut)]
pub struct VictoryTimer(pub Timer);

pub fn check_for_win(
    mut commands: Commands,
    characters: Query<(Entity, &WorldPosition), (With<Character>, Without<VictoryTimer>)>,
    world_map: Res<WorldMap>,
) {
    for (entity, player_pos) in characters {
        if let Some(open_exit_position) = world_map.open_exit_position() {
            if manhattan_distance(*player_pos, open_exit_position.to_world_position())
                < WINNING_TRIGGER_DISTANCE
            {
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
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut characters: Query<
        (
            &mut WorldPosition,
            &mut MovementDirection,
            &mut VictoryTimer,
            &mut RenderScale,
            &MovementSpeed,
        ),
        (With<Character>, Without<DeathTimer>),
    >,
    world_map: Res<WorldMap>,
    time: Res<Time<Fixed>>,
) {
    let delta_time = time.delta();

    let Some(gate_pos) = world_map.open_exit_position() else {
        return;
    };
    let gate_pos = gate_pos.to_world_position();

    for (
        mut world_position,
        mut animation_dir,
        mut victory_timer,
        mut render_scale,
        movement_speed,
    ) in characters.iter_mut()
    {
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
            if victory_timer.0.elapsed() == Duration::ZERO {
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
