use bevy::prelude::*;

#[derive(Resource)]
pub struct MazeAssets {
    pub basic_wall: Handle<Scene>,
    pub doorway: Handle<Scene>,
    pub door: Handle<Scene>,
    pub floor_texture: Handle<Image>
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
            floor_texture: server.load("Carpet_04.png")
        });
    }
}

