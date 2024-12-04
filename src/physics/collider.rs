use bevy::{
    prelude::*,
    math::bounding::Aabb2d
};

use crate::position::Position;
use crate::player::player::LogicalPlayer;
use super::velocity::Velocity;
use crate::GameRunSet;

use crate::maze::maze_direction::MazeDirection;
use crate::consts;

pub struct ColliderPlugin;
impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (check_for_collisions).in_set(GameRunSet));
    }
}

#[derive(Component)]
pub struct Collider{
    half_size: Vec2
}

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CollisionOrientation {
    HORIZONTAL,
    VERTICAL
}

impl Collider {
    pub fn create_from_transform(transform: &Transform) -> Self {
        Self {
            half_size: Vec2::new(transform.scale.x / 2., transform.scale.z / 2.)
        }
    }
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

    pub fn get_wall_aabb2d(transform: &GlobalTransform, wall_facing: &MazeDirection) -> Aabb2d {
        let wall_size = get_wall_size(wall_facing);

        let mid_point = get_wall_midpoint(transform, wall_facing);

        Aabb2d::new(
            mid_point,
            wall_size
        )
    }
}

fn get_wall_size(wall_facing: &MazeDirection) -> Vec2 {
    match wall_facing {
        MazeDirection::EAST => Vec2::new(consts::WALL_THICKNESS, consts::MAZE_SCALE as f32 / 2.),
        MazeDirection::WEST => Vec2::new(consts::WALL_THICKNESS, consts::MAZE_SCALE as f32 / 2.),
        MazeDirection::NORTH => Vec2::new(consts::MAZE_SCALE as f32 / 2., consts::WALL_THICKNESS ),
        MazeDirection::SOUTH => Vec2::new(consts::MAZE_SCALE as f32 / 2., consts::WALL_THICKNESS ),
    }
}

fn get_wall_midpoint(transform: &GlobalTransform, wall_facing: &MazeDirection) -> Vec2 {
    match wall_facing {
        MazeDirection::NORTH => Vec2::new(transform.translation().x + 2.5, transform.translation().z),
        MazeDirection::EAST =>  Vec2::new(transform.translation().x , transform.translation().z + 2.5),
        MazeDirection::SOUTH => Vec2::new(transform.translation().x - 2.5, transform.translation().z),
        MazeDirection::WEST =>  Vec2::new(transform.translation().x, transform.translation().z - 2.5)
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

// right now this is checking every collider against the player.
// however, for walls and doors, the only time a player is going to be able to collide with them is
// when they are in the same cell as the collider.
// Ergo, we should be able to to instead get the player's cell, and then check collision with the objects
// for that cell
pub(crate) fn check_for_collisions(
    mut player_query: Query<(&mut Velocity, &Transform, &Position), With<LogicalPlayer>>,
    collider_query: Query<(&GlobalTransform, &Position), (With<Collider>, Without<LogicalPlayer>)>,
) {
    let (mut player_velocity, player_transform) = player_query.single_mut();

    let player_collider = Collider::transform_to_aabb2d(player_transform);

    let mut number_of_collisions = 0;

    let nearby_colliders = collider_query.iter().filter(|(_, position| { })

    for (collider_transform, wall_position) in collider_query.iter() {
        // need to get the dimensions of the wall and the dimensions of the player size
        // then use those to determine if one is inside the other
        let wall_collider = Collider::get_wall_aabb2d(collider_transform, wall_position);
        let collision = Collider::box_collision(player_collider, wall_collider);

        if let Some(collision) = collision {
            // collision_events.send(CollisionEvent);
            match collision {
                MazeDirection::EAST => player_velocity.x = f32::min(player_velocity.x, 0.),
                MazeDirection::WEST => player_velocity.x = f32::max(player_velocity.x, 0.),
                MazeDirection::NORTH => player_velocity.y = f32::min(player_velocity.y, 0.),
                MazeDirection::SOUTH => player_velocity.y = f32::max(player_velocity.y, 0.)
            }

            number_of_collisions += 1;
        }
    }
}