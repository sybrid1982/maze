use bevy::prelude::*;
use rand::Rng;

use crate::{position::Position, random::Random};

use super::maze_direction::MazeDirection;

#[derive(Component, Clone)]
pub struct MazeCell {
    position: Position,
    render: bool,
    defined_edges: Vec<MazeDirection>
}

impl MazeCell {
    pub fn new(x: f32, y: f32) -> Self {
        MazeCell { position: Position::new( x, y ), render: false, defined_edges: vec![] }
    }

    pub fn is_render(&self) -> bool {
        self.render
    }

    pub fn toggle_render(&mut self) {
        self.render = !self.render
    }

    pub fn get_position(&self) -> Position {
        self.position.clone()
    }

    pub fn add_edge(&mut self, maze_direction: &MazeDirection) {
        if self.has_edge(&maze_direction) {
            panic!("Pushed same edge twice, stopping");
        }

        self.defined_edges.push(maze_direction.clone());
    }

    pub fn has_edge(&self, maze_direction: &MazeDirection) -> bool {
        self.defined_edges.contains(maze_direction)
    }

    pub fn is_edge_complete(&self) -> bool {
        self.defined_edges.len() == 4
    }

    pub fn get_random_unused_direction_for_cell(&self, rand: &mut ResMut<Random>) -> MazeDirection {
        let mut skips: usize = rand.gen_range(0..4-self.defined_edges.len());
        for i in 0..4 {
            let new_direction = &MazeDirection::get_direction_from_index(i);
            if !self.has_edge(new_direction) {
                if skips == 0 {
                    return new_direction.clone();
                }
                else 
                {
                    skips -= 1;
                }
            }
        }
        panic!("Ran out of possible edges before ran out of skips");
    }
}
