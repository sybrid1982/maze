use std::collections::HashMap;

use bevy::prelude::*;
use rand::Rng;

use crate::{consts, position::{MazePosition, Position}, random::Random};

use super::{maze_assets::MazeAssets, maze_cell_edge::{EdgeType, MazeCellEdge}, maze_direction::MazeDirection, maze_room::RoomAssets};

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
        self.position.clone()
    }

    pub fn add_edge(&mut self, maze_direction: &MazeDirection, edge_type: Option<EdgeType>) {
        if self.has_edge(maze_direction) {
            panic!("Pushed same edge twice, stopping");
        }

        match edge_type {
            Some(edge_type) => {
                let new_edge = Some(MazeCellEdge::new(maze_direction, edge_type));
                self.edges.insert(*maze_direction, new_edge);
            },
            None => {
                self.edges.insert(*maze_direction, None);
            },
        }

        self.defined_edges.push(maze_direction.clone());
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
                    return new_direction.clone();
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
                transform: Transform { translation, rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2), ..default() },
                ..default()
            },
            MazePosition(self.get_position().get_as_vec2()),
            Name::new(format!("Floor: {:#?}", self.get_position()))
        )).id();
        self.entity = Some(floor);
        commands.entity(floors).push_children(&[floor]);
    }

    fn render_ceiling(&mut self, commands: &mut Commands, room_assets: &RoomAssets) {
        let half_cell = consts::MAZE_SCALE / 2.;
        let transform = Transform::from_xyz(-half_cell, half_cell, 6.0)
            .with_rotation(Quat::from_euler(EulerRot::XYZ, std::f32::consts::FRAC_PI_2, 0.0, 0.0 ))
            .with_scale(Vec3::splat(2.0));
        let ceiling = commands.spawn( (
            SceneBundle {
                scene: room_assets.ceiling.clone(),
                transform,
                ..default()
            },
        )).id();
        commands
            .entity(self.entity.expect("Somehow adding ceiling to room with no floor?"))
            .push_children(&[ceiling]);
}
    
    fn render_walls(
        &self,
        commands: &mut Commands<'_, '_>,
        room_assets: &RoomAssets) {
    
        for (_maze_direction, edge) in &self.edges {
            match edge {
                Some(edge) => {
                    let new_edge = edge.create_edge_entity(commands, &room_assets);
                    commands
                        .entity(self.entity.expect("somehow adding edge entity to non-existant floor"))
                        .push_children(&[new_edge.expect("somehow adding edge that isn't an edge")]);
                }
                None => {},
            }
        }
    }

    pub fn get_room_index(&self) -> usize {
        self.room_index
    }
}
