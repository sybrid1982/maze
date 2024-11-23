use bevy::{ecs::observer::TriggerTargets, prelude::*};
use rand::Rng;

use super::{maze_direction::MazeDirection, maze_door::MazeDoor, maze_room::RoomAssets, paintings::Painting};
use crate::{consts, physics::collider::Collider, random::Random};


#[derive(Default, Copy, Clone, PartialEq)]
pub enum EdgeType {
    #[default]
    Wall,
    Doorway,
    InverseDoorway
}

#[derive(Clone)]
pub struct MazeCellEdge {
    maze_direction: MazeDirection,
    edge_type: EdgeType,
    painting: Option<Painting>,
    wall_furniture: Vec<String>,
    door: Option<Entity>
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct WallPosition(pub MazeDirection);

impl MazeCellEdge {
    pub fn new(maze_direction: &MazeDirection, edge_type: EdgeType) -> MazeCellEdge {
        MazeCellEdge { maze_direction: *maze_direction, edge_type, painting: None, wall_furniture: vec![], door: None }
    }

    pub fn get_edge_type(&self) -> EdgeType {
        self.edge_type
    }

    pub fn get_maze_direction(&self) -> MazeDirection {
        self.maze_direction
    }

    pub fn generate_furniture(&mut self, rand: &mut ResMut<Random>) {
        if self.get_edge_type() == EdgeType::Wall {
            let light_chance = rand.gen_range(0.0..1.);
            if light_chance < consts::WALL_LIGHT_PROBABILITY {
                // Add a wall light
                self.wall_furniture.push(String::from("wall_light"));
            }    
        }
    }

    // Ideally we would have some way for a cell to say whether it is possible to move from one cell to another.
    // ...or maybe this could be about room links?
    pub fn is_passable(&self) -> bool {
        if (self.edge_type == EdgeType::Doorway || self.edge_type == EdgeType::InverseDoorway) // && self.is_open {
        {
            return true;
        }
        false
    }

    pub fn create_edge_entity(
        &mut self,
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

            if self.wall_furniture.contains(&String::from("wall_light"))
            {
                if let Some(wall_light_handle) = room_assets.other_furniture.get("wall_light") {
                    let light_position = Vec3::new(1.3, 1.8, 0.1);
                    let light_model = commands.spawn((
                        SceneBundle {
                            scene: wall_light_handle.clone(),
                            transform: Transform::from_xyz(light_position.x, light_position.y, light_position.z)
                                .with_scale(Vec3::splat(0.5)),
                            ..default()
                        },
                    )).with_children(|parent: &mut ChildBuilder<'_>| {
                        parent.spawn(PointLightBundle {
                            transform: Transform::from_xyz(0.0, 0.0, 0.4),
                                point_light: PointLight {
                                color: Color::srgb(0.0, 0.1, 1.0),
                                intensity: 20000.0,
                                ..default()
                            },
                            ..default()
                        });
                    }).id();

                    commands.entity(wall).push_children(&[light_model]);
                }

            }

            Some(wall)
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

            let maze_door = MazeDoor::new(commands, room_assets.door.clone(), self.get_maze_direction());
            let door = maze_door.get_door_child();
            commands.entity(door).insert(maze_door);
            commands.entity(doorway).push_children(&[door]);
            self.door = Some(door);
            return Some(doorway);
        } else {
            return None;
        }
    }
}
