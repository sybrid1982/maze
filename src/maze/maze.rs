use std::usize;

use bevy::prelude::*;
use rand::Rng;

use crate::consts;
use crate::position::Position;
use crate::random::Random;
use super::maze_assets::MazeAssets;
use super::maze_cell::MazeCell;
use super::maze_cell_edge::EdgeType;
use super::maze_direction::MazeDirection;
use super::maze_room::MazeRooms;

#[derive(Default, Resource)]
pub struct Maze {
    pub size_x: i32,
    pub size_y: i32,
    // cells: Vec<MazeCell>,
    maze_rooms: MazeRooms
}

impl Maze { 
    pub fn new(x: i32, y: i32) -> Self {
        Maze {
            size_x: x,
            size_y: y,
            // cells: vec![],
            maze_rooms: MazeRooms::new()
        }
    }

    pub fn generate(&mut self, rand: &mut ResMut<Random>, maze_assets: Res<MazeAssets>, materials: ResMut<'_, Assets<StandardMaterial>>) {
        // probably need to check if cells exists, and if it does, wipe it
        
        // will this do it?
        self.initialize_maze_rooms(maze_assets, materials);

        let mut active_positions: Vec<Position> = vec![];

        self.do_first_generation_step(&mut active_positions, rand);

        while !active_positions.is_empty() {
            self.do_next_generation_step(&mut active_positions, rand);
        }
    }

    fn initialize_maze_rooms(&mut self, maze_assets: Res<MazeAssets>, mut materials: ResMut<'_, Assets<StandardMaterial>>) {
        self.maze_rooms.initialize_maze_rooms(maze_assets, &mut materials);
    }

