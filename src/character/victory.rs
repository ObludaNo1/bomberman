use bevy::prelude::*;

use crate::{
    game_state::GameState,
    position::WorldPosition,
    world_entities::{Character, ExitGate},
};

const WINNING_TRIGGER_DISTANCE: f32 = 0.5;

fn manhattan_distance(pos1: &WorldPosition, pos2: &WorldPosition) -> f32 {
    (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()
}

pub fn check_for_win(
    mut state: ResMut<NextState<GameState>>,
    characters: Query<&WorldPosition, With<Character>>,
    exit_gates: Query<&WorldPosition, With<ExitGate>>,
) {
    for player_pos in characters {
        for gate_pos in exit_gates {
            if manhattan_distance(player_pos, gate_pos) < WINNING_TRIGGER_DISTANCE {
                state.set(GameState::Win);
                return;
            }
        }
    }
}
