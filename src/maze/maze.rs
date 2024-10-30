use std::usize;

use bevy::prelude::*;
use rand::Rng;

use crate::consts;
use crate::position::Position;
use crate::random::Random;
use super::maze_cell::MazeCell;
use super::maze_cell_edge::EdgeType;
use super::maze_direction::MazeDirection;
use super::maze_room::MazeRooms;
use super::paintings::Painting;

#[derive(Default, Resource)]
pub struct Maze {
    pub size_x: i32,
    pub size_y: i32,
    cells: Vec<MazeCell>,
}

impl Maze { 
    pub fn new(x: i32, y: i32) -> Self {
        Maze {
            size_x: x,
            size_y: y,
            cells: vec![],
        }
    }

    pub fn generate(&mut self, rand: &mut ResMut<Random>, maze_rooms: &mut ResMut<MazeRooms>) {
        // probably need to check if cells exists, and if it does, wipe it
        
        // will this do it?
        self.cells.clear();

        let mut active_positions: Vec<Position> = vec![];
        self.do_first_generation_step(&mut active_positions, rand, maze_rooms);

        while active_positions.len() > 0 {
            self.do_next_generation_step(&mut active_positions, rand, maze_rooms);
        }
    }

    fn do_first_generation_step(&mut self, active_positions: &mut Vec<Position>, rand: &mut ResMut<Random>, maze_rooms: &mut ResMut<MazeRooms>) {
        let position = random_position(self.size_x, self.size_y, rand);
        active_positions.push(position.clone());

        let room_index = maze_rooms.create_room_and_return_index(usize::MAX, rand);

        self.add_cell(&position, room_index)
    }

    fn do_next_generation_step(&mut self, active_positions: &mut Vec<Position>, rand: &mut ResMut<Random>, maze_rooms: &mut ResMut<MazeRooms>) {
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
                        Some(entered_cell) => {
                            if current_cell.get_room_index() == entered_cell.get_room_index() {
                                self.expand_room(active_positions, position, new_position);
                            } else {
                                self.add_wall_to_position(active_positions, position, new_position, rand);
                            }
                        },
                        None => {
                            self.generate_passage_to_new_cell(active_positions, position, new_position, rand, maze_rooms);
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

    fn expand_room(&mut self, active_positions: &mut Vec<Position>, position: Position, new_position: Position) {
        active_positions.push(position);
        self.add_passage(&position, &new_position);
    }
    
    fn generate_passage_to_new_cell(&mut self, active_positions: &mut Vec<Position>, position: Position, new_position: Position, rand: &mut ResMut<'_, Random>, maze_rooms: &mut ResMut<MazeRooms>) {
        active_positions.push(position);
        active_positions.push(new_position);
        let current_room_index = self.get_cell_mut(&position).expect("Current cell not in maze somehow").get_room_index();
        let index_to_exclude = maze_rooms.get_settings_index_from_room_index(current_room_index);
        if rand.gen_range(0. .. 1.) < consts::DOOR_PROBABILITY {
            println!("Generated a door");
            let new_room_index = maze_rooms.create_room_and_return_index(index_to_exclude, rand);
            self.add_cell(&new_position, new_room_index);
            self.add_door(&position, &new_position);
        } else {
            self.add_cell(&new_position, current_room_index);
            self.add_passage(&position, &new_position);
        }
    }
    
    fn add_wall_to_position(&mut self, active_positions: &mut Vec<Position>, position: Position, new_position: Position, rand: &mut ResMut<Random>) {
        let maybe_painting = Painting::generate_random_painting(rand);
        active_positions.push(position);
        self.add_wall(&position, &new_position, maybe_painting)
    }
    
    pub fn add_cell(&mut self, position: &Position, room_index: usize) {
        let mut cell = MazeCell::new(position.x, position.y, room_index);
        cell.toggle_render();
        self.cells.push(cell);
    }

    pub fn add_wall(&mut self, prev_position: &Position, curr_position: &Position, maybe_painting: Option<Painting>) {
        let maze_direction = MazeDirection::get_direction_position_from_positions(prev_position, curr_position);
        let cell_leaving = self.get_cell_mut(&prev_position);
        match cell_leaving {
            Some(cell) => {
                cell.add_edge(&maze_direction, Some(EdgeType::Wall));
            },
            None => {
            }
        }
        let cell_entering = self.get_cell_mut(&curr_position);
        match cell_entering {
            Some(cell) => {
                cell.add_edge(&maze_direction.get_opposite_direction(), Some(EdgeType::Wall));
            },
            None => {
            }
        }
    }

    fn add_passage(&mut self, prev_position: &Position, curr_position: &Position) {
        let maze_direction = MazeDirection::get_direction_position_from_positions(prev_position, curr_position);

        let cell_leaving = self.get_cell_mut(&prev_position);
        match cell_leaving {
            Some(cell) => {
                cell.add_edge(&maze_direction, None);
            },
            None => {
                println!("No cell at position {}", format!("{:#?}", prev_position));
            }
        }
        let cell_entering = self.get_cell_mut(&curr_position);
        match cell_entering {
            Some(cell) => {
                cell.add_edge(&maze_direction.get_opposite_direction(), None);
            },
            None => {
                println!("No cell at position {}", format!("{:#?}", prev_position));
            }
        }
    }

    fn add_door(&mut self, prev_position: &Position, curr_position: &Position) {
        let maze_direction = MazeDirection::get_direction_position_from_positions(prev_position, curr_position);

        let cell_leaving = self.get_cell_mut(&prev_position);
        match cell_leaving {
            Some(cell) => {
                cell.add_edge(&maze_direction, Some(EdgeType::Doorway));
            },
            None => {
                println!("No cell at position {}", format!("{:#?}", prev_position));
            }
        }
        let cell_entering = self.get_cell_mut(&curr_position);
        match cell_entering {
            Some(cell) => {
                cell.add_edge(&maze_direction.get_opposite_direction(), Some(EdgeType::Doorway));
            },
            None => {
                println!("No cell at position {}", format!("{:#?}", prev_position));
            }
        }
    }

    pub fn get_cells(&mut self) -> &mut Vec<MazeCell> {
        &mut self.cells
    }

    pub fn get_cell_mut(&mut self, position: &Position) -> Option<&mut MazeCell> {
        self.cells.iter_mut().find(|cell| cell.get_position() == *position)
    }

    pub fn get_cell(&self, position: &Position) -> Option<&MazeCell> {
        self.cells.iter().find(|cell| cell.get_position() == *position)
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