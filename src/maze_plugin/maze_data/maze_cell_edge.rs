use crate::maze::maze_direction::MazeDirection;

#[derive(Debug, Clone, PartialEq)]
pub enum EdgeType {
    Wall,
    Door(bool), // true = open, false = closed
    None
}

#[derive(Debug, Clone)]
pub struct MazeCellEdge {
    pub direction: MazeDirection,
    pub edge_type: EdgeType,
    pub wall_furniture: Vec<String>,
}

impl MazeCellEdge {
    pub fn new(direction: MazeDirection, edge_type: EdgeType) -> Self {
        Self {
            direction,
            edge_type,
            wall_furniture: vec![]
        }
    }

    pub fn is_passable(&self) -> bool {
        self.edge_type == EdgeType::None || self.edge_type == EdgeType::Door(true)
    }
}
