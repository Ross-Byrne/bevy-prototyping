use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Shield;

const MOVEMENT_SPEED: f32 = 180.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement);
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add player sprite
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            sprite: SpriteBundle {
                texture: asset_server.load("circle.png"),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            },
        },
        Player,
    ));

    // Add player shield and scale up sprite
    let mut shield_transform: Transform = Transform::from_xyz(0., 0., 0.);
    shield_transform.scale = Vec3::new(1.3, 1.3, 0.);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ring.png"),
            transform: shield_transform,
            ..default()
        },
        Player,
        Shield,
    ));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut sprite_position: Query<(&mut Transform, With<Player>)>,
) {
    for (mut transform, ()) in &mut sprite_position {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += MOVEMENT_SPEED * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= MOVEMENT_SPEED * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= MOVEMENT_SPEED * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += MOVEMENT_SPEED * time.delta_seconds();
        }
    }
}
