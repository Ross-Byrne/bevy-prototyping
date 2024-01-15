use crate::player::*;
use bevy::prelude::*;

pub mod player;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut _commands: Commands, _asset_server: Res<AssetServer>) {
    println!("Setting up main")
}
