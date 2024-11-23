use bevy::prelude::*;

pub struct Assets;

impl Assets {
    pub fn load_glb_asset(file_path: String, asset_server: Res<AssetServer>, scene_number: Option<usize>) -> Handle<Scene>
    {
        let scene_number_unwrapped = scene_number.unwrap_or(0);
        let full_path = format!("{}#Scene{}", file_path, scene_number_unwrapped);
        asset_server.load(full_path)
    }

    pub fn load_png(file_path: String, asset_server: AssetServer) -> Handle<Image>
    {
        asset_server.load(format!("{}", file_path))
    }

}