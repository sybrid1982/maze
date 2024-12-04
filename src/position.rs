use std::ops::{Add, Mul, Sub};
use bevy::prelude::*;

#[derive(Default, Debug, Copy, Clone, PartialEq, Component, Reflect, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct MazePosition(pub Vec2);

impl<'a> Add<Position> for &'a Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<'a, 'b> Sub <&'b Position> for &'a Position {
    type Output = Position;

    fn sub(self, other: &Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Mul <usize> for Position {
    type Output = Position;

    fn mul(self, mult: usize) -> Position {
        Position {
            x: self.x * mult,
            y: self.y * mult 
        }
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position {x, y}
    }
    
    pub fn new_from_i32(x: i32, y: i32) -> Self {
        Position {x: x as usize, y: y as usize}
    }

    pub fn new_from_usize(x: usize, y: usize) -> Self {
        Position {x: x as usize, y: y as usize}
    }

    pub fn new_from_ivec2(ivec: IVec2) -> Self {
        Position { x: ivec.x as usize, y: ivec.y as usize }
    }
    
    pub fn get_as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
    // y is height in bevy, but we are using it as depth
    pub fn to_vec3_by_scale(&self, scale: usize) -> Vec3 {
        let (x, y) = self.get_as_tuple();
        Vec3::new((x * scale) as f32, (0 * scale) as f32, (y * scale) as f32)
    }

    pub fn get_as_vec2(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }

    pub fn get_as_ivec2(&self) -> IVec2 {
        IVec2::new(self.x as i32, self.y as i32)
    }

    pub fn get_from_transform(transform: &Transform, scale: usize) -> Self {
        Position { x: (transform.translation.x / scale as f32).round() as usize, y: (transform.translation.z / scale as f32).round() as usize }
    }

    pub fn get_distance_to_position(&self, position: Position) -> usize {
        (self.x as usize).abs_diff(position.x as usize) + (self.y as usize).abs_diff(position.y as usize)
    }
}
