use bevy::prelude::*;

#[derive(Resource)]
pub struct MazeAssets {
    pub basic_wall: Handle<Scene>,
    pub doorway: Handle<Scene>,
    pub door: Handle<Scene>,
    pub carpet_1: Handle<Image>,
    pub carpet_2: Handle<Image>,
    pub bathroom_tile: Handle<Image>,
    pub kitchen_tile: Handle<Image>,
    pub wall_light: Handle<Scene>,
    pub wall_light_2: Handle<Scene>,
    pub ceiling: Handle<Scene>
}

impl MazeAssets {
    pub fn load_assets(
        mut commands: Commands,
        server: Res<AssetServer>,
    ) {
        commands.insert_resource(MazeAssets {
            basic_wall: server.load("walls/basic-wall.glb#Scene0"),
            doorway: server.load("walls/doorway.glb#Scene0"),
            door: server.load("walls/door.glb#Scene0"),
            carpet_1: server.load("Carpet_04.png"),
            carpet_2: server.load("Carpet_05.png"),
            bathroom_tile: server.load("Tile_Bathroom_01.png"),
            kitchen_tile: server.load("Tile_Kitchen_01.png"),
            wall_light: server.load("wall_light.glb#Scene0"),
            wall_light_2: server.load("wall_light_2.glb#Scene0"),
            ceiling: server.load("ceiling.glb#Scene0")
        });
    }
}

