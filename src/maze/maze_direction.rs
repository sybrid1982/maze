use bevy::prelude::*;

use crate::position::Position;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MazeDirection {
    NORTH,
    EAST,
    SOUTH,
    WEST
}

impl MazeDirection {
    pub fn get_direction_from_index(index: u32) -> MazeDirection {
        let direction_index = unsafe { ::std::mem::transmute(index) };
        direction_index
    }

    pub fn to_position_modifier(&self) -> Position {
        match self {
            MazeDirection::NORTH => Position::new(0., 1.),
            MazeDirection::EAST => Position::new(1., 0.),
            MazeDirection::SOUTH => Position::new(0., -1.),
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
            Position { x: 0., y: 1.} => MazeDirection::NORTH,
            Position { x: 1., y: 0.} => MazeDirection::EAST,
            Position { x: 0., y: -1.} => MazeDirection::SOUTH,
            Position { x: -1., y: 0.} => MazeDirection::WEST,
            _ => panic!("positions not adjacent")
        }
    }

    pub fn get_direction_quat(&self) -> Quat {
        match &self {
            MazeDirection::EAST => Quat::from_euler(EulerRot::XZY, 0., 0., std::f32::consts::FRAC_PI_2 ),
            MazeDirection::NORTH => Quat::from_euler(EulerRot::XZY, 0., 0., 0. ),
            MazeDirection::WEST => Quat::from_euler(EulerRot::XZY, 0., 0., std::f32::consts::FRAC_PI_2),
            MazeDirection::SOUTH => Quat::from_euler(EulerRot::XZY, 0., 0., 0. ),
        }
    }

    pub fn get_wall_fudge(&self) -> Vec3 {
        match &self {
            MazeDirection::EAST => Vec3::new(0., 0.0, 2.5),
            MazeDirection::NORTH => Vec3::new(-2.5, 0.0, 0.),
            MazeDirection::WEST => Vec3::new(0., 0.0, 2.5),
            MazeDirection::SOUTH => Vec3::new(-2.5, 0.0, 0.),
        }
    }

    pub fn get_door_fudge(&self) -> Vec3 {
        match &self {
            MazeDirection::EAST => Vec3::new(0., 0.0, 2.5),
            MazeDirection::NORTH => Vec3::new(-2.5, 0.0, 0.),
            MazeDirection::WEST => Vec3::new(0., 0.0, 2.5),
            MazeDirection::SOUTH => Vec3::new(-2.5, 0.0, 0.),
        }
    }

}
