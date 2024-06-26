use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub droid: Handle<Scene>,
    pub missile: Handle<Scene>,
    pub healthbar: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        asteroid: asset_server.load("Ultimate Space Kit-glb/Rock.glb#Scene0"),
        spaceship: asset_server.load("Ultimate Space Kit-glb/Spaceship.glb#Scene0"),
        droid: asset_server.load("Ultimate Space Kit-glb/Spaceship-Jqfed124pQ.glb#Scene0"),
        missile: asset_server.load("Ultimate Space Kit-glb/Bullets Pickup.glb#Scene0"),
        healthbar: asset_server.load("healthbar.png"),
    }
}