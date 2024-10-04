use bevy::{
    color::palettes::css::*,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::maze::maze::Maze;
use crate::maze::maze_cell_edge::EdgeType;

use player::{Player, PlayerPlugin};
use random::Random;
use collider::{Collider, CollisionEvent};
use velocity::{Velocity, VelocityPlugin};

mod maze;
mod position;
mod player;
mod random;
mod consts;
mod collider;
mod velocity;

const WALL_THICKNESS: f32 = consts::MAZE_SCALE / 8.;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, (setup_rng, setup).chain())
        .add_plugins(PlayerPlugin)
        .add_plugins(VelocityPlugin)
        .run();
}

fn setup_rng(
    mut commands: Commands
) {
    let rng = ChaCha8Rng::from_entropy();
    // insert the random resource so we can use it everywhere else
    commands.insert_resource(Random(rng));
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<Random>
) {
    // create a maze
    let mut maze = Maze::new(consts::MAZE_X, consts::MAZE_Y);
    maze.generate(rng);

    render_floors(&maze, &mut commands, &mut meshes, &mut materials);

    render_walls(maze, &mut commands, &mut meshes, &mut materials);

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::srgba(0.896, 0.715, 0.704, 1.000),
        brightness: 60.,
    });

    add_top_view_camera(commands);
}

fn add_top_view_camera(mut commands: Commands<'_, '_>) {
    // camera
    // x -> left/right (?)
    // y -> up/down
    // z -> back/forth
    let camera_x_position: f32 = consts::MAZE_X as f32 * consts::MAZE_SCALE as f32 / 2.;
    let camera_z_position: f32 = consts::MAZE_Y as f32 * consts::MAZE_SCALE as f32;

    let camera_x_target: f32 = consts::MAZE_X as f32 * consts::MAZE_SCALE as f32 / 2.;
    let camera_z_target: f32 = consts::MAZE_Y as f32 * consts::MAZE_SCALE as f32 / 2.;

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(camera_x_position, 120.0, camera_z_position).looking_at(Vec3::new(camera_x_target, 0.0, camera_z_target), Vec3::Y),
        ..default()
    });
}

fn render_walls(maze: Maze, commands: &mut Commands<'_, '_>, meshes: &mut ResMut<'_, Assets<Mesh>>, materials: &mut ResMut<'_, Assets<StandardMaterial>>) {
    let edges = maze.get_edges();

    for edge in edges {
        let translation = edge.get_position().to_vec3_by_scale(consts::MAZE_SCALE) + edge.get_maze_direction().to_position_modifier().to_vec3_by_scale(consts::MAZE_SCALE) * 0.5;
        let rotation = edge.get_maze_direction().get_direction_quat();
        if edge.get_edge_type() == EdgeType::Wall {
            commands.spawn( (
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(consts::MAZE_SCALE + WALL_THICKNESS, WALL_THICKNESS, consts::MAZE_SCALE)),
                    material: materials.add(Color::WHITE),
                    transform: Transform::from_xyz(translation.x, translation.y + consts::MAZE_SCALE / 2., translation.z)
                        .with_rotation(rotation),
                    ..default()
                },
                Collider
            ));
        }
    }
}

fn render_floors(maze: &Maze, commands: &mut Commands<'_, '_>, meshes: &mut ResMut<'_, Assets<Mesh>>, materials: &mut ResMut<'_, Assets<StandardMaterial>>) {
    let cells = maze.get_cells();
    
    for cell in cells {
        let translation = cell.get_position().to_vec3_by_scale(consts::MAZE_SCALE);
        if cell.is_render() {
            commands.spawn( PbrBundle {
                mesh: meshes.add(Rectangle::new(consts::MAZE_SCALE, consts::MAZE_SCALE)),
                material: materials.add(Color::WHITE),
                transform: Transform { translation, rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2), ..default() },
                ..default()
            });
        }
    }
}

fn check_for_collisions(
    mut commands: Commands,
    mut player_query: Query<(&mut Velocity, &Transform), With<Player>>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>
) {
    
}