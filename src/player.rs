// A lot of this code has been cribbed from https://github.com/qhdwight/bevy_fps_controller, modified because I'm not using rapier physics for the maze
// and I don't need jumping/no clip mode (maybe), etc
use std::f32::consts::*;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use super::position::Position;
use super::consts;
use super::velocity::Velocity;

const ANGLE_EPSILON: f32 = 0.001953125;
const PLAYER_START_POSITION: Position = Position { x: 0., y: 0. };

pub struct PlayerPlugin;

#[derive(Debug, Component)]
struct WorldModelCamera;

#[derive(Component, Deref, DerefMut)]
struct Speed(f32);

#[derive(Component)]
pub struct LogicalPlayer;

#[derive(Component)]
pub struct RenderPlayer {
    pub logical_entity: Entity,
}

#[derive(Component)]
pub struct CameraConfig {
    pub height_offset: f32,
}

#[derive(Component)]
pub struct Controller {
    pub pitch: f32,
    pub yaw: f32,
    pub sensitivity: f32,
    pub speed: f32,
    pub mouse_look: bool,
    pub draw_gizmos: bool
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            yaw: 0.0,
            sensitivity: 0.001,
            speed: consts::PLAYER_SPEED,
            mouse_look: true,
            draw_gizmos: false
        }
    }
}

#[derive(Component)]
pub struct ControllerInput {
    pub pitch: f32,
    pub yaw: f32,
    pub movement: Vec3,
    pub mouse_look: bool,
    pub draw_gizmos: bool
}

impl Default for ControllerInput {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            yaw: 0.0,
            movement: Vec3::ZERO,
            mouse_look: true,
            draw_gizmos: false
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        use bevy::input::{keyboard, mouse};

        app.add_systems(Startup, setup);
        app.add_systems(PreUpdate, (controller_input, controller_look, controller_move, controller_render).chain().after(mouse::mouse_button_input_system).after(keyboard::keyboard_input_system));
    }
}

fn get_pressed(key_input: &Res<ButtonInput<KeyCode>>, key: KeyCode) -> f32 {
    if key_input.pressed(key) {
        1.0
    } else {
        0.0
    }
}

fn get_axis(key_input: &Res<ButtonInput<KeyCode>>, key_pos: KeyCode, key_neg: KeyCode) -> f32 {
    get_pressed(key_input, key_pos) - get_pressed(key_input, key_neg)
}

fn setup (    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let player = (
        PbrBundle {
            mesh: meshes.add(Cuboid::new(consts::PLAYER_LENGTH, consts::PLAYER_HEIGHT, consts::PLAYER_WIDTH)),
            material: materials.add(Color::srgb(0.7,0.1,0.2)),
            transform: Transform::from_xyz(PLAYER_START_POSITION.x, consts::MAZE_SCALE as f32 / 2., PLAYER_START_POSITION.y),
            ..default()
        },
        LogicalPlayer,
        Speed(consts::PLAYER_SPEED),
        Name::new("Player"),
        Velocity::new(0.0, 0.0),
        Controller::default(),
        ControllerInput::default(),
    );

    let light = (
        SpotLightBundle {
            spot_light: SpotLight {
                intensity: 40_000.0, // lumens
                color: Color::WHITE,
                shadows_enabled: true,
                inner_angle: PI / 4.0 * 0.85,
                outer_angle: PI / 4.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, consts::PLAYER_HEIGHT / 4., 0.0),
            ..default()
        },
        Name::new("PlayerLight")
    );

    let logical_player =     commands.spawn( player ).insert(CameraConfig {
        height_offset: -0.5,
    }).id();

    commands.spawn((
        RenderPlayer { logical_entity: logical_player},
        WorldModelCamera,
        Camera3dBundle {
            projection: PerspectiveProjection {
                fov: 90.0_f32.to_radians(),
                ..default()
            }
            .into(),
            camera: Camera {
                order: 0,
                ..default()
            },
            ..default()
        },
    )).with_children(|parent: &mut ChildBuilder<'_>| {
        parent.spawn(light);
    });
}

pub fn controller_input(
    key_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<(&Controller, &mut ControllerInput)>,
) {
    for (controller, mut input) in query.iter_mut() {
        if controller.mouse_look {
            let mut mouse_delta = Vec2::ZERO;
            for motion in mouse_motion.read() {
                mouse_delta += motion.delta;
            }
            mouse_delta *= controller.sensitivity;

            input.pitch = (input.pitch - mouse_delta.y)
                .clamp(-FRAC_PI_2 + ANGLE_EPSILON, FRAC_PI_2 - ANGLE_EPSILON);
            input.yaw -= mouse_delta.x;
        }

        if key_input.pressed(KeyCode::KeyL) {
            input.mouse_look = !input.mouse_look;
        }
        if key_input.pressed(KeyCode::KeyG) {
            input.draw_gizmos = !input.draw_gizmos;
        }

        input.movement = Vec3::new(
            get_axis(&key_input, KeyCode::ArrowRight, KeyCode::ArrowLeft),
            0.0,
            get_axis(&key_input, KeyCode::ArrowUp, KeyCode::ArrowDown)
        );
    }
}

pub fn controller_look(mut query: Query<(&mut Controller, &ControllerInput)>) {
    for (mut controller, input) in query.iter_mut() {
        controller.mouse_look = input.mouse_look;
        controller.pitch = input.pitch;
        controller.yaw = input.yaw;

        controller.draw_gizmos = input.draw_gizmos;
    }
}

pub fn controller_move(
    mut query: Query<(
        &ControllerInput,
        &mut Controller,
        &mut Velocity,
    )>) {
        for (input, mut controller, mut velocity) in
        query.iter_mut()
    {
        let mut move_to_world = Mat3::from_axis_angle(Vec3::Y, input.yaw);
        move_to_world.z_axis *= -1.0;
        let mut move_direction = move_to_world * (input.movement * Vec3::new(controller.speed, 0.0, controller.speed));
        if move_direction.length() > f32::EPSILON {
            move_direction /= move_direction.length()
        }

        velocity.set_velocity(Vec2::new((move_direction.x * controller.speed), (move_direction.z * controller.speed)));
    }
}

pub fn controller_render(
    mut render_query: Query<(&mut Transform, &RenderPlayer), With<RenderPlayer>>,
    logical_query: Query<
        (&Transform, &Controller, &CameraConfig),
        (With<LogicalPlayer>, Without<RenderPlayer>),
    >,
) {
    for (mut render_transform, render_player) in render_query.iter_mut() {
        if let Ok((logical_transform, controller, camera_config)) =
            logical_query.get(render_player.logical_entity)
        {
            let camera_offset = Vec3::Y * camera_config.height_offset;
            render_transform.translation = logical_transform.translation + camera_offset;
            render_transform.rotation = Quat::from_euler(EulerRot::YXZ, controller.yaw, controller.pitch, 0.0);
        }
    }
}