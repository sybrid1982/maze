use bevy::color::Color;

pub const MAZE_X: i32 = 5;
pub const MAZE_Y: i32 = 5;
pub const MAZE_SCALE: f32 = 5.;
pub const WALL_THICKNESS: f32 = 0.2;

pub const GLOBAL_LIGHT_INTENSITY: f32 = 25.0;
pub const GLOBAL_LIGHT_TINT: Color = Color::srgb(0.5, 0.1, 0.1);

pub const DIRECTIONAL_LIGHT_INTENSITY: f32 = 100.0;
pub const DIRECTIONAL_LIGHT_TINT: Color = Color::WHITE;

pub const PLAYER_SPEED: f32 = 8.;
pub const PLAYER_HEIGHT: f32 = 2.5;
pub const PLAYER_LENGTH: f32 = 1.3;
pub const PLAYER_WIDTH: f32 = 1.3;

pub const TOP_DOWN_CAMERA_HEIGHT: f32 = 30.0;

// range from 0-1
pub const PROBABILITY_PAINTING: f32 = 0.25;
pub const PAINTING_THICKNESS: f32 = 0.2;

pub const WALL_LIGHT_PROBABILITY: f32 = 0.25;

pub const DOOR_PROBABILITY: f32 = 0.3;