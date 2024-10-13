use bevy::{
    prelude::*,
    math::bounding::{Bounded2d, Aabb2d}
};

use super::maze::maze_direction::MazeDirection;
use super::consts;

#[derive(Component)]
pub struct Collider;

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CollisionOrientation {
    HORIZONTAL,
    VERTICAL
}

impl Collider {
    /**
     * Takes two colliders and says what side of the first collider is colliding with the second, if any
     */
    pub fn box_collision(moving_collider: Aabb2d, static_collider: Aabb2d) -> Option<MazeDirection> {
        let vertical_collision_percentage = get_percentage_of_side_in_collision(moving_collider.min.y, static_collider.min.y, moving_collider.max.y, static_collider.max.y);
        let horizontal_collision_percentage = get_percentage_of_side_in_collision(moving_collider.min.x, static_collider.min.x, moving_collider.max.x, static_collider.max.x);

        let collision_orientation = if vertical_collision_percentage > horizontal_collision_percentage { CollisionOrientation::VERTICAL } else { CollisionOrientation::HORIZONTAL };

        if is_collision_on_east(moving_collider, static_collider, collision_orientation) {
            return Some(MazeDirection::EAST);
        } else if is_collision_on_west(moving_collider, static_collider, collision_orientation) {
            return Some(MazeDirection::WEST);
        } else if is_collision_on_north(moving_collider, static_collider, collision_orientation) {
            return Some(MazeDirection::NORTH);
        } else if is_collision_on_south(moving_collider, static_collider, collision_orientation) {
            return Some(MazeDirection::SOUTH);
        }
        None
    }

    pub fn transform_to_aabb2d(transform: &Transform) -> Aabb2d {
        Aabb2d::new(
        Vec2::new(transform.translation.x, transform.translation.z),
        Vec2::new(transform.scale.x / 2., transform.scale.z / 2.),
        )
    }

    pub fn get_wall_aabb2d(transform: &Transform, wall_facing: &MazeDirection) -> Aabb2d {
        let wall_half_size = match wall_facing {
            MazeDirection::EAST => Vec2::new(consts::WALL_THICKNESS / 2., consts::MAZE_SCALE / 2.),
            MazeDirection::WEST => Vec2::new(consts::WALL_THICKNESS / 2., consts::MAZE_SCALE / 2.),
            MazeDirection::NORTH => Vec2::new(consts::MAZE_SCALE / 2., consts::WALL_THICKNESS / 2. ),
            MazeDirection::SOUTH => Vec2::new(consts::MAZE_SCALE / 2., consts::WALL_THICKNESS / 2.),
        };

        Aabb2d::new(
            Vec2::new(transform.translation.x, transform.translation.z),
            wall_half_size
        )
    }
}

fn value_inside_range(value: f32, min: f32, max: f32) -> bool {
    min < value && value < max
}

fn get_percentage_of_side_in_collision(a_min: f32, b_min: f32, a_max: f32, b_max: f32) -> f32 {
    let a_size = a_max - a_min;
    let last_point_of_a_in_b = f32::min(a_max, b_max);
    let first_point_of_a_in_b = f32::max(a_min, b_min);
    let amount_of_a_in_b = f32::max(last_point_of_a_in_b - first_point_of_a_in_b, 0.0);
    amount_of_a_in_b / a_size
}

fn is_collision_on_east(moving_collider: Aabb2d, static_collider: Aabb2d, collision_orientation: CollisionOrientation) -> bool {
    value_inside_range(moving_collider.max.x, static_collider.min.x, static_collider.max.x) &&
    (value_inside_range(moving_collider.min.y, static_collider.min.y, static_collider.max.y) ||
    value_inside_range(moving_collider.max.y, static_collider.min.y, static_collider.max.y)) &&
    collision_orientation == CollisionOrientation::VERTICAL

}

fn is_collision_on_west(moving_collider: Aabb2d, static_collider: Aabb2d, collision_orientation: CollisionOrientation) -> bool {
    value_inside_range(moving_collider.min.x, static_collider.min.x, static_collider.max.x) &&
    (value_inside_range(moving_collider.min.y, static_collider.min.y, static_collider.max.y) ||
    value_inside_range(moving_collider.max.y, static_collider.min.y, static_collider.max.y)) &&
    collision_orientation == CollisionOrientation::VERTICAL
}

fn is_collision_on_north(moving_collider: Aabb2d, static_collider: Aabb2d, collision_orientation: CollisionOrientation) -> bool {
    value_inside_range(moving_collider.max.y, static_collider.min.y, static_collider.max.y) &&
    (value_inside_range(moving_collider.min.x, static_collider.min.x, static_collider.max.x) ||
    value_inside_range(moving_collider.max.x, static_collider.min.x, static_collider.max.x)) &&
    collision_orientation == CollisionOrientation::HORIZONTAL
}

fn is_collision_on_south(moving_collider: Aabb2d, static_collider: Aabb2d, collision_orientation: CollisionOrientation) -> bool {
    value_inside_range(moving_collider.min.y, static_collider.min.y, static_collider.max.y) &&
    (value_inside_range(moving_collider.min.x, static_collider.min.x, static_collider.max.x) ||
    value_inside_range(moving_collider.max.x, static_collider.min.x, static_collider.max.x)) &&
    collision_orientation == CollisionOrientation::HORIZONTAL
}