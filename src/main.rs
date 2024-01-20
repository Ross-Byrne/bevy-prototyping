pub mod asset_loader;
pub mod enemy;
pub mod movement;
pub mod player;
pub mod ui;

use crate::asset_loader::AssetLoaderPlugin;
use crate::enemy::EnemyPlugin;
use crate::movement::MovementPlugin;
use crate::player::PlayerPlugin;
// use crate::ui::UIPlugin;
use bevy::prelude::*;

#[derive(Component, Debug)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoaderPlugin)
        // .add_plugins(UIPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(EnemyPlugin)
        .add_systems(Startup, setup)
        .add_systems(PostStartup, player::setup)
        .run();
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    println!("Setting up main");
    commands.spawn(Camera2dBundle::default());
}
