use crate::asset_loader::ImageAssets;
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

pub fn setup(mut commands: Commands, image_assets: Res<ImageAssets>) {
    // Add player sprite
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            sprite: SpriteBundle {
                texture: image_assets.player.clone(),
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            },
        },
        Player,
    ));

    // Add player shield and scale up sprite
    let mut shield_transform: Transform = Transform::from_xyz(0., 0., 1.);
    shield_transform.scale = Vec3::new(1.3, 1.3, 0.);

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            sprite: SpriteBundle {
                texture: image_assets.shield.clone(),
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            },
        },
        Player,
        Shield,
    ));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in &mut query {
        let mut movement: Vec3 = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            movement.y = MOVEMENT_SPEED;
        }

        if keyboard_input.pressed(KeyCode::S) {
            movement.y = -MOVEMENT_SPEED;
        }

        if keyboard_input.pressed(KeyCode::A) {
            movement.x = -MOVEMENT_SPEED;
        }

        if keyboard_input.pressed(KeyCode::D) {
            movement.x = MOVEMENT_SPEED;
        }

        velocity.value = movement;
    }
}
