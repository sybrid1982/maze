use std::f32::consts::FRAC_PI_2;

use bevy::{prelude::*, render::view::RenderLayers};

use crate::{consts, maze::maze_room::RoomAssets, physics::collider::Collider, position::Position};
use crate::maze_plugin::maze_data::{maze_cell::MazeCell, maze_cell_edge::{EdgeType, MazeCellEdge}};


struct CellTemp {
    cell: Entity,
    edges: Vec<Option<Entity>>,
    floor: Entity,
    ceiling: Entity
}

pub struct CellRenderer {
    render: bool,
    edges: Vec<Option<Entity>>,
    cell: Entity,
    floor: Entity,
    ceiling: Entity
}

#[derive(Component)]
pub struct Door;

/// As entities, the relationship will be the cell_entity (which has nothing itself to render but positions its children)
/// with the four walls, ceiling, and floor as children (create marker components for each?)
/// But we will also store them here because it will make access easier
impl CellRenderer {
    pub fn new(
        cell: &MazeCell,
        commands: &mut Commands<'_, '_>,
        meshes: &mut ResMut<'_, Assets<Mesh>>,
        floor_material: Handle<StandardMaterial>,
        room_assets: RoomAssets
    ) -> Self {
        let temp: CellTemp = render_cell(
            cell, commands, meshes, floor_material, room_assets
        );
        
        Self {
            render: true,
            edges: temp.edges,
            cell: temp.cell,
            floor: temp.floor,
            ceiling: temp.ceiling
        }
    }

    pub fn toggle_render(&mut self) {
        self.render = !self.render
    }
}

fn render_cell(
    cell: &MazeCell,
    commands: &mut Commands<'_, '_>,
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    floor_material: Handle<StandardMaterial>,
    room_assets: RoomAssets
) -> CellTemp {
    let translation = cell.get_position().to_vec3_by_scale(consts::MAZE_SCALE);
    let cell_entity = commands.spawn(Transform::from_translation(translation)).id();
    let floor_entity = render_floor(cell, commands, meshes, floor_material, translation);
    let ceiling_entity = render_ceiling(commands, &room_assets);
    let edge_entities = render_walls(cell, commands, &room_assets);

    CellTemp {
        cell: cell_entity,
        edges: edge_entities,
        floor: floor_entity,
        ceiling: ceiling_entity
    }
}

fn render_floor(
    maze_cell: &MazeCell, 
    commands: &mut Commands<'_, '_>,
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    floor_material: Handle<StandardMaterial>,
    translation: Vec3
) -> Entity {
    let floor = commands.spawn( (
        PbrBundle {
            mesh: meshes.add(Rectangle::new(consts::MAZE_SCALE as f32, consts::MAZE_SCALE as f32)),
            material: floor_material,
            transform: Transform { translation, rotation: Quat::from_rotation_x(-FRAC_PI_2), ..default() },
            ..default()
        },
        maze_cell.get_position(),
        Name::new(format!("Floor: {:#?}", maze_cell.get_position()))
    )).id();
    floor
}

fn render_ceiling(commands: &mut Commands, room_assets: &RoomAssets) -> Entity {
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
    ceiling
}

fn render_walls(
    maze_cell: &MazeCell,
    commands: &mut Commands<'_, '_>,
    room_assets: &RoomAssets
) -> Vec<Option<Entity>> {
    maze_cell.edges.iter().map(|edge| {
        match edge {
            edge if edge.is_some() && edge.as_ref().unwrap().edge_type == EdgeType::Wall => {
                render_wall(edge.as_ref().unwrap(), maze_cell.get_position(), commands, room_assets)
            },
            _ => {
                None
            },
        }
    }).collect()
}

fn create_door(edge: MazeCellEdge, position: Position, commands: &mut Commands<'_, '_>, room_assets: &RoomAssets) -> Option<Entity> {
    let translation: Vec3 = edge.direction.get_door_position_for_cell();
    let rotation = edge.direction.get_direction_quat();
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
        Name::new(format!("Door {:#?}", edge.direction))
    )).id();
    let door = get_door_render(commands, room_assets.clone().door);
    commands.entity(door).insert(Door);
    commands.entity(doorway).push_children(&[door]);
    return Some(doorway);
}

fn get_door_render(
    commands: &mut Commands<'_, '_>,
    door: Handle<Scene>
) -> Entity {
    let door = commands.spawn( (
        SceneBundle {
            scene: door,
            transform: Transform::from_xyz(0.75,0.,0.0)
                .with_scale(Vec3::new(1.0, 1.0, 1.0)),
            ..default()
        },
        Collider,
    )).id();

    door
}


fn render_wall(edge: &MazeCellEdge, position: Position, commands: &mut Commands<'_, '_>, room_assets: &RoomAssets) -> Option<Entity> {
    /// I should change this so that there is a parent object that sits on the center of the edge of the cell
    /// And then the child is adjusted so that it spawns some offset off, that way the positions for the walls
    /// are just "go from center of cell to edge in direction"
    /// and then when they get parented to the cell objects, they'll all be in the correct spot
    /// basically put in a layer of separation between where the model's "0" is and the object's "0"
    let translation: Vec3 = edge.direction.get_wall_position_for_cell();
    let rotation = edge.direction.get_direction_quat();
    let transform = Transform::from_translation(translation)
        .with_rotation(rotation)
        .with_scale(Vec3::splat(2.));

    let wall = commands.spawn( (
        SceneBundle {
            scene: room_assets.wall.clone(),
            transform,
            ..default()
        },
        Collider,
        position,
        Name::new(format!("Wall {:#?}", edge.edge_type))
    )).id();

    if edge.wall_furniture.contains(&String::from("wall_light"))
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

