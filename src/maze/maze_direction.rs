use bevy::prelude::*;

use crate::{consts, position::Position};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MazeDirection {
    NORTH,
    EAST,
    SOUTH,
    WEST
}

impl MazeDirection {
    pub fn get_direction_from_index(index: u32) -> MazeDirection {
        
        unsafe { ::std::mem::transmute(index) }
    }

    pub fn to_position_modifier(&self) -> Position {
        match self {
            MazeDirection::NORTH => Position::new(0., -1.),
            MazeDirection::EAST => Position::new(1., 0.),
            MazeDirection::SOUTH => Position::new(0., 1.),
            MazeDirection::WEST => Position::new(-1., 0.)
        }
    }

    pub fn get_opposite_direction(&self) -> MazeDirection {
        match self {
            MazeDirection::NORTH => MazeDirection::SOUTH,
            MazeDirection::EAST => MazeDirection::WEST,
            MazeDirection::SOUTH => MazeDirection::NORTH,
            MazeDirection::WEST => MazeDirection::EAST
        }
    }

    pub fn get_direction_position_from_positions(position1: &Position, position2: &Position) -> MazeDirection {
        let sum = position2 - position1;
        match sum {
            Position { x: 0., y: -1.} => MazeDirection::NORTH,
            Position { x: 1., y: 0.} => MazeDirection::EAST,
            Position { x: 0., y: 1.} => MazeDirection::SOUTH,
            Position { x: -1., y: 0.} => MazeDirection::WEST,
            _ => panic!("positions not adjacent")
        }
    }

    pub fn get_direction_quat(&self) -> Quat {
        match &self {
            MazeDirection::EAST => Quat::from_euler(EulerRot::XYZ, std::f32::consts::FRAC_PI_2, -std::f32::consts::FRAC_PI_2, 0.0 ),
            MazeDirection::NORTH => Quat::from_euler(EulerRot::XYZ, std::f32::consts::FRAC_PI_2, 0.0, 0.0 ),
            MazeDirection::WEST => Quat::from_euler(EulerRot::XYZ, std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2, 0.0),
            MazeDirection::SOUTH => Quat::from_euler(EulerRot::XYZ, std::f32::consts::FRAC_PI_2, -std::f32::consts::PI, 0.0 ),
        }
    }

    pub fn get_wall_position_for_cell(&self) -> Vec3 {
        match &self {
            MazeDirection::EAST => Vec3::new(2.5 - consts::WALL_THICKNESS, 2.5, 0.0),
            MazeDirection::NORTH => Vec3::new(-2.5, 2.5 - consts::WALL_THICKNESS, 0.),
            MazeDirection::WEST => Vec3::new(-2.5 + consts::WALL_THICKNESS, -2.5, 0.0),
            MazeDirection::SOUTH => Vec3::new(2.5, -2.5 + consts::WALL_THICKNESS, 0.0),
        }
    }

    pub fn get_door_position_for_cell(&self) -> Vec3 {
        match &self {
            MazeDirection::EAST => Vec3::new(2.5 - consts::WALL_THICKNESS, 2.5, 0.0),
            MazeDirection::NORTH => Vec3::new(-2.5, 2.5 - consts::WALL_THICKNESS, 0.),
            MazeDirection::WEST => Vec3::new(-2.5 + consts::WALL_THICKNESS, -2.5, 0.0),
            MazeDirection::SOUTH => Vec3::new(2.5, -2.5 + consts::WALL_THICKNESS, 0.0),
        }
    }

}
