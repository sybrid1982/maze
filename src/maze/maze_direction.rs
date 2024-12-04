use bevy::prelude::*;

use crate::{consts, position::Position};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Component)]
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

    pub fn get_position(&self, position: &Position) -> Position {
        match self {
            MazeDirection::NORTH => Position::new(position.x, position.y - 1),
            MazeDirection::EAST => Position::new(position.x + 1, position.y),
            MazeDirection::SOUTH => Position::new(position.x, position.y + 1),
            MazeDirection::WEST => Position::new(position.x - 1, position.y)
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

    pub fn get_as_ivec2_modifier(&self) -> IVec2 {
        match self {
            MazeDirection::NORTH => IVec2::new(0, -1),
            MazeDirection::EAST => IVec2::new(1, 0),
            MazeDirection::SOUTH => IVec2::new(0, 1),
            MazeDirection::WEST => IVec2::new(-1, 0)
        }
    }

    pub fn get_direction_position_from_positions(position1: &Position, position2: &Position) -> MazeDirection {
        let x = position2.x as isize - position1.x as isize;
        let y = position2.x as isize - position1.x as isize;
        match (x, y) {
            (0, -1) => MazeDirection::NORTH,
            (1, 0) => MazeDirection::EAST,
            (0, 1) => MazeDirection::SOUTH,
            (-1, 0) => MazeDirection::WEST,
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
