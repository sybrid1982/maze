use bevy::prelude::*;

use super::{maze_assets::MazeAssets, maze_direction::MazeDirection};
use crate::{collider::Collider, position::Position};
/**
 * Because a door is more complicated, I want to make sure I have the logic for it all in one place.
 * A door consists of two entities, the frame which is the parent, and the child that is the door itself.
 * The anchor for the door needs to be on an edge of the door, so that we can open/shut the door by rotating the door's transform
 * 
 * Public methods needed:
 * new(position: Position) -> Spawns a door at a position with the door closed, returns a new MazeDoor
 * get_door_render(??) -> gives instructions to build the 3D objects/entities that make up a maze door, maybe returns an Entity or a Bundle? 
 * open_door(swing_forward: boolean) -> opens the door.  If swing_forward is true, then we rotate the y axis 90 positive, if false then 90 negative
 */

const MAZE_DOOR_VERTICAL_FRAME: f32 = 0.5;
const MAZE_DOOR_BOTTOM_FRAME: f32 = 0.1;
const MAZE_DOOR_TOP_FRAME: f32 = 1.0;

#[derive(Component)]
pub struct MazeDoor {
    door_child: Entity,
    position: Position,
    maze_direction: MazeDirection
}

impl MazeDoor {
    pub fn new(        
        commands: &mut Commands<'_, '_>, 
        wall_assets: &Res<MazeAssets>,
        position: Position,
        maze_direction: MazeDirection
    ) -> Self 
    {
        // create the door frame entity
        let door = MazeDoor::get_door_render(commands, wall_assets);

        MazeDoor { 
            door_child: door,
            position,
            maze_direction
        }
    }

    pub fn get_door_child(&self) -> Entity {
        self.door_child
    }

    pub fn get_door_render(
        commands: &mut Commands<'_, '_>,
        wall_assets: &Res<MazeAssets>
    ) -> Entity {
        let door = commands.spawn( (
            SceneBundle {
                scene: wall_assets.door.clone(),
                transform: Transform::from_xyz(0.75,0.,0.0)
                    .with_scale(Vec3::new(1.0, 1.0, 1.0)),
                ..default()
            },
            Collider,
        )).id();

        door
}

    pub fn open_door(swing_forward: bool) {

    }
}