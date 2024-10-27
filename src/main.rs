use bevy::{prelude::*, render::camera::Viewport};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use maze::{maze_cell_edge::WallPosition, maze_direction::MazeDirection, maze_assets::MazeAssets};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::maze::maze::Maze;

use player::{LogicalPlayer, PlayerPlugin};
use random::Random;
use collider::Collider;
use velocity::{Velocity, VelocityPlugin};
use position::MazePosition;
use game_states::GameState;

mod maze;
mod position;
mod player;
mod random;
mod consts;
mod collider;
mod velocity;
mod game_states;

#[derive(Component)]
struct TopDownCamera;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct GameStartSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct GameRunSet;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
        ))
        .insert_state(GameState::InGame)
        .add_systems(OnEnter(GameState::InGame), (MazeAssets::load_assets, setup_rng, generate_maze, render_maze).chain().in_set(GameStartSet))
        .add_plugins(PlayerPlugin)
        .add_plugins(VelocityPlugin)
        .add_systems(Update, (check_for_collisions, move_minimap_position).in_set(GameRunSet).run_if(in_state(GameState::InGame)))
        .run();
}

fn setup_rng(
    mut commands: Commands
) {
    let rng = ChaCha8Rng::from_entropy();
    commands.insert_resource(Random(rng));
}

/// set up a simple 3D scene
fn render_maze(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    wall_assets: Res<MazeAssets>,
    maze: Res<Maze>
) {
    render_floors(&maze, &mut commands, &wall_assets, &mut meshes, &mut materials);
    render_walls(&maze, &mut commands, &wall_assets);
    add_lights(&mut commands);
    add_top_view_camera(commands);
}

fn generate_maze(mut commands: Commands, mut rng: ResMut<Random>) {
    // create a maze
    let mut maze = Maze::new(consts::MAZE_X, consts::MAZE_Y);
    maze.generate(&mut rng);
    commands.insert_resource(maze);
}

fn add_lights(commands: &mut Commands<'_, '_>) {
    // ambient light
    commands.insert_resource(AmbientLight {
        color: consts::GLOBAL_LIGHT_TINT,
        brightness: consts::GLOBAL_LIGHT_INTENSITY,
    });

    let light_position: Vec2 = Vec2::splat(consts::MAZE_X as f32 * consts::MAZE_SCALE as f32);

    // directional light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(light_position.x, 120.0, light_position.y).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        directional_light: DirectionalLight {
            color: consts::DIRECTIONAL_LIGHT_TINT,
            illuminance: consts::DIRECTIONAL_LIGHT_INTENSITY,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}

fn add_top_view_camera(mut commands: Commands<'_, '_>) {
    let mut camera_transform = Transform::from_xyz(0., consts::TOP_DOWN_CAMERA_HEIGHT, 0.).looking_at(Vec3::new(0., 0.0, 0.), Vec3::Y);
    camera_transform.rotate_y(-std::f32::consts::FRAC_PI_2);

    commands.spawn((Camera3dBundle {
            transform: camera_transform,
            camera: Camera {
                order: 1,
                viewport: Some(Viewport {
                    physical_position: UVec2::new(0, 0),
                    physical_size: UVec2::new(280, 256),
                    ..default()
                }),
                ..default()
            },
            ..default()
        },
        TopDownCamera
    ));
}

fn render_walls(
    maze: &Maze,
    commands: &mut Commands<'_, '_>,
    wall_assets: &Res<MazeAssets>) {
    let edges = maze.get_edges();

    let walls = commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.,0.,0.),
            ..default()
        },
        Name::new("walls"))
    ).id();

    for edge in edges {
        edge.render_edge_resources(commands, &wall_assets, walls);
    }
}

fn render_floors(maze: &Maze, commands: &mut Commands<'_, '_>, wall_assets: &Res<MazeAssets>, meshes: &mut ResMut<'_, Assets<Mesh>>, materials: &mut ResMut<'_, Assets<StandardMaterial>>) {
    let cells = maze.get_cells();

    let floor_material = materials.add(StandardMaterial {
        base_color_texture: Some(wall_assets.floor_texture.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: false,
        ..default()
    });

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
                    material: floor_material.clone(),
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

pub fn check_for_collisions(
    mut player_query: Query<(&mut Velocity, &Transform), With<LogicalPlayer>>,
    collider_query: Query<(&Transform, &WallPosition), (With<Collider>, Without<LogicalPlayer>)>,
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

fn move_minimap_position(
    mut query: Query<&mut Transform, With<TopDownCamera>>,
    player_query: Query<&Transform, (With<LogicalPlayer>, Without<TopDownCamera>)>
) {
    let mut camera_transform = query.single_mut();
    let player_transform = player_query.single();

    camera_transform.translation = Vec3::new(player_transform.translation.x, consts::TOP_DOWN_CAMERA_HEIGHT, player_transform.translation.z);
}