use std::collections::btree_set::Range;

use bevy::{
    color::palettes::basic::SILVER,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use maze::Maze;
use player::Player;
use random::Random;

mod maze;
mod position;
mod player;
mod random;
mod maze_direction;
mod maze_cell;

const MAZE_X: i32 = 20;
const MAZE_Y: i32 = 20;
const MAZE_SCALE: f32 = 5.;

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

    for column in cells {
        for cell in column {
            let translation = cell.get_position_as_vec3_to_scale(MAZE_SCALE);
            if cell.is_render() {
                commands.spawn( PbrBundle {
                    mesh: meshes.add(Rectangle::new(MAZE_SCALE, MAZE_SCALE)),
                    material: materials.add(Color::WHITE),
                    transform: Transform { translation, rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2), ..default() },
                    ..default()
                });
            }
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

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(MAZE_X as f32 / 2., 8.0, MAZE_Y as f32 / 2.),
        ..default()
    });
    // camera
    // x -> left/right (?)
    // y -> up/down
    // z -> back/forth
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(MAZE_X as f32 / 2., 200.0, MAZE_Y as f32 / 2.0).looking_at(Vec3::new(MAZE_X as f32 / 2., 0.0, MAZE_Y as f32 / 2.), Vec3::Y),
        ..default()
    });
}