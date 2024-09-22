use bevy::prelude::*;

use super::position::Position;

#[derive(Component, Copy, Clone)]
pub struct MazeCell {
    // not sure this is ever actually used?
    position: Position,
    // this is definitely used
    render: bool,
}

impl MazeCell {
    pub fn new(x: i32, y: i32) -> Self {
        MazeCell { position: Position::new( x, y ), render: false }
    }

    // y is height in bevy, but we are using it as depth
    pub fn get_position_as_vec3_to_scale(&self, scale: f32) -> Vec3 {
        let (x, y) = self.position.get_as_tuple();
        Vec3::new(x as f32 * scale, 0. * scale, y as f32 * scale)
    }

    pub fn is_render(&self) -> bool {
        self.render
    }

    pub fn toggle_render(&mut self) {
        self.render = !self.render
    }
}
