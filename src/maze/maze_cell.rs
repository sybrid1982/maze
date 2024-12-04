use std::collections::HashMap;
use std::f32::consts::*;

use bevy::{prelude::*, render::view::RenderLayers};
use rand::Rng;

use crate::{consts, position::{MazePosition, Position}, random::Random, physics::collider::Collider};

use super::{maze_direction::MazeDirection, maze_door::MazeDoor, maze_room::RoomAssets, paintings::Painting};

#[derive(Component, Clone)]
pub struct MazeCell {
    position: Position,
    render: bool,
    defined_edges: Vec<MazeDirection>,
    edges: HashMap<MazeDirection, Option<MazeCellEdge>>,
    entity: Option<Entity>,
    room_index: usize
}

impl MazeCell {
    pub fn new(x: usize, y: usize, room_index: usize) -> Self {
        MazeCell {
            position: Position::new( x, y ),
            render: false,
            defined_edges: vec![],
            edges: HashMap::new(),
            entity: None,
            room_index
        }
    }

    pub fn is_render(&self) -> bool {
        self.render
    }

    pub fn toggle_render(&mut self) {
        self.render = !self.render
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn set_room_index(&mut self, room_index: usize) {
        self.room_index = room_index;
    }

    pub fn add_edge(&mut self, maze_direction: &MazeDirection, edge_type: Option<EdgeType>, rand: &mut ResMut<Random>) {
        if self.has_edge(maze_direction) {
            panic!("Pushed same edge twice, stopping");
        }

        match edge_type {
            Some(edge_type) => {
                let mut new_edge = MazeCellEdge::new(maze_direction, edge_type);
                new_edge.generate_furniture(rand);
                let new_edge_option = Some(new_edge);
                self.edges.insert(*maze_direction, new_edge_option);
            },
            None => {
                self.edges.insert(*maze_direction, None);
            },
        }

        self.defined_edges.push(*maze_direction);
    }

    pub fn has_edge(&self, maze_direction: &MazeDirection) -> bool {
        self.defined_edges.contains(maze_direction)
    }

    pub fn is_edge_complete(&self) -> bool {
        self.defined_edges.len() == 4
    }

    pub fn get_random_unused_direction_for_cell(&self, rand: &mut ResMut<Random>) -> MazeDirection {
        let mut skips: usize = rand.gen_range(0..4-self.defined_edges.len());
        for i in 0..4 {
            let new_direction = &MazeDirection::get_direction_from_index(i);
            if !self.has_edge(new_direction) {
                if skips == 0 {
                    return *new_direction;
                }
                else 
                {
                    skips -= 1;
                }
            }
        }
        panic!("Ran out of possible edges before ran out of skips");
    }

    pub fn render_cell(&mut self, commands: &mut Commands<'_, '_>, meshes: &mut ResMut<'_, Assets<Mesh>>, floor_material: Handle<StandardMaterial>, room_assets: RoomAssets, floors: Entity) {
        let translation = self.get_position().to_vec3_by_scale(consts::MAZE_SCALE);
        if self.is_render() {
            self.render_floor(commands, meshes, floor_material, translation, floors);
            self.render_ceiling(commands, &room_assets);
            self.render_walls(commands, &room_assets);
        }
    }

    fn render_floor(&mut self, commands: &mut Commands<'_, '_>, meshes: &mut ResMut<'_, Assets<Mesh>>, floor_material: Handle<StandardMaterial>, translation: Vec3, floors: Entity) {
        let floor = commands.spawn( (
            PbrBundle {
                mesh: meshes.add(Rectangle::new(consts::MAZE_SCALE as f32, consts::MAZE_SCALE as f32)),
                material: floor_material,
                transform: Transform { translation, rotation: Quat::from_rotation_x(-FRAC_PI_2), ..default() },
                ..default()
            },
            MazePosition(self.get_position().get_as_vec2()),
            Name::new(format!("Floor: {:#?}", self.get_position()))
        )).id();
        self.entity = Some(floor);
        commands.entity(floors).push_children(&[floor]);
    }

    fn render_ceiling(&mut self, commands: &mut Commands, room_assets: &RoomAssets) {
        // TODO: Make this only render for the FPS camera and not the top down camera
        let half_cell = consts::MAZE_SCALE as f32 / 2.;
        let transform = Transform::from_xyz(-half_cell, half_cell, 6.0)
            .with_rotation(Quat::from_euler(EulerRot::XYZ, FRAC_PI_2, 0.0, 0.0 ))
            .with_scale(Vec3::splat(2.0));
        let ceiling = commands.spawn( (
            SceneBundle {
                scene: room_assets.ceiling.clone(),
                transform,
                ..default()
            },
            RenderLayers::layer(1)
        )).id();
        commands
            .entity(self.entity.expect("Somehow adding ceiling to room with no floor?"))
            .push_children(&[ceiling]);
}
    
    fn render_walls(
        &mut self,
        commands: &mut Commands<'_, '_>,
        room_assets: &RoomAssets) {
    
        for (_maze_direction, edge) in &mut self.edges {
            match edge {
                Some(edge) => {
                    if edge.get_edge_type() == EdgeType::Doorway(false) || edge.get_edge_type() == EdgeType::Wall {
                        let new_edge = edge.create_edge_entity(commands, room_assets);
                        commands
                            .entity(self.entity.expect("somehow adding edge entity to non-existant floor"))
                            .push_children(&[new_edge.expect("somehow adding edge that isn't an edge")]);
                    }
                }
                None => {},
            }
        }
    }

    pub fn get_room_index(&self) -> usize {
        self.room_index
    }

    pub fn get_entity(&self) -> Entity {
        self.entity.expect("trying to get entity for maze cell that never generated one")
    }

    pub fn get_edge(&mut self, maze_direction: &MazeDirection) -> &mut Option<MazeCellEdge> {
        self.edges.get_mut(maze_direction).expect("Trying to get maze edge that was not yet defined?")
    }
}

#[derive(Default, Copy, Clone, PartialEq)]
pub enum EdgeType {
    #[default]
    Wall,
    Doorway(bool),
    InverseDoorway(bool)
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
        if (self.edge_type == EdgeType::Doorway(true) || self.edge_type == EdgeType::InverseDoorway(true)) // && self.is_open {
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
        match self.get_edge_type() {
            EdgeType::Wall => {
                self.create_wall(commands, room_assets)
            },
            EdgeType::Doorway(_) => {
                self.create_door(commands, room_assets)
            },
            _ => None
        }
    }

fn create_door(&mut self, commands: &mut Commands<'_, '_>, room_assets: &RoomAssets) -> Option<Entity> {
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
    }

fn create_wall(&mut self, commands: &mut Commands<'_, '_>, room_assets: &RoomAssets) -> Option<Entity> {
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
    }
}
