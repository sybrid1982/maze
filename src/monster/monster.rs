use bevy::prelude::*;

use crate::{character::character::{CharacterBundle, Speed}, consts, game_states::GameState, maze::maze::Maze, physics::velocity::Velocity, player::player::LogicalPlayer, position::Position};

use super::{monster_assets::MonsterAssets, monster_events::MonsterReachedPlayer};

// probably want a speed, position, velocity, maybe some way to track the player, maybe a pathfinding goal?
// Most of those are probably components?  Pathfinding goal should be a component for sure
#[derive(Bundle)]
pub struct MonsterBundle {
    character_bundle: CharacterBundle,
    scene_bundle: SceneBundle
}

#[derive(Component)]
pub struct NavigateToPlayer;

#[derive(Component)]
pub struct PathfindingGoal {
    pub goal: Option<Vec2>
}


pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initialize), spawn_demon)
            .add_systems(Update, (pathfind_towards_player, move_agents_towards_goals).chain().run_if(in_state(GameState::InGame)))
            .add_event::<MonsterReachedPlayer>();
    }
}

// lets spawn the monster in the center for now
fn spawn_demon(mut commands: Commands, assets: Res<MonsterAssets>) {
    let demon_position = Position::new((consts::MAZE_X / 2) as usize, (consts::MAZE_Y / 2) as usize);
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
        NavigateToPlayer,
        PathfindingGoal { goal: None }
        ),
    );
}

//Query<&Transform, (With<LogicalPlayer>, Without<TopDownCamera>)>
fn pathfind_towards_player(
    maze: Res<Maze>,
    mut monsters: Query<(&Transform, &mut PathfindingGoal), With<NavigateToPlayer>>,
    player: Query<(&Transform, &Position), (With<LogicalPlayer>, Without<NavigateToPlayer>)>,
    mut writer: EventWriter<MonsterReachedPlayer>
) {
    let (player_transform, player_position) = player.single();
    let player_room = maze.get_room_number_for_position(player_position.clone());
    for (monster_transform, mut goal) in monsters.iter_mut() {
        let monster_position = Position::get_from_transform(&monster_transform, consts::MAZE_SCALE);
        let monster_room = maze.get_room_number_for_position(monster_position);

        if monster_room == player_room {
            if monster_transform.translation.xz().distance(player_transform.translation.xz()) < 1.0 {
                goal.goal = None;
                writer.send(MonsterReachedPlayer);
            } else if goal.goal.is_some_and(|goal_pos| goal_pos == player_transform.translation.xz()) {
                // goal hasn't changed
            } else {
                // set the pathfinding goal I guess to the player's position?
                goal.goal = Some(player_transform.translation.xz());
                println!("Changed goal to new position");
            }
        } else {
            // work out how to get the monster into the player's room and set that as a goal
            // if the player changes rooms we will need to update the goal.
        }
    }
}

fn move_agents_towards_goals(time: Res<Time>, mut agents: Query<(&mut Transform, &mut Velocity, &PathfindingGoal)>) {
    for (mut agent_transform, mut agent_velocity, goal) in agents.iter_mut() {
        match goal.goal {
            Some(goal) => {
                let move_direction = (goal - agent_transform.translation.xz()).normalize();

                agent_velocity.set_velocity(Vec2::new(move_direction.x, move_direction.y) * 2.0);
        
                rotate_towards_direction(agent_transform, move_direction, &time)        
            },
            None => {
                agent_velocity.zero_velocity();
            },
        }
    }
}

fn rotate_towards_direction(mut monster_transform: Mut<'_, Transform>, move_direction: Vec2, time: &Res<'_, Time>) {
    let monster_forward = (monster_transform.rotation * Vec3::Z).xz();
    let forward_dot_player = monster_forward.dot(move_direction);
    if (forward_dot_player - 1.0).abs() < f32::EPSILON {
        return;
    }
    let monster_right = (monster_transform.rotation * Vec3::X).xz();
    let right_dot_player = monster_right.dot(move_direction);
    let rotation_sign = f32::copysign(1.0, right_dot_player);
    let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos();
    let monster_angle = rotation_sign * (2.0 * time.delta_seconds()).min(max_angle);
    monster_transform.rotate_y(monster_angle);
}