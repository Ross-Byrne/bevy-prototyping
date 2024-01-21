use crate::asset_loader::ImageAssets;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use crate::schedule::InGameSet;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Shield;

#[derive(Component, Debug)]
pub struct Projectile {
    pub despawn_timer: Timer,
}

const MOVEMENT_SPEED: f32 = 180.0;
const PROJECTILE_SPEED: f32 = 300.0;
const PROJECTILE_FORWARD_SPAWN_SCALAR: f32 = 30.0;
const PROJECTILE_DESPAWN_TIME_SECONDS: f32 = 2.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(
                Update,
                (player_movement, player_weapon_controls)
                    .chain()
                    .in_set(InGameSet::UserInput),
            )
            .add_systems(
                Update,
                despawn_projectile.in_set(InGameSet::DespawnEntities),
            );
    }
}

fn spawn_player(mut commands: Commands, image_assets: Res<ImageAssets>) {
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
                transform: shield_transform,
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

fn player_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, (With<Player>, Without<Shield>)>,
    keyboard_input: Res<Input<KeyCode>>,
    image_assets: Res<ImageAssets>,
) {
    let Ok(transform) = query.get_single() else {
        return info!("Error trying to get Player Transform");
    };

    if keyboard_input.pressed(KeyCode::Space) {
        // calculate where to spawn the projectile (in front of player)
        let transform_vec: Vec3 =
            transform.translation + transform.up() * PROJECTILE_FORWARD_SPAWN_SCALAR;
        let mut projectile_transform: Transform = Transform::from_translation(transform_vec);
        projectile_transform.scale = Vec3::new(0.4, 0.4, 0.);

        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(transform.up() * PROJECTILE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                sprite: SpriteBundle {
                    texture: image_assets.projectile.clone(),
                    transform: projectile_transform,
                    ..default()
                },
            },
            Projectile {
                despawn_timer: Timer::from_seconds(
                    PROJECTILE_DESPAWN_TIME_SECONDS,
                    TimerMode::Once,
                ),
            },
        ));
    }
}

fn despawn_projectile(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Projectile), With<Projectile>>,
    time: Res<Time>,
) {
    for (entity, mut projectile) in query.iter_mut() {
        projectile.despawn_timer.tick(time.delta());

        if projectile.despawn_timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
