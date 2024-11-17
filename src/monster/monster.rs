use bevy::prelude::*;

use crate::character::character::CharacterBundle;

// probably want a speed, position, velocity, maybe some way to track the player, maybe a pathfinding goal?
// Most of those are probably components?  Pathfinding goal should be a component for sure
#[derive(Bundle)]
pub struct MonsterBundle {
    character_bundle: CharacterBundle,
    scene_bundle: SceneBundle
}