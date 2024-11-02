use bevy::prelude::*;

use crate::game_states::GameState;

use super::velocity::apply_velocity;
use super::collider::check_for_collisions;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (check_for_collisions, apply_velocity).chain().run_if(in_state(GameState::InGame)));
    }
}