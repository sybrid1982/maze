use bevy::{prelude::*, render::{camera::Viewport, view::RenderLayers}};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use maze::{maze_assets::MazeAssets, maze_room::MazeRooms};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::maze::maze::Maze;

use player::player::{LogicalPlayer, PlayerPlugin};
use random::Random;
use game_states::GameState;
use physics::physics::PhysicsPlugin;

mod maze;
mod position;
mod player;
mod random;
mod consts;
mod physics;
mod game_states;
mod apply_render_layers_to_children;
mod assets;

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
        .add_systems(Update, (move_minimap_position).run_if(in_state(GameState::InGame)))
        .add_plugins(PhysicsPlugin)
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
    let maze_rooms = MazeRooms::new(maze_assets, &mut materials);
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
        RenderLayers::layer(0),
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

fn move_minimap_position(
    mut query: Query<&mut Transform, With<TopDownCamera>>,
    player_query: Query<&Transform, (With<LogicalPlayer>, Without<TopDownCamera>)>
) {
    let mut camera_transform = query.single_mut();
    let player_transform = player_query.single();

    camera_transform.translation = Vec3::new(player_transform.translation.x, consts::TOP_DOWN_CAMERA_HEIGHT, player_transform.translation.z);
}