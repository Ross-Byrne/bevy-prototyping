use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct ImageAssets {
    pub player: Handle<Image>,
    pub enemy: Handle<Image>,
    pub shield: Handle<Image>,
    pub projectile: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ImageAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut image_assets: ResMut<ImageAssets>, asset_server: Res<AssetServer>) {
    *image_assets = ImageAssets {
        player: asset_server.load("triangle.png"),
        enemy: asset_server.load("diamond.png"),
        shield: asset_server.load("ring.png"),
        projectile: asset_server.load("circle.png"),
    }
}
