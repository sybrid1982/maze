use bevy::prelude::*;

use super::maze_direction::MazeDirection;
use crate::{collider::Collider, consts, position::{MazePosition, Position}};


#[derive(Default, Copy, Clone, PartialEq)]
pub enum EdgeType {
    #[default]
    Passage,
    Wall
}

pub struct MazeCellEdge {
    position: Position,
    maze_direction: MazeDirection,
    edge_type: EdgeType
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct WallPosition(pub MazeDirection);

impl MazeCellEdge {
    pub fn new(position: Position, adjacent_position: Position) -> MazeCellEdge {
        let maze_direction = MazeDirection::get_direction_position_from_positions(&position, &adjacent_position);
        MazeCellEdge { position, maze_direction, edge_type: EdgeType::default() }
    }

    pub fn set_wall(&mut self) {
        self.edge_type = EdgeType::Wall;
    }

    pub fn get_edge_type(&self) -> EdgeType {
        self.edge_type
    }

    pub fn get_position(&self) -> Position {
        self.position.clone()
    }

    pub fn get_maze_direction(&self) -> MazeDirection {
        self.maze_direction
    }

    pub fn render_edge(&self, commands: &mut Commands<'_, '_>, meshes: &mut ResMut<'_, Assets<Mesh>>, materials: &mut ResMut<'_, Assets<StandardMaterial>>, walls: Entity) {    
        if self.get_edge_type() == EdgeType::Wall {
            let translation: Vec3 = self.get_position().to_vec3_by_scale(consts::MAZE_SCALE) + self.get_maze_direction().to_position_modifier().to_vec3_by_scale(consts::MAZE_SCALE) * 0.5;
            let rotation = self.get_maze_direction().get_direction_quat();
    
            let wall = commands.spawn( (
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(consts::MAZE_SCALE + consts::WALL_THICKNESS, consts::WALL_THICKNESS, consts::MAZE_SCALE)),
                    material: materials.add(Color::WHITE),
                    transform: Transform::from_xyz(translation.x, translation.y + consts::MAZE_SCALE / 2., translation.z)
                        .with_rotation(rotation),
                    ..default()
                },
                Collider,
                MazePosition(self.get_position().get_as_vec2()),
                WallPosition(self.get_maze_direction()),
                Name::new(format!("Wall {:#?} at ({:#?}, {:#?})", self.get_maze_direction(), self.get_position().x, self.get_position().y))
            )).id();
    
            commands.entity(walls).push_children(&[wall]);
        }
    }
}
