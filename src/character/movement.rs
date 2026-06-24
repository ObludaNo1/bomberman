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
    let check_position: WorldPosition = Vec2::new(
        character_position.x + direction.horizontal_movement(),
        character_position.y + direction.vertical_movement(),
    )
    .into();

    let step_distance = CHARACTER_SPEED * delta_secs;

    let (cv_tile, ccv_tile, move_dir) = match direction {
        Direction::Left => {
            // There is a small trick. We check tile left of the character. The position is
            // shifted by 0.5 to account for character size. Then we also need to check slightly
            // behind that tile - `step_distance` is added to check against the tile we are
            // moving into. Then it is multiplied by 1.5 to ensure that moving character by
            // `step_distance` will never result in character being stuck in wall due to
            // rounding errors.
            let x_position_to_check = character_position.x - 0.5 - step_distance * 1.5;
            (
                collision_map.get_tile_at_position(
                    &Vec2::new(x_position_to_check, check_position.y + 0.5).into(),
                ),
                collision_map.get_tile_at_position(
                    &Vec2::new(x_position_to_check, check_position.y - 0.5).into(),
                ),
                Vec2::new(-1.0, 0.0),
            )
        }
        Direction::Up => {
            let y_position_to_check = character_position.y + 0.5 + step_distance * 1.5;
            (
                collision_map.get_tile_at_position(
                    &Vec2::new(check_position.x + 0.5, y_position_to_check).into(),
                ),
                collision_map.get_tile_at_position(
                    &Vec2::new(check_position.x - 0.5, y_position_to_check).into(),
                ),
                Vec2::new(0.0, 1.0),
            )
        }
        Direction::Right => {
            let x_position_to_check = character_position.x + 0.5 + step_distance * 1.5;
            (
                collision_map.get_tile_at_position(
                    &Vec2::new(x_position_to_check, check_position.y - 0.5).into(),
                ),
                collision_map.get_tile_at_position(
                    &Vec2::new(x_position_to_check, check_position.y + 0.5).into(),
                ),
                Vec2::new(1.0, 0.0),
            )
        }
        Direction::Down => {
            let y_position_to_check = character_position.y - 0.5 - step_distance * 1.5;
            (
                collision_map.get_tile_at_position(
                    &Vec2::new(check_position.x - 0.5, y_position_to_check).into(),
                ),
                collision_map.get_tile_at_position(
                    &Vec2::new(check_position.x + 0.5, y_position_to_check).into(),
                ),
                Vec2::new(0.0, -1.0),
            )
        }
    };

    let cv_dir = CV_ROTATION_MATRIX * move_dir;
    let ccv_dir = -(CV_ROTATION_MATRIX * move_dir);

    // Next both of those tiles are Some. Otherwise we are out of map - undefined behaviour.
    if let (Some(cv_tile), Some(ccv_tile)) = (cv_tile, ccv_tile) {
        // If both tiles are walkable, we can move freely.
        if cv_tile.marker == MapTileMarker::Walkable && ccv_tile.marker == MapTileMarker::Walkable {
            character_position.0 += move_dir * step_distance;
            movement_dir.0 = Some(direction);
        } else if cv_tile.marker == MapTileMarker::Walkable
            && ccv_tile.marker == MapTileMarker::Obstacle
            && ((cv_tile.world_pos().0 - character_position.0).mul(cv_dir)).length()
                < BORDER_PASSING
        {
            let needed_perpendicular_direction =
                (character_position.0 - cv_tile.world_pos().0).mul(-cv_dir.abs());
            let needed_perpendicular_distance = needed_perpendicular_direction.length();
            if needed_perpendicular_distance < step_distance {
                character_position.0 += needed_perpendicular_direction;
                character_position.0 += move_dir * (step_distance - needed_perpendicular_distance);
                movement_dir.0 = Some(direction);
            } else {
                character_position.0 += needed_perpendicular_direction.normalize() * step_distance;
                movement_dir.0 = get_direction(needed_perpendicular_direction);
            }
        } else if ccv_tile.marker == MapTileMarker::Walkable
            && cv_tile.marker == MapTileMarker::Obstacle
            && ((ccv_tile.world_pos().0 - character_position.0).mul(ccv_dir)).length()
                < BORDER_PASSING
        {
            let needed_perpendicular_direction =
                (ccv_tile.world_pos().0 - character_position.0).mul(cv_dir.abs());
            let needed_perpendicular_distance = needed_perpendicular_direction.length();
            if needed_perpendicular_distance < step_distance {
                character_position.0 += needed_perpendicular_direction;
                character_position.0 += move_dir * (step_distance - needed_perpendicular_distance);
                movement_dir.0 = Some(direction);
            } else {
                character_position.0 += needed_perpendicular_direction.normalize() * step_distance;
                movement_dir.0 = get_direction(needed_perpendicular_direction);
            }
        } else {
            // Character is block by wall. Do not move him.
            movement_dir.0 = None;
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
