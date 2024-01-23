use crate::player::{Player, Shield};
use crate::schedule::InGameSet;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            // Run after entities update to fix player jutter
            // to stop player position being updated after camera moves
            .add_systems(Update, sync_player_camera.after(InGameSet::EntityUpdates));
    }
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

pub fn sync_player_camera(
    player: Query<&Transform, (With<Player>, Without<Shield>)>,
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };

    let Ok(mut camera_transform) = camera.get_single_mut() else {
        return;
    };

    let delta: Vec3 = player.translation - camera_transform.translation;

    if delta != Vec3::ZERO {
        camera_transform.translation += delta;
    }
}
