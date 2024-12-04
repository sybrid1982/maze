use bevy::{prelude::*, utils::HashMap};

use crate::maze::maze_room::RoomAssets;
use crate::{maze_plugin::maze_data::maze::Maze, position::Position};

use super::cell_renderer::CellRenderer;

#[derive(Resource)]
pub struct MazeRenderer {
    cells: HashMap<Position, CellRenderer>
}

impl MazeRenderer {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new()
        }
    }

    pub fn render_maze(
        maze: &Maze, 
        commands: &mut Commands, 
        meshes: &mut ResMut<'_, Assets<Mesh>>,
        floor_material: Handle<StandardMaterial>,
        room_assets: RoomAssets
    ) {
        let mut maze_renderer = MazeRenderer::new();

        maze.cells.iter().for_each(|cell| {
            let renderer = CellRenderer::new(
                cell,
                commands,
                meshes,
                floor_material.clone(),
                room_assets.clone()
            );
            maze_renderer.cells.insert(cell.get_position(), renderer);
        });
        commands.insert_resource(maze_renderer);
    }
}