    fn do_first_generation_step(&mut self, active_positions: &mut Vec<Position>, rand: &mut ResMut<Random>) {
        let position = random_position(self.size_x, self.size_y, rand);
        active_positions.push(position);

        let room_index = self.maze_rooms.create_room_and_return_index(usize::MAX, rand);

        self.add_cell(&position, room_index)
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
                        Some(entered_cell) => {
                            if self.maze_rooms.get_settings_index_from_room_index(current_cell.get_room_index()) == self.maze_rooms.get_settings_index_from_room_index(entered_cell.get_room_index()) {
                                self.expand_room(active_positions, position, new_position, self.maze_rooms.get_settings_index_from_room_index(current_cell.get_room_index()), self.maze_rooms.get_settings_index_from_room_index(entered_cell.get_room_index()), rand);
                            } else {
                                self.add_wall_to_position(active_positions, position, new_position, rand);
                            }
                        },
                        None => {
                            self.generate_passage_to_new_cell(active_positions, position, new_position, rand);
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

    fn expand_room(&mut self, active_positions: &mut Vec<Position>, position: Position, new_position: Position, room_index: usize, new_room_index: usize, rand: &mut ResMut<'_, Random>) {
        if room_index == new_room_index {
            active_positions.push(position);
            self.add_passage(&position, &new_position, rand);    
        } else {
            println!("merging two rooms");
            self.maze_rooms.merge_rooms(room_index, new_room_index);
            self.add_passage(&position, &new_position, rand);
        }
    }
    
    fn generate_passage_to_new_cell(&mut self, active_positions: &mut Vec<Position>, position: Position, new_position: Position, rand: &mut ResMut<'_, Random>) {
        active_positions.push(position);
        active_positions.push(new_position);
        let current_room_index = self.get_cell_mut(&position).expect("Current cell not in maze somehow").get_room_index();
        let index_to_exclude = self.maze_rooms.get_settings_index_from_room_index(current_room_index);
        if rand.gen_range(0. .. 1.) < consts::DOOR_PROBABILITY {
            let new_room_index = self.maze_rooms.create_room_and_return_index(index_to_exclude, rand);
            self.add_cell(&new_position, new_room_index);
            self.add_door(&position, &new_position, rand);
        } else {
            self.add_cell(&new_position, current_room_index);
            self.add_passage(&position, &new_position, rand);
        }
    }
    
    fn add_wall_to_position(&mut self, active_positions: &mut Vec<Position>, position: Position, new_position: Position, rand: &mut ResMut<Random>) {
        active_positions.push(position);
        self.add_wall(&position, &new_position, rand)
    }
    
    pub fn add_cell(&mut self, position: &Position, room_index: usize) {
        let mut cell = MazeCell::new(position.x, position.y, room_index);
        cell.toggle_render();
        self.maze_rooms.add_cell_to_room(cell, room_index);
    }

    pub fn add_wall(&mut self, prev_position: &Position, curr_position: &Position, rand: &mut ResMut<Random>) {
        let maze_direction = MazeDirection::get_direction_position_from_positions(prev_position, curr_position);
        let cell_leaving = self.get_cell_mut(prev_position);
        if let Some(cell) = cell_leaving {
            cell.add_edge(&maze_direction, Some(EdgeType::Wall) , rand);
        }
        let cell_entering = self.get_cell_mut(curr_position);
        if let Some(cell) = cell_entering {
            cell.add_edge(&maze_direction.get_opposite_direction(), Some(EdgeType::Wall), rand);
        }
    }

    pub fn get_cell(&self, position: &Position) -> Option<&MazeCell> {
        self.maze_rooms.get_cell(&position)
    }

    fn get_cell_mut(&mut self, position: &Position) -> Option<&mut MazeCell> {
        self.maze_rooms.get_cell_mut(&position)
    }

    fn add_passage(&mut self, prev_position: &Position, curr_position: &Position, rand: &mut ResMut<Random>) {
        let maze_direction = MazeDirection::get_direction_position_from_positions(prev_position, curr_position);

        let cell_leaving = self.get_cell_mut(prev_position);
        match cell_leaving {
            Some(cell) => {
                cell.add_edge(&maze_direction, None, rand);
            },
            None => {
                println!("No cell at position {}", format!("{:#?}", prev_position));
            }
        }
        let cell_entering = self.get_cell_mut(curr_position);
        match cell_entering {
            Some(cell) => {
                cell.add_edge(&maze_direction.get_opposite_direction(), None, rand);
            },
            None => {
                println!("No cell at position {}", format!("{:#?}", prev_position));
            }
        }
    }

    fn add_door(&mut self, prev_position: &Position, curr_position: &Position, rand: &mut ResMut<Random>) {
        let maze_direction = MazeDirection::get_direction_position_from_positions(prev_position, curr_position);

        let cell_leaving = self.get_cell_mut(prev_position);
        match cell_leaving {
            Some(cell) => {
                cell.add_edge(&maze_direction, Some(EdgeType::Doorway), rand);
            },
            None => {
                println!("No cell at position {}", format!("{:#?}", prev_position));
            }
        }
        let cell_entering = self.get_cell_mut(curr_position);
        match cell_entering {
            Some(cell) => {
                cell.add_edge(&maze_direction.get_opposite_direction(), Some(EdgeType::InverseDoorway), rand);
            },
            None => {
                println!("No cell at position {}", format!("{:#?}", prev_position));
            }
        }
    }

    pub fn render_maze(  
        &mut self,
        commands: &mut Commands,
        assets: &mut ResMut<Assets<Mesh>>,
        floors: Entity,
    ) {
        for index in 0..self.maze_rooms.get_room_count() {
            self.maze_rooms.render_room(commands, assets, floors, index);
        }
    }
    
    fn contains_position(&self, position: &Position) -> bool {
        position.x >= 0. && position.x < self.size_x as f32 && position.y >= 0. && position.y < self.size_y as f32
    }

    pub fn get_room_number_for_position(&self, position: Position) -> usize {
        self.maze_rooms.get_room_number_for_position(position)
    }
}

fn random_position(max_x: i32, max_y: i32, rand: &mut ResMut<Random>) -> Position {
    let random_x = rand.gen_range(0..max_x);
    let random_y = rand.gen_range(0..max_y);
    
    Position::new_from_i32(random_x, random_y)
}