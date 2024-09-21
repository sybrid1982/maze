use bevy::{
    color::palettes::basic::SILVER,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

use maze::Maze;

mod maze;
mod position;

const MAZE_X: i32 = 16;
const MAZE_Y: i32 = 16;
const MAZE_SCALE: f32 = 5.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // create a maze
    let mut maze = Maze::new(MAZE_X, MAZE_Y);
    maze.generate();

    let cells = maze.get_cells();

    for column in cells {
        for cell in column {
            let translation = cell.get_position_as_vec3_to_scale(MAZE_SCALE);
            commands.spawn( PbrBundle {
                mesh: meshes.add(Rectangle::new(MAZE_SCALE, MAZE_SCALE)),
                material: materials.add(Color::WHITE),
                transform: Transform { translation, rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2), ..default() },
                ..default()
            });
        }
    }
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 25.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}