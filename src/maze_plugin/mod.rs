use bevy::prelude::*;
use maze_data::maze::Maze;
use maze_renderer::maze_renderer::MazeRenderer;
use crate::{consts::{self, *}, game_states::GameState, maze::maze_room::RoomAssets, random::Random};


mod maze_data;
mod maze_renderer;

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(GameState = GameState::Initialize)]
enum MazeCreationState {
    #[default]
    StartPlugin,        // initialize any resources that we are going to need before generating the rest of the maze here
    InitializeRooms,      
    GenerateMaze,
    RenderMaze,
    Complete            // unsure if we need this
}

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_sub_state::<MazeCreationState>()
            .add_systems(OnEnter(MazeCreationState::StartPlugin), startup);
    }
}

fn startup(mut commands: Commands) {
    
}

fn generate_rooms(mut commands: Commands, mut rng: ResMut<Random>) {
    let mut maze_data = Maze::new(consts::MAZE_X as usize, consts::MAZE_Y as usize);
    maze_data.generate(&mut rng);
}

fn generate_maze(mut commands: Commands, mut rng: ResMut<Random>) {
    let mut maze_data = Maze::new(consts::MAZE_X as usize, consts::MAZE_Y as usize);
    maze_data.generate(&mut rng);
    commands.insert_resource(maze_data);
}

fn render_maze(
    mut commands: Commands, 
    maze: Res<Maze>, 
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    floor_material: Handle<StandardMaterial>,
    room_assets: RoomAssets
) {
    MazeRenderer::render_maze(&maze, &mut commands, meshes, floor_material, room_assets);
}
