use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut _commands: Commands, _asset_server: Res<AssetServer>) {
    println!("Init game UI");
}
