mod asset_loader;
mod camera;
mod debug;
mod enemy;
mod level_manager;
mod movement;
mod player;
mod schedule;
mod state;
mod ui;
mod util;

use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use camera::CameraPlugin;
// use debug::DebugPlugin;
use enemy::EnemyPlugin;
use level_manager::LevelManagerPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use schedule::SchedulePlugin;
use state::StatePlugin;
use ui::UIPlugin;

#[derive(Component, Debug)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(SchedulePlugin)
        // .add_plugins(DebugPlugin)
        .add_plugins(StatePlugin)
        .add_plugins(LevelManagerPlugin)
        .run();
}
