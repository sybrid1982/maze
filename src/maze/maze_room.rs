use std::collections::HashMap;

use bevy::prelude::*;
use rand::Rng;

use crate::random::Random;

use super::maze_assets::MazeAssets;

#[derive(Clone)]
pub struct MazeRoomSettings {
    room_assets: RoomAssets,
    floor: Handle<StandardMaterial>,
    name: String
}

#[derive(Clone)]
pub struct RoomAssets {
    pub wall: Handle<Scene>,
    pub doorway: Handle<Scene>,
    pub other_furniture: HashMap<String, Handle<Scene>>
}

pub struct MazeRoom {
    settings: MazeRoomSettings,
    settings_index: usize,
}

impl MazeRoom {
    fn new(settings: &MazeRoomSettings, settings_index: usize) -> Self {
        MazeRoom {
            settings: settings.clone(),
            settings_index,
        }
    }
}

#[derive(Resource)]
pub struct MazeRooms {
    all_settings: Vec<MazeRoomSettings>,
    maze_rooms: Vec<MazeRoom>
}

impl Default for MazeRooms {
    fn default() -> Self {
        MazeRooms {
            all_settings: vec![],
            maze_rooms: vec![]
        }
    }
}

impl MazeRooms {
    pub fn new(assets: Res<MazeAssets>, materials: &mut ResMut<'_, Assets<StandardMaterial>>) -> Self {
        let basic_carpet = generate_material_from_image(materials, assets.carpet_1.clone());
        let second_carpet = generate_material_from_image(materials, assets.carpet_2.clone());
        let bathroom_tile = generate_material_from_image(materials, assets.bathroom_tile.clone());
        let kitchen_tile = generate_material_from_image(materials, assets.kitchen_tile.clone());

        let default_room_assets = RoomAssets { wall: assets.basic_wall.clone(), doorway: assets.doorway.clone(), other_furniture: HashMap::new(), };

        MazeRooms {
            all_settings: vec![
                MazeRoomSettings { room_assets: default_room_assets.clone(), floor: basic_carpet, name: String::from("Basic Room") },
                MazeRoomSettings { room_assets: default_room_assets.clone(), floor: second_carpet, name: String::from("Second Basic Room") },
                MazeRoomSettings { room_assets: default_room_assets.clone(), floor: bathroom_tile, name: String::from("Bathroom") },
                MazeRoomSettings { room_assets: default_room_assets.clone(), floor: kitchen_tile, name: String::from("Kitchen") },
            ],
            maze_rooms: vec![]
        }
    }

    pub fn create_room_and_return_index(&mut self, index_to_exclude: usize, rng: &mut ResMut<Random>) -> usize {
        let settings_size = self.all_settings.len();
        if settings_size == 0 {
            panic!("cannot create rooms without room settings");
        }

        let mut new_setting_index = rng.gen_range(0..settings_size - 1);
        if new_setting_index == index_to_exclude {
            new_setting_index = (new_setting_index + 1) % settings_size
        }

        let new_room = MazeRoom::new(&self.all_settings[new_setting_index], new_setting_index);
        self.maze_rooms.push(new_room);

        return self.maze_rooms.len() - 1;
    }

    pub fn get_material_for_floor_by_room_index(&self, room_index: usize) -> Handle<StandardMaterial> {
        let maze_room = &self.maze_rooms[room_index];
        maze_room.settings.floor.clone()
    }

    pub fn get_settings_index_from_room_index(&self, room_index: usize) -> usize {
        self.maze_rooms[room_index].settings_index
    }

    pub fn get_assets_for_room_index(&self, room_index: usize) -> RoomAssets {
        self.maze_rooms[room_index].settings.room_assets.clone()
    }
}

fn generate_material_from_image(materials: &mut ResMut<'_, Assets<StandardMaterial>>, image: Handle<Image>) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color_texture: Some(image),
        alpha_mode: AlphaMode::Blend,
        unlit: false,
        ..default()
    })
}

// or should maze cells be a position?
// is it better for the maze to hold all the cells, and the rooms to then have the positions for the cells?
// or should the rooms have the cells, and the maze then has to query the rooms, and the rooms then would have the cells