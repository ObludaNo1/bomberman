use bevy::prelude::*;

use crate::{
    controls::{Controls, Direction},
    map::{CollisionMap, MAP_HEIGHT},
    position::WorldPosition,
    world_entities::{Character, MapTileMarker},
};

pub const CHARACTER_SPEED: f32 = 2.0;
const BORDER_PASSING: f32 = 0.6666;

fn tile_y_to_world_pos(y: usize) -> f32 {
    y as f32 - (MAP_HEIGHT - 1) as f32 * 0.5
}

fn move_in_step(
    character_position: &mut WorldPosition,
    direction: Direction,
    delta_secs: f32,
    collision_map: &CollisionMap,
) {
    let check_position: WorldPosition = Vec2::new(
        character_position.x + direction.horizontal_movement(),
        character_position.y + direction.vertical_movement(),
    )
    .into();

    let step_distance = CHARACTER_SPEED * delta_secs;

    match direction {
        Direction::Left => {
            // First check two tiles in direction of movement.

            // There is a small trick. We check tile left of the character. The position is shifted
            // by 0.5 to account for character size. Then we also need to check slightly behind that
            // tile - `step_distance` is added to check against the tile we are moving into. Then it
            // is multiplied by 1.5 to ensure that moving character by `step_distance` will never
            // result in character being stuck in wall due to rounding errors.
            let x_position_to_check = character_position.x - 0.5 - step_distance * 1.5;
            let tile_top = collision_map.get_tile_at_position(
                &Vec2::new(x_position_to_check, check_position.y + 0.5).into(),
            );
            let tile_bottom = collision_map.get_tile_at_position(
                &Vec2::new(x_position_to_check, check_position.y - 0.5).into(),
            );
            // Next both of those tiles are Some. Otherwise we are out of map - undefined behaviour.
            if let (Some(tile_top), Some(tile_bottom)) = (tile_top, tile_bottom) {
                // If both tiles are walkable, we can move freely.
                if tile_top.marker == MapTileMarker::Walkable
                    && tile_bottom.marker == MapTileMarker::Walkable
                {
                    character_position.x -= step_distance;
                } else if tile_top.marker == MapTileMarker::Walkable
                    && tile_bottom.marker == MapTileMarker::Obstacle
                    && (tile_y_to_world_pos(tile_top.y) - character_position.y) < BORDER_PASSING
                {
                    let needed_y_distance = tile_y_to_world_pos(tile_top.y) - character_position.y;
                    if needed_y_distance < step_distance {
                        character_position.y += needed_y_distance;
                        character_position.x -= step_distance - needed_y_distance;
                    } else {
                        character_position.y += step_distance;
                    }
                } else if tile_bottom.marker == MapTileMarker::Walkable
                    && tile_top.marker == MapTileMarker::Obstacle
                    && (character_position.y - tile_y_to_world_pos(tile_bottom.y)) < BORDER_PASSING
                {
                    let needed_y_distance =
                        character_position.y - tile_y_to_world_pos(tile_bottom.y);
                    if needed_y_distance < step_distance {
                        character_position.y -= needed_y_distance;
                        character_position.x -= step_distance - needed_y_distance;
                    } else {
                        character_position.y -= step_distance;
                    }
                } else {
                    // Character is block by wall. Do nothing for now.
                }
            } else {
                // If one of the tiles is None, it means we are at the edge of the map, so we can
                // move freely
                character_position.x -= step_distance;
            }
        }
        Direction::Right => {
            character_position.x += step_distance;
        }
        Direction::Up => {
            character_position.y += step_distance;
        }
        Direction::Down => {
            character_position.y -= step_distance;
        }
    }
}

pub fn move_character(
    mut characters: Query<&mut WorldPosition, With<Character>>,
    controls: Res<Controls>,
    time: Res<Time>,
    collision_map: Res<CollisionMap>,
) {
    let elapsed = time.delta_secs();

    for mut world_position in characters.iter_mut() {
        let desired_movement = controls.into_movement();

        if let Some(direction) = desired_movement {
            move_in_step(&mut world_position, direction, elapsed, &collision_map);
        }
    }
}
