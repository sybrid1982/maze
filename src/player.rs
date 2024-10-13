use bevy::prelude::*;

use super::position::Position;
use super::consts;
use super::velocity::Velocity;

const PLAYER_START_POSITION: Position = Position { x: 0., y: 0. };

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
struct Speed(f32);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, input_player_update);
    }
}

fn setup (    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let player = (
        PbrBundle {
            mesh: meshes.add(Cuboid::new(consts::PLAYER_LENGTH, consts::PLAYER_WIDTH, consts::PLAYER_HEIGHT)),
            material: materials.add(Color::srgb(0.7,0.1,0.2)),
            transform: Transform::from_xyz(PLAYER_START_POSITION.x, consts::MAZE_SCALE as f32 / 2., PLAYER_START_POSITION.y)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            ..default()
        },
        Player,
        Speed(consts::PLAYER_SPEED),
        Name::new("Player"),
        Velocity::new(0.0, 0.0),
    );

    let light = (
        PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 8.0),
            ..default()
        },
        Name::new("PlayerLight")
    );

    commands.spawn( player ).with_children(|parent: &mut ChildBuilder<'_>| {
        parent.spawn(light);
    });
}

fn input_player_update(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
    speed_query: Query<&Speed, With<Player>>,
) {
    let mut velocity = query.single_mut();
    let player_speed = **speed_query.single();
    let mut direction_x = 0.0;
    let mut direction_z = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction_x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction_x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction_z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction_z += 1.0;
    }

    let new_velocity_x = direction_x * player_speed;
    let new_velocity_z = direction_z * player_speed;


    velocity.set_velocity(new_velocity_x, new_velocity_z);
}