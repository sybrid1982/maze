use bevy::{
    color::palettes::css::*,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use maze::maze_direction::MazeDirection;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::maze::maze::Maze;
use crate::maze::maze_cell_edge::EdgeType;

use player::Player;
use random::Random;

mod maze;
mod position;
mod player;
mod random;

const MAZE_X: i32 = 8;
const MAZE_Y: i32 = 8;
const MAZE_SCALE: f32 = 5.;

const WALL_THICKNESS: f32 = MAZE_SCALE / 8.;

#[derive(Component)]
struct GridLight;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_rng, setup).chain())
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
    let mut maze = Maze::new(MAZE_X, MAZE_Y);
    maze.generate(rng);

    let cells = maze.get_cells();

    for cell in cells {
        let translation = cell.get_position().to_vec3_by_scale(MAZE_SCALE);
        if cell.is_render() {
            commands.spawn( PbrBundle {
                mesh: meshes.add(Rectangle::new(MAZE_SCALE, MAZE_SCALE)),
                material: materials.add(Color::WHITE),
                transform: Transform { translation, rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2), ..default() },
                ..default()
            });
        }
    }

    let edges = maze.get_edges();

    for edge in edges {
        let translation = edge.get_position().to_vec3_by_scale(MAZE_SCALE) + edge.get_maze_direction().to_position_modifier().to_vec3_by_scale(MAZE_SCALE) * 0.5;
        let rotation = edge.get_maze_direction().get_direction_quat();
        if edge.get_edge_type() == EdgeType::Wall {
            commands.spawn( (
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(MAZE_SCALE + WALL_THICKNESS, WALL_THICKNESS, MAZE_SCALE / 2. + WALL_THICKNESS)),
                    material: materials.add(Color::WHITE),
                    transform: Transform::from_xyz(translation.x, translation.y + MAZE_SCALE / 2., translation.z)
                        .with_rotation(rotation),
                    ..default()
                },
            ));
        }
    }

    // spawn reference cube
    commands.spawn( PbrBundle {
        mesh: meshes.add(Cuboid::new(MAZE_SCALE / 2., MAZE_SCALE / 2., MAZE_SCALE / 2.)),
        material: materials.add(Color::srgb(224.,35.,50.)),
        transform: Transform::from_xyz(MAZE_X as f32 / 2., MAZE_SCALE as f32 / 2., MAZE_Y as f32 / 2.)
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // ambient light
    commands.insert_resource(AmbientLight {
        color: ORANGE_RED.into(),
        brightness: 60.,
    });

    // light
    for i in 1..MAZE_X {
        for j in 1..MAZE_Y {
                commands.spawn((
                    PointLightBundle {
                        point_light: PointLight {
                            shadows_enabled: true,
                            ..default()
                        },
                        transform: Transform::from_xyz(MAZE_X as f32 * MAZE_SCALE as f32 / i as f32, 15.0, MAZE_Y as f32 * MAZE_SCALE as f32 / j as f32),
                        ..default()
                    },
                    GridLight
                ));
        }
    }
    // camera
    // x -> left/right (?)
    // y -> up/down
    // z -> back/forth
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(MAZE_X as f32 / 2., 100.0, MAZE_Y as f32 / 2.0).looking_at(Vec3::new(MAZE_X as f32 / 2., 0.0, MAZE_Y as f32 / 2.), Vec3::Y),
        ..default()
    });
}