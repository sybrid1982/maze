use bevy::prelude::*;

use crate::position::Position;

#[derive(Event)]
pub struct PlayerCellChangeEvent(pub Position);