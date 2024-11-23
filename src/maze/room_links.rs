use std::ops::Index;

use crate::position::Position;

struct RoomPosition {
    room: usize,
    position: Position
}

pub struct RoomLink {
    room_position1: RoomPosition,
    room_position2: RoomPosition,
    is_open: bool
}

impl RoomLink {
    fn has_room_number(&self, room_number: usize) -> bool {
        self.room_position1.room == room_number || self.room_position2.room == room_number
    }

    fn is_open(&self) -> bool {
        self.is_open
    }

    fn open_link(&mut self) {
        self.is_open = true;
    }
}

pub struct RoomLinks {
    pub room_links: Vec<RoomLink>
}

impl RoomLinks {
    pub fn get_room_links(&self, room_number: usize) -> Vec<&RoomLink> {
        let matching_links = self.room_links.iter().filter(|room_link| room_link.has_room_number(room_number));
        matching_links.collect()
    }

    pub fn find_room_path(&self, starting_room: usize, ending_room: usize) -> Vec<usize> {
        // idea -> inside a room, we can do a pathfind across the room to find something, but to go from room
        // to room, we can do the pathfinding as a series of room links until the nav agent is in the room with the nav target
        // so we need to see if we can find a path from starting_room to ending_room
        // and the the output will be the room numbers for each room, and we can go pairwise through to find the path as the agent moves from room to room
        let mut open = vec![starting_room];
        let mut closed: Vec<(usize, usize)> = vec![];
        let mut path: Vec<usize> = vec![];
        let mut path_found = false;

        while open.len() > 0 && path_found == false {
            
        }

        // if path ends up empty, then we know there was no path between the rooms
        path
    }
}

fn get_estimated_distance(start: RoomLink, goal: Position) -> usize {
    // for the room link, we have two positions
    goal.get_distance_to_position(start.room_position1.position)
}
