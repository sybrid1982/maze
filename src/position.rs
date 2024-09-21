use std::ops::Add;

pub struct Position {
    x: i32,
    y: i32,
}

impl<'a, 'b> Add<&'b Position> for &'a Position {
    type Output = Position;

    fn add(self, other: &'b Position) -> Position {
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