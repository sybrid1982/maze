use bevy::prelude::*;

use super::{maze_direction::MazeDirection, paintings::Painting};
use crate::{collider::Collider, consts, position::{MazePosition, Position}, random::Random};


#[derive(Default, Copy, Clone, PartialEq)]
pub enum EdgeType {
    #[default]
    Passage,
    Wall
}

pub struct MazeCellEdge {
    position: Position,
    maze_direction: MazeDirection,
    edge_type: EdgeType,
    painting: Option<Painting>
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct WallPosition(pub MazeDirection);

impl MazeCellEdge {
    pub fn new(position: Position, adjacent_position: Position) -> MazeCellEdge {
        let maze_direction = MazeDirection::get_direction_position_from_positions(&position, &adjacent_position);
        MazeCellEdge { position, maze_direction, edge_type: EdgeType::default(), painting: None }
    }

    pub fn set_wall(&mut self, painting: Option<Painting>) {
        self.edge_type = EdgeType::Wall;
        self.set_painting(painting);
    }

    fn get_edge_type(&self) -> EdgeType {
        self.edge_type
    }

    fn get_position(&self) -> Position {
        self.position.clone()
    }

    pub fn get_maze_direction(&self) -> MazeDirection {
        self.maze_direction
    }

    fn set_painting(&mut self, painting: Option<Painting>) {
        self.painting = painting;
    }

    pub fn render_edge(
        &self, 
        commands: &mut Commands<'_, '_>, 
        meshes: &mut ResMut<'_, Assets<Mesh>>,
        materials: &mut ResMut<'_, Assets<StandardMaterial>>, 
        walls: Entity, 
    ) {    
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

            if self.painting.is_some() {
                let painting = self.painting.as_ref().unwrap().get_painting(meshes, materials);
                let painting_entity = commands.spawn(painting).id();
                commands.entity(wall).push_children(&[painting_entity]);
            }

            commands.entity(walls).push_children(&[wall]);
        }
    }
}
