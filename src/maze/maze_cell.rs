use std::collections::HashMap;
use std::f32::consts::*;

use bevy::{prelude::*, render::view::RenderLayers};
use rand::Rng;

use crate::{consts, player::{player::LogicalPlayer, player_events::PlayerCellChangeEvent}, position::{MazePosition, Position}, random::Random};

use super::{maze_cell_edge::{EdgeType, MazeCellEdge}, maze_direction::MazeDirection, maze_door::MazeDoor, maze_room::RoomAssets};

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
    pub fn new(x: f32, y: f32, room_index: usize) -> Self {
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

    // pub fn hide_cell(&self, &mut commands: &mut Commands) {
    //     commands.entity(self.entity.unwrap()).
    // }

    // pub fn get_doors(&mut self) -> Vec<MazeDoor> {
    //     let mut doors = vec![];
    //     for (key, value) in self.edges {
    //         match value {
    //             Some(edge) => {
    //                 if edge.is_door() {
    //                     doors.push(edge)
    //                 }
    //             }
    //             None => todo!(),
    //         }
    //     }
    // }

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
                mesh: meshes.add(Rectangle::new(consts::MAZE_SCALE, consts::MAZE_SCALE)),
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
        let half_cell = consts::MAZE_SCALE / 2.;
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
                    if edge.get_edge_type() == EdgeType::Doorway || edge.get_edge_type() == EdgeType::Wall {
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

