use bevy::prelude::*;
use rand::Rng;

use super::position::Position;
use super::random::Random;
use super::maze_direction::MazeDirection;
use super::maze_cell::MazeCell;

#[derive(Default)]
pub struct Maze {
    pub size_x: i32,
    pub size_y: i32,
    cells: Vec<Vec<MazeCell>>
}

impl Maze { 
    pub fn new(x: i32, y: i32) -> Self {
        Maze {
            size_x: x,
            size_y: y,
            cells: vec![]
        }
    }

    pub fn generate(&mut self, mut rand: ResMut<Random>) {
        // probably need to check if cells exists, and if it does, wipe it
        
        // will this do it?
        self.cells.clear();

        for i in 0..self.size_x {
            let mut row: Vec<MazeCell> = vec![];
            for j in 0..self.size_y {
                let new_cell = MazeCell::new(i, j);
                row.push(new_cell);
            }
            self.cells.push(row);
        }

        let mut active_positions: Vec<Position> = vec![];
        self.do_first_generation_step(&mut active_positions, &mut rand);

        while active_positions.len() > 0 {
            self.do_next_generation_step(&mut active_positions, &mut rand);
        }
    }

    fn do_first_generation_step(&mut self, active_positions: &mut Vec<Position>, rand: &mut ResMut<Random>) {
        let mut position = random_position(self.size_x, self.size_y, rand);
        active_positions.push(position);

        self.get_cell(&position).toggle_render();
    }

    fn do_next_generation_step(&mut self, active_positions: &mut Vec<Position>, rand: &mut ResMut<Random>) {
        let current_position = active_positions.pop();
        match current_position {
            Some(position) => {
                let new_position = &position + MazeDirection::get_random_direction(rand).to_position_modifier();
                if self.contains_position(&new_position) && self.get_cell(&new_position).is_render() == false {
                    active_positions.push(position);
                    active_positions.push(new_position);
                    self.get_cell(&new_position).toggle_render();
                }
            },
            None => {
                panic!("Tried to pop an empty vec of positions!")
            }
        }
    }

    pub fn get_cells(&self) -> &Vec<Vec<MazeCell>> {
        &self.cells
    }

    pub fn get_cell(&mut self, position: &Position) -> &mut MazeCell {
        &mut self.cells[position.x as usize][position.y as usize]
    }
    
    fn contains_position(&self, position: &Position) -> bool {
        position.x >= 0 && position.x < self.size_x && position.y >= 0 && position.y < self.size_y
    }
}

fn random_position(max_x: i32, max_y: i32, rand: &mut ResMut<Random>) -> Position {
    let mut random_x = rand.gen_range(0..max_x);
    let mut random_y = rand.gen_range(0..max_y);
    
    Position::new(random_x, random_y)
}