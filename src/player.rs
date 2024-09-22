use bevy::prelude::*;

use super::position::Position;

#[derive(Default)]
pub struct Player {
    entity: Option<Entity>,
    position: Position,
}