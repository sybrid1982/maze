use super::maze_direction::MazeDirection;
use crate::position::Position;

#[derive(Default, Copy, Clone, PartialEq)]
pub enum EdgeType {
    #[default]
    Passage,
    Wall
}

pub struct MazeCellEdge {
    position: Position,
    maze_direction: MazeDirection,
    edge_type: EdgeType
}

impl MazeCellEdge {
    pub fn new(position: Position, adjacent_position: Position) -> MazeCellEdge {
        let maze_direction = MazeDirection::get_direction_position_from_positions(&position, &adjacent_position);
        MazeCellEdge { position, maze_direction, edge_type: EdgeType::default() }
    }

    pub fn set_wall(&mut self) {
        self.edge_type = EdgeType::Wall;
    }

    pub fn get_edge_type(&self) -> EdgeType {
        self.edge_type
    }

    pub fn get_position(&self) -> Position {
        self.position.clone()
    }

    pub fn get_maze_direction(&self) -> MazeDirection {
        self.maze_direction
    }
}
