
use super::maze_cell_edge::MazeCellEdge;
use crate::{maze::maze_direction::MazeDirection, position::Position};

#[derive(Default, Clone)]
pub struct MazeCell {
    pub room_index: usize,
    position: Position,
    pub edges: Vec<Option<MazeCellEdge>>,
    initialized: bool
}

impl MazeCell {
    pub fn new(room_index: usize, position: &Position) -> Self {
        Self {
            room_index,
            edges: vec![None; 4],
            initialized: false,
            position: position.clone()
        }
    }

    pub fn set_edge(&mut self, direction: MazeDirection, edge: MazeCellEdge) {
        self.edges[direction as usize] = Some(edge);
    }

    pub fn get_edge(&self, direction: MazeDirection) -> Option<&MazeCellEdge> {
        self.edges[direction as usize].as_ref()
    }

    pub fn initialized(&mut self) {
        self.initialized = true;
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn get_position(&self) -> Position {
        self.position.clone()
    }
}
