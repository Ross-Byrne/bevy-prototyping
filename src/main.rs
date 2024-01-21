mod asset_loader;
mod camera;
mod enemy;
mod movement;
mod player;
mod schedule;
mod ui;
mod debug;

use asset_loader::AssetLoaderPlugin;
use enemy::EnemyPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
// use ui::UIPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use schedule::SchedulePlugin;
use debug::DebugPlugin;

#[derive(Component, Debug)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        // .add_plugins(UIPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(DebugPlugin)
        .run();
}
