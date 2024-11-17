use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::maze_direction::MazeDirection;
use crate::physics::collider::Collider;
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

#[derive(PartialEq)]
enum DoorState {
    Closed,
    OpeningForward,
    OpeningBackward,
    Open
}

#[derive(Component, Deref, DerefMut)]
pub struct MazeDoor {
    #[deref]
    door_child: Entity,
    maze_direction: MazeDirection,
    state: DoorState,
}

#[derive(Component)]
pub struct DoorChild;

impl MazeDoor {
    pub fn new(        
        commands: &mut Commands<'_, '_>, 
        door_handle: Handle<Scene>,
        maze_direction: MazeDirection,
    ) -> Self 
    {
        // create the door frame entity
        let door = MazeDoor::get_door_render(commands, door_handle);

        MazeDoor { 
            door_child: door,
            maze_direction,
            state: DoorState::Closed
        }
    }

    pub fn get_door_child(&self) -> Entity {
        self.door_child
    }

    pub fn get_door_render(
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

    pub fn open_door(&mut self, swing_forward: bool) {
        if self.state == DoorState::Open { return };
        if swing_forward {
            self.state = DoorState::OpeningForward
        } else {
            self.state = DoorState::OpeningBackward
        }
    }

    pub fn get_maze_direction(&self) -> &MazeDirection {
        &self.maze_direction
    }
}

pub fn door_open_system(time: Res<Time>, mut door_query: Query<(&mut MazeDoor, &mut Transform)>) {
    let door_open_speed: f32 = 3.5;
    let door_open_max: f32 = 7.5;
    for (mut door, mut door_transform) in door_query.iter_mut() {
        if door.state == DoorState::OpeningForward {
            let rotation_angle = (door_open_speed * time.delta_seconds()).min(door_open_max);
            door_transform.rotate_y(rotation_angle);
            if door_transform.rotation.to_euler(EulerRot::XYZ).0 <= -1.0 {
                door.state = DoorState::Open
            }
        } else if door.state == DoorState::OpeningBackward {
            let rotation_angle = -1.0 * (door_open_speed * time.delta_seconds()).max(-door_open_max);
            door_transform.rotate_y(rotation_angle);
            if door_transform.rotation.to_euler(EulerRot::XYZ).0 >= 1.0 {
                door.state = DoorState::Open
            }
        }
    }
}