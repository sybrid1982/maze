use bevy::prelude::*;
use crate::assets::Assets;

#[derive(Resource)]
pub struct MonsterAssets {
    pub demon_model: Handle<Scene>
}

impl MonsterAssets {
    pub fn load_assets(
        mut commands: Commands,
        server: Res<AssetServer>,
    ) {
        commands.insert_resource(MonsterAssets {
            demon_model: Assets::load_glb_asset(String::from("demon.glb"), server, Some(0))
        });
    }
}
