use bevy::prelude::*;

use crate::{physics::velocity::Velocity, position::Position};

#[derive(Component, Deref, DerefMut)]
pub struct Speed(pub f32);

#[derive(Bundle)]
pub struct CharacterBundle {
    pub velocity: Velocity,
    pub position: Position,
    pub speed: Speed
}