use bevy::prelude::*;
use rand::RngExt;

use crate::{
    animation::MovementDirection,
    death::DeathTimer,
    enemy::EnemyRngGen,
    map::WorldMap,
    position::WorldPosition,
    world_entities::{Direction, Enemy, MovementSpeed},
};

const CV_ROTATION_MATRIX: Mat2 = Mat2::from_cols_array(&[0.0, -1.0, 1.0, 0.0]);

const CHANCE_TO_TURN: f64 = 0.4;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnemyMovement {
    /// Desired position is always defined. If the enemy is not moving, it will be the same as its
    /// current position.
    pub desired_position: (usize, usize),
    /// A hint for choosing the next tile. With this enemy prefer to continue moving in the same
    /// direction or turn left/right. If None, the enemy will choose a random direction.
    pub last_direction: Option<Direction>,
}

impl EnemyMovement {
    pub fn new(desired_position: (usize, usize)) -> Self {
        Self {
            desired_position,
            last_direction: None,
        }
    }
}

fn keep_larger_component(v: Vec2) -> Vec2 {
    if v.x.abs() >= v.y.abs() {
        Vec2::new(v.x, 0.0)
    } else {
        Vec2::new(0.0, v.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MovementChoice {
    Straight,
    Left,
    Right,
    Return,
    Stop,
}

impl MovementChoice {
    fn to_direction(&self, current_dir: Vec2) -> Vec2 {
        match self {
            MovementChoice::Straight => current_dir,
            MovementChoice::Left => CV_ROTATION_MATRIX * current_dir,
            MovementChoice::Right => CV_ROTATION_MATRIX * current_dir * -1.0,
            MovementChoice::Return => -current_dir,
            MovementChoice::Stop => Vec2::ZERO,
        }
    }
}

fn next_tile_choices(rng_gen: &mut EnemyRngGen) -> [MovementChoice; 5] {
    let continue_straight = rng_gen.random_bool(1.0 - CHANCE_TO_TURN);
    if continue_straight {
        [
            MovementChoice::Straight,
            MovementChoice::Left,
            MovementChoice::Right,
            MovementChoice::Return,
            MovementChoice::Stop,
        ]
    } else {
        let turn_left = rng_gen.random_bool(0.5);
        if turn_left {
            [
                MovementChoice::Left,
                MovementChoice::Straight,
                MovementChoice::Right,
                MovementChoice::Return,
                MovementChoice::Stop,
            ]
        } else {
            [
                MovementChoice::Right,
                MovementChoice::Straight,
                MovementChoice::Left,
                MovementChoice::Return,
                MovementChoice::Stop,
            ]
        }
    }
}

fn random_direction(rng_gen: &mut EnemyRngGen) -> Vec2 {
    match rng_gen.random_range(0..=3) {
        0 => Vec2::new(1.0, 0.0),
        1 => Vec2::new(-1.0, 0.0),
        2 => Vec2::new(0.0, 1.0),
        _ => Vec2::new(0.0, -1.0),
    }
}

fn choose_next_tile(
    current_dir: &Vec2,
    position: &(usize, usize),
    collision_map: &WorldMap,
    rng_gen: &mut EnemyRngGen,
) -> (usize, usize) {
    let current_dir = current_dir
        .try_normalize()
        .map(keep_larger_component)
        .unwrap_or_else(|| random_direction(rng_gen));

    let (position_x, position_y) = (position.0 as isize, position.1 as isize);

    for tile_choice in next_tile_choices(rng_gen) {
        let next_dir = tile_choice.to_direction(current_dir);

        let (next_tile_dir_x, next_tile_dir_y) = if next_dir.x.abs() > next_dir.y.abs() {
            (next_dir.x.signum() as isize, 0)
        } else {
            (0, next_dir.y.signum() as isize)
        };
        let next_tile = (position_x + next_tile_dir_x, position_y + next_tile_dir_y);
        // Negative coordinates could only happen if the enemy is as the edge of the map, which
        // should be impossible, since there are indestructible walls there.
        let next_tile = (next_tile.0 as usize, next_tile.1 as usize);
        let next_tile = collision_map.get_tile(next_tile.0, next_tile.1);

        // We can keep moving in the same direction
        if let Some(next_tile) = next_tile
            && next_tile.marker.is_ai_walkable()
        {
            return (next_tile.x, next_tile.y);
        };
    }

    return *position;
}

fn move_enemy(
    position: &mut WorldPosition,
    animation: &mut MovementDirection,
    movement: &mut EnemyMovement,
    speed: &MovementSpeed,
    collision_map: &WorldMap,
    delta_secs: f32,
    enemy_rng_gen: &mut EnemyRngGen,
) {
    let mut step_distance = speed.0 * delta_secs;
    let desired_tile =
        collision_map.get_tile(movement.desired_position.0, movement.desired_position.1);
    if let Some(desired_tile) = desired_tile
        && desired_tile.marker.is_ai_walkable()
    {
        let mut movement_dir_unnormalized = desired_tile.world_pos().0 - position.0;
        let distance = movement_dir_unnormalized.length();
        if distance <= step_distance {
            // Enemy has reached the desired tile, so we can set its position to the desired tile
            position.0 = desired_tile.world_pos().0;
            let next_tile_dir = movement
                .last_direction
                .map(|dir| dir.to_vec2())
                .unwrap_or_else(|| movement_dir_unnormalized);

            let next_tile = choose_next_tile(
                &next_tile_dir,
                // enemy has already reached the desired tile, so we can use its coordinates as
                // the current position
                &movement.desired_position,
                collision_map,
                enemy_rng_gen,
            );

            movement.desired_position = (next_tile.0, next_tile.1);

            movement_dir_unnormalized = collision_map
                .get_tile(movement.desired_position.0, movement.desired_position.1)
                .map(|tile| tile.world_pos().0 - position.0)
                .unwrap_or(Vec2::ZERO);
            step_distance -= distance;

            // Now move the enemy by the remaining distance in one tick towards the next tile
        }

        position.0 += movement_dir_unnormalized.normalize_or_zero() * step_distance;
        movement.last_direction = Direction::from_vec2(movement_dir_unnormalized);
        animation.0 = movement.last_direction;
    } else {
        // If the desired tile is not empty, we need to choose a new desired tile.

        // This solution chooses next tile for the next tick, but the enemy will stay in the same
        // position for this tick. This is a simple solution that works well enough for now.

        // We get a tile closest to the enemy's current position. The only possibility that the
        // desired tile was not empty is either that it is out of the map (which should not happen)
        // or an explosion started on that tile. In the second case an enemy is either close to the
        // explosion and dies anyway or it will be the previous tile.
        let new_tile = collision_map.get_position_from_world(position);
        movement.desired_position =
            choose_next_tile(&Vec2::ZERO, &new_tile, collision_map, enemy_rng_gen);
        movement.last_direction = None;
    }
}

pub fn move_enemies(
    mut enemies: Query<
        (
            &mut WorldPosition,
            &mut MovementDirection,
            &mut EnemyMovement,
            &MovementSpeed,
        ),
        (With<Enemy>, Without<DeathTimer>),
    >,
    collision_map: Res<WorldMap>,
    time: Res<Time<Fixed>>,
    mut enemy_rng_gen: ResMut<EnemyRngGen>,
) {
    let delta_secs = time.delta_secs();

    for (mut position, mut animation_dir, mut movement, speed) in enemies.iter_mut() {
        move_enemy(
            &mut position, &mut animation_dir, &mut movement, speed, &collision_map, delta_secs,
            &mut enemy_rng_gen,
        );
    }
}
