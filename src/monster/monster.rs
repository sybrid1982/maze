use bevy::prelude::*;

use crate::{character::character::{CharacterBundle, Speed}, consts, game_states::GameState, physics::velocity::Velocity, player::player::LogicalPlayer, position::Position};

use super::monster_assets::MonsterAssets;

// probably want a speed, position, velocity, maybe some way to track the player, maybe a pathfinding goal?
// Most of those are probably components?  Pathfinding goal should be a component for sure
#[derive(Bundle)]
pub struct MonsterBundle {
    character_bundle: CharacterBundle,
    scene_bundle: SceneBundle
}

#[derive(Component)]
pub struct NavigateToPlayer;


pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initialize), spawn_demon)
            .add_systems(Update, move_monster_towards_player);
    }
}

// lets spawn the monster in the center for now
fn spawn_demon(mut commands: Commands, assets: Res<MonsterAssets>) {
    let demon_position = Position::new((consts::MAZE_X / 2) as f32, (consts::MAZE_Y / 2) as f32);
    commands.spawn((
        MonsterBundle {
            character_bundle: CharacterBundle {
                velocity: Velocity::new(0., 0.),
                position: demon_position,
                speed: Speed(5.0)
            },
            scene_bundle: SceneBundle {
                scene: assets.demon_model.clone(),
                transform: Transform::from_translation(demon_position.to_vec3_by_scale(consts::MAZE_SCALE)).with_scale(Vec3::splat(2.2)),
                ..default()
            },
        },
        Name::new(String::from("Demon")),
        NavigateToPlayer
        ),
    );
}
//Query<&Transform, (With<LogicalPlayer>, Without<TopDownCamera>)>
fn move_monster_towards_player(time: Res<Time>, mut monsters: Query<(&mut Transform, &mut Velocity), With<NavigateToPlayer>>, player: Query<&Transform, (With<LogicalPlayer>, Without<NavigateToPlayer>)>) {
    let player_transform = player.single();
    for (mut monster_transform, mut monster_velocity) in monsters.iter_mut() {
        let monster_forward = (monster_transform.rotation * Vec3::Z).xz();

        let move_direction = (player_transform.translation.xz() - monster_transform.translation.xz()).normalize();
        monster_velocity.set_velocity(Vec2::new(move_direction.x, move_direction.y) * 2.0);

        let forward_dot_player = monster_forward.dot(move_direction);

        if (forward_dot_player - 1.0).abs() < f32::EPSILON {
            continue;
        }

        let monster_right = (monster_transform.rotation * Vec3::X).xz();

        let right_dot_player = monster_right.dot(move_direction);

        let rotation_sign = f32::copysign(1.0, right_dot_player);

        let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos();

        let monster_angle = rotation_sign * (2.0 * time.delta_seconds()).min(max_angle);

        monster_transform.rotate_y(monster_angle);
    }
}