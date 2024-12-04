
use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct MazeDoor {
    pub is_open: bool,
}

impl MazeDoor {
    pub fn new(is_open: bool) -> Self {
        Self { is_open }
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }
}
