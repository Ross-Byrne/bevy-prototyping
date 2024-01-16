use crate::player::*;
use crate::ui::*;
use bevy::prelude::*;

pub mod player;
pub mod ui;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UIPlugin, PlayerPlugin))
        .add_systems(Startup, setup)
        .add_systems(Startup, player::setup)
        .run();
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    println!("Setting up main");
    commands.spawn(Camera2dBundle::default());
}
