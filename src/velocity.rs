use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity( Vec2::new(x, y) )
    }

    pub fn set_velocity(&mut self, new_x: f32, new_y: f32) {
        self.x = new_x;
        self.y = new_y;
    }

    pub fn zero_velocity(&mut self) {
        self.set_velocity(0.0, 0.0)
    }
}

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

fn update_position (
    mut query: Query<&mut Velocity>,
) {
    let speed_objects = query.iter()
}
