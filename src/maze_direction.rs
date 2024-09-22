use bevy::prelude::*;
use rand::Rng;

use super::random::Random;
use super::position::Position;

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum MazeDirection {
    NORTH,
    EAST,
    SOUTH,
    WEST
}

impl MazeDirection {
    pub fn get_random_direction(rand: &mut ResMut<Random>) -> MazeDirection {
        let direction_index = unsafe { ::std::mem::transmute(rand.gen_range(0..4)) };
        direction_index
    }

    pub fn to_position_modifier(&self) -> Position {
        match self {
            MazeDirection::NORTH => Position::new(0, 1),
            MazeDirection::EAST => Position::new(1, 0),
            MazeDirection::SOUTH => Position::new(0, -1),
            MazeDirection::WEST => Position::new(-1, 0)
        }
    }
}
