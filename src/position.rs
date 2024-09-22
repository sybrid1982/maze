use std::ops::Add;

#[derive(Default, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl<'a, 'b> Add<Position> for &'a Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position {x, y}
    }
    pub fn get_as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}