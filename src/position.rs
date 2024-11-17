use std::ops::{Add, Mul, Sub};
use bevy::prelude::*;

#[derive(Default, Debug, Copy, Clone, PartialEq, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
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

impl Mul <f32> for Position {
    type Output = Position;

    fn mul(self, mult: f32) -> Position {
        Position {
            x: self.x * mult,
            y: self.y * mult 
        }
    }
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position {x, y}
    }
    
    pub fn new_from_i32(x: i32, y: i32) -> Self {
        Position {x: x as f32, y: y as f32}
    }
    
    pub fn get_as_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    // y is height in bevy, but we are using it as depth
    pub fn to_vec3_by_scale(&self, scale: f32) -> Vec3 {
        let (x, y) = self.get_as_tuple();
        Vec3::new(x * scale, 0. * scale, y * scale)
    }

    pub fn get_as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn get_from_transform(transform: &Transform, scale: f32) -> Self {
        Position { x: (transform.translation.x / scale).round(), y: (transform.translation.z / scale).round() }
    }
}