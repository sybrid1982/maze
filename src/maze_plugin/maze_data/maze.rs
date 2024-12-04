
use bevy::prelude::*;
use rand::Rng;
use super::maze_cell::MazeCell;
use super::maze_cell_edge::{EdgeType, MazeCellEdge};
use crate::consts;
use crate::maze::maze_direction::MazeDirection;
use crate::position::Position;
use crate::random::Random;

#[derive(Resource)]
pub struct Maze {
    size_x: usize,
    size_y: usize,
    pub cells: Vec<MazeCell>,
}

impl Maze {
    pub fn new(size_x: usize, size_y: usize) -> Self {
        Self {
            size_x,
            size_y,
            cells: vec![MazeCell::default(); size_x * size_y],
        }
    }

    /// Generates the maze using the randomized depth-first search algorithm.
    pub fn generate(&mut self, rng: &mut Random) {
        let mut active_positions = Vec::new();
        let starting_position = random_position(self.size_x, self.size_y, rng);
        active_positions.push(starting_position);

        while let Some(active_position) = active_positions.pop() {
            if let Some(neighbors) = self.get_unvisited_neighbors(active_position) {
                active_positions.push(active_position);
                let (neighbor_position, direction) = rng.choose(&neighbors);
                self.create_edges_for_direction(active_position, direction, rng);
                active_positions.push(neighbor_position);
            }
        }
    }

    /// creates an edge for a cell in a direction
    /// if a neighbor exists, creates an edge in the opposite direction of the same type
    fn create_edges_for_direction(
        &mut self,
        position: Position,
        direction: MazeDirection,
        rng: &mut Random
    ) {
        let possible_position = position.get_as_ivec2() + direction.get_as_ivec2_modifier();

        if !self.is_in_maze(possible_position) || self.get_cell(&Position::new_from_i32(possible_position.x, possible_position.y)).is_initialized() {
            // create a wall
            self.create_wall(position, direction)
        } else {
            // create a passage or door
            if rng.gen_range(0. ..1.) < consts::DOOR_PROBABILITY {
                self.create_door(position, direction)
            } else {
                self.create_passage(position, direction)
            }
        }
        // let edge = MazeCellEdge::new(direction, EdgeType::None);
        // self.set_edge(x1, y1, direction, edge);

        // let opposite_direction = direction.get_opposite_direction();
        // let opposite_edge = MazeCellEdge::new(opposite_direction, EdgeType::None);
        // self.set_edge(x2, y2, opposite_direction, opposite_edge);
    }

    /// Finds unvisited neighbors of a cell.
    fn get_unvisited_neighbors(&self, position: Position) -> Option<Vec<(Position, MazeDirection)>> {
        let mut neighbors = Vec::new();

        if position.x > 0 && self.is_unvisited(&position, MazeDirection::WEST) {
            neighbors.push((MazeDirection::WEST.get_position(&position), MazeDirection::WEST));
        }
        if position.x < self.size_x - 1 && self.is_unvisited(&position, MazeDirection::EAST) {
            neighbors.push((MazeDirection::EAST.get_position(&position), MazeDirection::EAST));
        }
        if position.y > 0 && self.is_unvisited(&position, MazeDirection::NORTH) {
            neighbors.push((MazeDirection::NORTH.get_position(&position), MazeDirection::NORTH));
        }
        if position.y < self.size_y - 1 && self.is_unvisited(&position, MazeDirection::SOUTH) {
            neighbors.push((MazeDirection::SOUTH.get_position(&position), MazeDirection::SOUTH));
        }

        if neighbors.is_empty() {
            None
        } else {
            Some(neighbors)
        }
    }

    fn is_in_maze(&self, possible_position: IVec2) -> bool {
        possible_position.x >= 0 && possible_position.x < self.size_x as i32 && possible_position.y >= 0 && possible_position.y < self.size_y as i32
    }

    /// Checks if a cell already has an edge (aka, these two cells already have a link) 
    fn is_unvisited(&self, position: &Position, maze_direction: MazeDirection) -> bool {
        self.get_cell(position).get_edge(maze_direction).is_none()
    }

    /// Sets an edge on a cell.
    fn set_edge(
        &mut self,
        x: usize,
        y: usize,
        direction: MazeDirection,
        edge: MazeCellEdge,
    ) {
        let index = y * self.size_x + x;
        self.cells[index].set_edge(direction, edge);
    }

    fn get_cell_mut(&mut self, position: &Position) -> &mut MazeCell {
        let index = position.y as usize * self.size_x + position.x as usize;
        &mut self.cells[index]
    }

    fn get_cell(&self, position: &Position) -> &MazeCell {
        let index = position.y as usize * self.size_x + position.x as usize;
        &self.cells[index]
    }

    fn get_cell_in_direction(&mut self, position: &Position, direction: MazeDirection) -> &mut MazeCell {
        self.get_cell_mut(&direction.get_position(position))
    }

    fn create_wall(&mut self, position1: Position, direction: MazeDirection) {
        self.get_cell_mut(&position1).set_edge(direction, MazeCellEdge::new(direction, EdgeType::Wall));
        let possible_position = position1.get_as_ivec2() + direction.get_as_ivec2_modifier();
        if self.is_in_maze(possible_position) {
            let opposite_direction = direction.get_opposite_direction();
            self.get_cell_mut(&Position::new_from_ivec2(possible_position)).set_edge(opposite_direction, MazeCellEdge::new(opposite_direction, EdgeType::Wall));
        }
    }

    fn create_door(&mut self, position1: Position, direction: MazeDirection) {
        self.get_cell_mut(&position1).set_edge(direction, MazeCellEdge::new(direction, EdgeType::Door(false)));
        let possible_position = position1.get_as_ivec2() + direction.get_as_ivec2_modifier();
        if self.is_in_maze(possible_position) {
            let opposite_direction = direction.get_opposite_direction();
            self.get_cell_mut(&Position::new_from_ivec2(possible_position)).set_edge(opposite_direction, MazeCellEdge::new(opposite_direction, EdgeType::Door(false)));
        }
    }

    fn create_passage(&mut self, position1: Position, direction: MazeDirection) {
        self.get_cell_mut(&position1).set_edge(direction, MazeCellEdge::new(direction, EdgeType::None));
        let possible_position = position1.get_as_ivec2() + direction.get_as_ivec2_modifier();
        if self.is_in_maze(possible_position) {
            let opposite_direction = direction.get_opposite_direction();
            self.get_cell_mut(&Position::new_from_ivec2(possible_position)).set_edge(opposite_direction, MazeCellEdge::new(opposite_direction, EdgeType::None));
        }
    }
}

fn random_position(max_x: usize, max_y: usize, rand: &mut Random) -> Position {
    let random_x = rand.gen_range(0..max_x);
    let random_y = rand.gen_range(0..max_y);
    
    Position::new_from_usize(random_x, random_y)
}