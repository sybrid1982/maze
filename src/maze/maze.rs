use bevy::prelude::*;
use rand::Rng;

use crate::consts;
use crate::position::Position;
use crate::random::Random;
use super::maze_cell::MazeCell;
use super::maze_cell_edge::MazeCellEdge;
use super::maze_door::MazeDoor;
use super::paintings::Painting;

#[derive(Default, Resource)]
pub struct Maze {
    pub size_x: i32,
    pub size_y: i32,
    cells: Vec<MazeCell>,
    edges: Vec<MazeCellEdge>
}

impl Maze { 
    pub fn new(x: i32, y: i32) -> Self {
        Maze {
            size_x: x,
            size_y: y,
            cells: vec![],
            edges: vec![]
        }
    }

    pub fn generate(&mut self, rand: &mut ResMut<Random>) {
        // probably need to check if cells exists, and if it does, wipe it
        
        // will this do it?
        self.cells.clear();
        self.edges.clear();

        let mut active_positions: Vec<Position> = vec![];
        self.do_first_generation_step(&mut active_positions, rand);

        while active_positions.len() > 0 {
            self.do_next_generation_step(&mut active_positions, rand);
        }
    }

    fn do_first_generation_step(&mut self, active_positions: &mut Vec<Position>, rand: &mut ResMut<Random>) {
        let position = random_position(self.size_x, self.size_y, rand);
        active_positions.push(position.clone());

        self.add_cell(&position)
    }

    fn do_next_generation_step(&mut self, active_positions: &mut Vec<Position>, rand: &mut ResMut<Random>) {
        let current_position = active_positions.pop();
        match current_position {
            Some(position) => {
                let current_cell = self.get_cell(&position).unwrap();
                if current_cell.is_edge_complete() {
                    return;
                }
        
                let new_position = &position + current_cell.get_random_unused_direction_for_cell(rand).to_position_modifier();
                if self.contains_position(&new_position) {
                    match self.get_cell(&new_position) {
                        Some(..) => {
                            self.add_wall_to_position(active_positions, position, new_position, rand);
                        },
                        None => {
                            active_positions.push(position);
                            active_positions.push(new_position);
                            self.add_cell(&new_position);
                            if rand.gen_range(0. .. 1.) < consts::DOOR_PROBABILITY {
                                println!("Generated a door");
                                self.add_door(&position, &new_position)
                            } else {
                                self.add_passage(&position, &new_position)
                            }
                        }
                    }
                } else {
                    self.add_wall_to_position(active_positions, position, new_position, rand);
                }
            },
            None => {
                panic!("Tried to pop an empty vec of positions!")
            }
        }
    }

    fn add_wall_to_position(&mut self, active_positions: &mut Vec<Position>, position: Position, new_position: Position, rand: &mut ResMut<Random>) {
        let maybe_painting = Painting::generate_random_painting(rand);
        active_positions.push(position);
        self.add_wall(&position, &new_position, maybe_painting)
    }
    
    pub fn add_cell(&mut self, position: &Position) {
        let mut cell = MazeCell::new(position.x, position.y);
        cell.toggle_render();
        self.cells.push(cell);
    }

    pub fn add_wall(&mut self, prev_position: &Position, curr_position: &Position, maybe_painting: Option<Painting>) {
        let mut wall = MazeCellEdge::new(prev_position.clone(), curr_position.clone());
        wall.set_wall(maybe_painting);
        let cell_leaving = self.get_cell(&prev_position);
        match cell_leaving {
            Some(cell) => {
                cell.add_edge(&wall.get_maze_direction());
            },
            None => {
            }
        }
        let cell_entering = self.get_cell(&curr_position);
        match cell_entering {
            Some(cell) => {
                cell.add_edge(&wall.get_maze_direction().get_opposite_direction());
            },
            None => {
            }
        }
        self.edges.push(wall);
    }

    fn add_passage(&mut self, prev_position: &Position, curr_position: &Position) {
        let passage = MazeCellEdge::new(prev_position.clone(), curr_position.clone());

        let cell_leaving = self.get_cell(&prev_position);
        match cell_leaving {
            Some(cell) => {
                cell.add_edge(&passage.get_maze_direction());
            },
            None => {
                println!("No cell at position {}", format!("{:#?}", prev_position));
            }
        }
        let cell_entering = self.get_cell(&curr_position);
        match cell_entering {
            Some(cell) => {
                cell.add_edge(&passage.get_maze_direction().get_opposite_direction());
            },
            None => {
                println!("No cell at position {}", format!("{:#?}", prev_position));
            }
        }
    }

    fn add_door(&mut self, prev_position: &Position, curr_position: &Position) {
        let mut doorway = MazeCellEdge::new(prev_position.clone(), curr_position.clone());
        doorway.set_door();

        let cell_leaving = self.get_cell(&prev_position);
        match cell_leaving {
            Some(cell) => {
                cell.add_edge(&doorway.get_maze_direction());
            },
            None => {
                println!("No cell at position {}", format!("{:#?}", prev_position));
            }
        }
        let cell_entering = self.get_cell(&curr_position);
        match cell_entering {
            Some(cell) => {
                cell.add_edge(&doorway.get_maze_direction().get_opposite_direction());
            },
            None => {
                println!("No cell at position {}", format!("{:#?}", prev_position));
            }
        }

        self.edges.push(doorway);

    }

    pub fn get_cells(&self) -> &Vec<MazeCell> {
        &self.cells
    }

    pub fn get_edges(&self) -> &Vec<MazeCellEdge> {
        &self.edges
    }

    pub fn get_cell(&mut self, position: &Position) -> Option<&mut MazeCell> {
        self.cells.iter_mut().find(|cell| cell.get_position() == *position)
    }
    
    fn contains_position(&self, position: &Position) -> bool {
        position.x >= 0. && position.x < self.size_x as f32 && position.y >= 0. && position.y < self.size_y as f32
    }
}

fn random_position(max_x: i32, max_y: i32, rand: &mut ResMut<Random>) -> Position {
    let random_x = rand.gen_range(0..max_x);
    let random_y = rand.gen_range(0..max_y);
    
    Position::new_from_i32(random_x, random_y)
}