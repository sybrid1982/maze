use std::ops::{Add, Mul, Sub};

use bevy::prelude::*;

use super::grid::Grid;

#[derive(Component, Eq, PartialEq, Hash, Clone, Debug, Deref, DerefMut)]
pub struct GridLocation(pub IVec2);

impl GridLocation {
    pub fn new(x: u32, y: u32) -> Self {
        GridLocation(IVec2::new(x as i32, y as i32))
    }

    pub fn from_world(position: Vec2) -> Option<Self> {
        let position = position + Vec2::splat(0.5);
        let location = GridLocation(IVec2::new(position.x as i32, position.y as i32));
        if Grid::<()>::valid_index(&location) {
            Some(location)
        } else {
            None
        }
    }
}

impl<'a> Add<GridLocation> for &'a GridLocation {
    type Output = GridLocation;

    fn add(self, other: GridLocation) -> GridLocation {
        GridLocation(
            IVec2::new(
                self.x + other.x,
                self.y + other.y
            )
        )
    }
}

impl<'a, 'b> Sub <&'b GridLocation> for &'a GridLocation {
    type Output = GridLocation;

    fn sub(self, other: &GridLocation) -> GridLocation {
        GridLocation (
            IVec2::new(
                self.x - other.x,
                self.y - other.y
            )
        )
    }
}

impl Mul <i32> for GridLocation {
    type Output = GridLocation;

    fn mul(self, mult: i32) -> GridLocation {
        GridLocation (
            IVec2::new(
                self.x * mult,
                self.y * mult
            )
        )
    }
}
