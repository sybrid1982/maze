use bevy::{prelude::*, render::camera::Viewport};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use maze::{maze_assets::MazeAssets, maze_cell_edge::WallPosition, maze_direction::MazeDirection, maze_room::MazeRooms};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::maze::maze::Maze;

use player::{LogicalPlayer, PlayerPlugin};
use random::Random;
use collider::Collider;
use velocity::{Velocity, VelocityPlugin};
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

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct GameLoadSet;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
        ))
        .insert_state(GameState::LoadingAssets)
        .add_systems(OnEnter(GameState::LoadingAssets), (MazeAssets::load_assets, setup_rng, initialize_maze_rooms).chain().in_set(GameLoadSet))
        .add_systems(OnEnter(GameState::Initialize), generate_maze)
        .add_systems(OnEnter(GameState::InGame), render_maze)
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

// Render everything
fn render_maze(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut maze: ResMut<Maze>,
    maze_rooms: Res<MazeRooms>,
) {
    render_cells(&mut maze, &mut commands, &mut meshes, maze_rooms);
    add_lights(&mut commands);
    add_top_view_camera(commands);
}

fn initialize_maze_rooms(mut commands: Commands, maze_assets: Res<MazeAssets>, mut materials: ResMut<'_, Assets<StandardMaterial>>, mut next_state: ResMut<NextState<GameState>>) {
    let mut maze_rooms = MazeRooms::new(maze_assets, &mut materials);
    commands.insert_resource(maze_rooms);
    next_state.set(GameState::Initialize);
}

fn generate_maze(
    mut commands: Commands, 
    mut rng: ResMut<Random>, 
    mut maze_rooms: ResMut<MazeRooms>,     
    mut next_state: ResMut<NextState<GameState>>
) {
    // create a maze
    let mut maze = Maze::new(consts::MAZE_X, consts::MAZE_Y);
    maze.generate(&mut rng, &mut maze_rooms);
    commands.insert_resource(maze);
    next_state.set(GameState::InGame)
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

fn render_cells(maze: &mut Maze, commands: &mut Commands<'_, '_>, meshes: &mut ResMut<'_, Assets<Mesh>>, maze_rooms: Res<MazeRooms>) {
    let cells = maze.get_cells();

    println!("Creating floor entity");
    let floors = generate_empty_object_with_name(commands, "floors");

    for cell in cells {
        let floor_material = maze_rooms.get_material_for_floor_by_room_index(cell.get_room_index());
        let room_assets = maze_rooms.get_assets_for_room_index(cell.get_room_index());
    
        cell.render_cell(commands, meshes, floor_material, room_assets, floors);
    }
}

fn generate_empty_object_with_name(commands: &mut Commands<'_, '_>, name: &str) -> Entity {
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.,0.,0.),
            ..default()
        },
        Name::new(String::from(name)))
    ).id()
}

fn check_for_collisions(
    mut player_query: Query<(&mut Velocity, &Transform), With<LogicalPlayer>>,
    collider_query: Query<(&GlobalTransform, &WallPosition), (With<Collider>, Without<LogicalPlayer>)>,
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