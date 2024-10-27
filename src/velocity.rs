use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity( Vec2::new(x, y) )
    }

    pub fn set_velocity(&mut self, new_velocity: Vec2) {
        self.x = new_velocity.x;
        self.y = new_velocity.y;
    }

    pub fn zero_velocity(&mut self) {
        self.set_velocity(Vec2{x: 0.0, y: 0.0})
    }
}

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_velocity.after(super::check_for_collisions));
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.z += velocity.y * time.delta_seconds();
    }
}
