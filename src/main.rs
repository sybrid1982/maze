use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use maze::{maze_cell_edge::WallPosition, maze_direction::MazeDirection};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::maze::maze::Maze;
use crate::maze::maze_cell_edge::EdgeType;

use player::{Player, PlayerPlugin};
use random::Random;
use collider::Collider;
use velocity::{Velocity, VelocityPlugin};
use position::MazePosition;

mod maze;
mod position;
mod player;
mod random;
mod consts;
mod collider;
mod velocity;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, (setup_rng, setup).chain())
        .add_plugins(PlayerPlugin)
        .add_plugins(VelocityPlugin)
        .add_systems(FixedUpdate, check_for_collisions)
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
    maze.generate(&mut rng);

    render_floors(&maze, &mut commands, &mut meshes, &mut materials);

    render_walls(maze, &mut commands, &mut meshes, &mut materials, &mut rng);

    // ambient light
    commands.insert_resource(AmbientLight {
        color: consts::GLOBAL_LIGHT_TINT,
        brightness: consts::GLOBAL_LIGHT_INTENSITY,
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

fn render_walls(
    maze: Maze,
    commands: &mut Commands<'_, '_>,
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    materials: &mut ResMut<'_, Assets<StandardMaterial>>,
    rng: &mut ResMut<Random>) {
    let edges = maze.get_edges();

    let walls = commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.,0.,0.),
            ..default()
        },
        Name::new("walls"))
    ).id();

    for edge in edges {
        edge.render_edge(commands, meshes, materials, walls,  rng);
    }
}



fn render_floors(maze: &Maze, commands: &mut Commands<'_, '_>, meshes: &mut ResMut<'_, Assets<Mesh>>, materials: &mut ResMut<'_, Assets<StandardMaterial>>) {
    let cells = maze.get_cells();

    let floors = commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.,0.,0.),
            ..default()
        },
        Name::new("floors"))
    ).id();
    
    for cell in cells {
        let translation = cell.get_position().to_vec3_by_scale(consts::MAZE_SCALE);
        if cell.is_render() {
            let floor = commands.spawn( (
                PbrBundle {
                    mesh: meshes.add(Rectangle::new(consts::MAZE_SCALE, consts::MAZE_SCALE)),
                    material: materials.add(Color::WHITE),
                    transform: Transform { translation, rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2), ..default() },
                    ..default()
                },
                MazePosition(cell.get_position().get_as_vec2()),
                Name::new(format!("Floor: {:#?}", cell.get_position()))
            )).id();
            commands.entity(floors).push_children(&[floor]);
        }
    }
}

fn check_for_collisions(
    mut player_query: Query<(&mut Velocity, &Transform), With<Player>>,
    collider_query: Query<(&Transform, &WallPosition), (With<Collider>, Without<Player>)>,
    // mut collision_events: EventWriter<CollisionEvent>
) {
    let (mut player_velocity, player_transform) = player_query.single_mut();

    let player_collider = Collider::transform_to_aabb2d(player_transform);

    let mut number_of_collisions = 0;

    for (collider_transform, wall_position) in collider_query.iter() {
        // need to get the dimensions of the wall and the dimensions of the player size
        // then use those to determine if one is inside the other
        let wall_collider = Collider::get_wall_aabb2d(&collider_transform, &wall_position);
        let collision = Collider::box_collision(player_collider, wall_collider);

        if let Some(collision) = collision {
            // collision_events.send(CollisionEvent);
            match collision {
                MazeDirection::EAST => player_velocity.x = f32::min(player_velocity.x, 0.),
                MazeDirection::WEST => player_velocity.x = f32::max(player_velocity.x, 0.),
                MazeDirection::NORTH => player_velocity.y = f32::min(player_velocity.y, 0.),
                MazeDirection::SOUTH => player_velocity.y = f32::max(player_velocity.y, 0.)
            }

            number_of_collisions = number_of_collisions + 1;
        }
    }
}