use std::ops::Mul;

use bevy::prelude::*;

use crate::{
    character::MovementDirection,
    controls::{Controls, Direction},
    map::CollisionMap,
    position::WorldPosition,
    world_entities::{Character, MapTileMarker},
};

pub const CHARACTER_SPEED: f32 = 2.0;
const BORDER_PASSING: f32 = 0.6666;

const CV_ROTATION_MATRIX: Mat2 = Mat2::from_cols_array(&[0.0, -1.0, 1.0, 0.0]);

fn get_direction(direction: Vec2) -> Option<Direction> {
    if direction.x.abs() > direction.y.abs() {
        if direction.x > 0.0 {
            Some(Direction::Right)
        } else {
            Some(Direction::Left)
        }
    } else if direction.y.abs() > direction.x.abs() {
        if direction.y > 0.0 {
            Some(Direction::Up)
        } else {
            Some(Direction::Down)
        }
    } else {
        None
    }
}

fn move_in_step(
    character_position: &mut WorldPosition,
    movement_dir: &mut MovementDirection,
    direction: Direction,
    delta_secs: f32,
    collision_map: &CollisionMap,
) {
    let step_distance = CHARACTER_SPEED * delta_secs;
    // There is a small trick. We check tile left of the character. The position is shifted by 0.5
    // to account for character size. Then we also need to check slightly behind that tile -
    // `step_distance` is added to check against the tile we are moving into. Then it is multiplied
    // by 1.5 to ensure that moving character by `step_distance` will never result in character
    // being stuck in wall due to rounding errors.
    let offset_distance = 0.5 + step_distance * 1.5;
    let move_dir = match direction {
        Direction::Left => Vec2::new(-1.0, 0.0),
        Direction::Up => Vec2::new(0.0, 1.0),
        Direction::Right => Vec2::new(1.0, 0.0),
        Direction::Down => Vec2::new(0.0, -1.0),
    };
    let cv_dir = CV_ROTATION_MATRIX * move_dir;

    let tile_move_dir_offset = character_position.0 + move_dir * offset_distance;
    let tile_perp_dir_offset = cv_dir * 0.5;

    let cv_tile =
        collision_map.get_tile_at_position(&(tile_move_dir_offset + tile_perp_dir_offset).into());
    let ccv_tile =
        collision_map.get_tile_at_position(&(tile_move_dir_offset - tile_perp_dir_offset).into());

    // Next both of those tiles are Some. Otherwise we are out of map - undefined behaviour.
    if let (Some(cv_tile), Some(ccv_tile)) = (cv_tile, ccv_tile) {
        if cv_tile.marker == MapTileMarker::Walkable && ccv_tile.marker == MapTileMarker::Walkable {
            // If both tiles are walkable, we can move freely.
            character_position.0 += move_dir * step_distance;
            movement_dir.0 = Some(direction);
        } else {
            // Now we have to solve multiple cases. Now we solve if the character position qualifies
            // for sliding along the wall to fit into the walkable tile. And also a case where the
            // character is standing on top of an obstacle (placed bomb) and can still move on top
            // of it until he leaves its tile.
            if let Some((perp_dir_to_walkable, perp_dist_to_walkable, cv_dir_sign)) = {
                // Check if the character is eligible for sliding along the wall.
                if cv_tile.marker == MapTileMarker::Obstacle
                    && ccv_tile.marker == MapTileMarker::Obstacle
                {
                    // Both tiles are obstacles - no sliding.
                    None
                } else {
                    let (walkable, cv_dir_sign) = if cv_tile.marker == MapTileMarker::Walkable {
                        (&cv_tile, 1.0)
                    } else {
                        (&ccv_tile, -1.0)
                    };
                    // Compute the perpendicular distance to the walkable tile - multiplying with
                    // perpendicular unit vector eliminates the component in the directions of the
                    // movement, leaving only the perpendicular component.
                    let perp_dir_to_walkable =
                        (walkable.world_pos().0 - character_position.0).mul(cv_dir.abs());
                    let perp_dist_to_walkable = perp_dir_to_walkable.length();
                    if perp_dist_to_walkable < BORDER_PASSING {
                        // The character is close enough to slide along the wall.
                        Some((perp_dir_to_walkable, perp_dist_to_walkable, cv_dir_sign))
                    } else {
                        // The character is too far from the walkable tile to slide along the wall.
                        None
                    }
                }
            } {
                // In this branch we know that the character is eligible for sliding along the wall.
                if perp_dist_to_walkable < step_distance {
                    // Slide the character along the wall until it is aligned with the walkable tile
                    character_position.0 += perp_dir_to_walkable;
                    // Move the character for the rest of the step distance in the desired direction
                    character_position.0 += move_dir * (step_distance - perp_dist_to_walkable);
                    movement_dir.0 = Some(direction);
                } else {
                    // Character has to slide along the wall for the whole step distance.
                    character_position.0 += cv_dir * cv_dir_sign * step_distance;
                    movement_dir.0 = get_direction(perp_dir_to_walkable);
                }
            } else {
                // If both tiles are obstacles or character is not eligible for sliding, we check
                // whether the character is on one of those tiles. If true he can freely walk on top
                // of that tile. This is the case for placed bomb.
                let character_tile = collision_map.get_tile_at_position(character_position);
                if let Some(character_tile) = character_tile
                    && (character_tile == cv_tile || character_tile == ccv_tile)
                {
                    character_position.0 += move_dir * step_distance;
                    movement_dir.0 = Some(direction);
                } else {
                    // Character is block by wall. Do not move him.
                    movement_dir.0 = None;
                }
            }
        }
    } else {
        // If one of the tiles is None, it means we are at the edge of the map, so we can
        // move freely
        character_position.0 += move_dir * step_distance;
        movement_dir.0 = Some(direction);
    }
}

pub fn move_character(
    mut characters: Query<(&mut WorldPosition, &mut MovementDirection), With<Character>>,
    controls: Res<Controls>,
    time: Res<Time>,
    collision_map: Res<CollisionMap>,
) {
    let elapsed = time.delta_secs();

    for (mut world_position, mut movement_dir) in characters.iter_mut() {
        let desired_movement = controls.into_movement();

        if let Some(direction) = desired_movement {
            move_in_step(
                &mut world_position, &mut movement_dir, direction, elapsed, &collision_map,
            );
        } else {
            movement_dir.0 = None;
        }
    }
}
