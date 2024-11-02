use bevy::prelude::*;

use super::{maze_direction::MazeDirection, maze_door::MazeDoor, maze_room::RoomAssets, paintings::Painting};
use crate::physics::collider::Collider;


#[derive(Default, Copy, Clone, PartialEq)]
pub enum EdgeType {
    #[default]
    Wall,
    Doorway
}

#[derive(Clone)]
pub struct MazeCellEdge {
    maze_direction: MazeDirection,
    edge_type: EdgeType,
    painting: Option<Painting>
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct WallPosition(pub MazeDirection);

impl MazeCellEdge {
    pub fn new(maze_direction: &MazeDirection, edge_type: EdgeType) -> MazeCellEdge {
        MazeCellEdge { maze_direction: maze_direction.clone(), edge_type, painting: None }
    }

    fn get_edge_type(&self) -> EdgeType {
        self.edge_type
    }

    pub fn get_maze_direction(&self) -> MazeDirection {
        self.maze_direction
    }

    pub fn create_edge_entity(
        &self,
        commands: &mut Commands<'_, '_>,
        room_assets: &RoomAssets,
    ) -> Option<Entity> {
        if self.get_edge_type() == EdgeType::Wall {
            let translation: Vec3 = self.get_maze_direction().get_wall_position_for_cell();
            let rotation = self.get_maze_direction().get_direction_quat();
            let transform = Transform::from_xyz(translation.x, translation.y, translation.z)
                .with_rotation(rotation)
                .with_scale(Vec3::splat(2.));
    
            let wall = commands.spawn( (
                SceneBundle {
                    scene: room_assets.wall.clone(),
                    transform,
                    ..default()
                },
                Collider,
                WallPosition(self.get_maze_direction()),
                Name::new(format!("Wall {:#?}", self.get_maze_direction()))
            )).id();
            return Some(wall);
        } else if self.get_edge_type() == EdgeType::Doorway {
            let translation: Vec3 = self.get_maze_direction().get_door_position_for_cell();
            let rotation = self.get_maze_direction().get_direction_quat();
            let transform = Transform::from_xyz(translation.x, translation.y, translation.z)
                .with_rotation(rotation)
                .with_scale(Vec3::new(2.0, 2.0, 2.0));

            let doorway = commands.spawn((
                SceneBundle {
                    scene: room_assets.doorway.clone(),
                    transform,
                    ..default()
                },
                Collider,
                Name::new(format!("Door {:#?}", self.get_maze_direction()))
            )).id();

            let door = MazeDoor::new(commands, room_assets.door.clone(), self.get_maze_direction()).get_door_child();

            commands.entity(doorway).push_children(&[door]);

            return Some(doorway);
        } else {
            return None;
        }
    }
}
