use bevy::{
    prelude::*,
    math::bounding::{Bounded2d, Aabb2d}
};

use super::maze::maze_direction::MazeDirection;

#[derive(Component)]
pub struct Collider;

#[derive(Event, Default)]
pub struct CollisionEvent;

impl Collider {
    fn box_collision(&self, other_collider: Aabb2d) -> Option<MazeDirection> {
        None
    }
}