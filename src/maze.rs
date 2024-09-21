use bevy::prelude::*;
use super::position::Position;


#[derive(Component)]
pub struct MazeCell {
    position: Position
}

impl MazeCell {
    pub fn new(x: i32, y: i32) -> Self {
        MazeCell { position: Position::new( x, y ) }
    }

    fn get_position(&self) -> &Position {
        &self.position
    }

    // y is height in bevy, but we are using it as depth
    pub fn get_position_as_vec3_to_scale(&self, scale: f32) -> Vec3 {
        let (x, y) = self.position.get_as_tuple();
        Vec3::new(x as f32 * scale, 0. * scale, y as f32 * scale)
    }
}

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

    pub fn generate(&mut self) {
        // probably need to check if cells exists, and if it does, wipe it
        
        // will this do it?
        self.cells.clear();

        for i in 0..self.size_x {
            let mut row: Vec<MazeCell> = vec![];
            for j in 0..self.size_y {
                let newCell = MazeCell::new(i, j);
                row.push(newCell);
            }
            self.cells.push(row);
        }
    }

    pub fn get_cells(&self) -> &Vec<Vec<MazeCell>> {
        &self.cells
    }
}