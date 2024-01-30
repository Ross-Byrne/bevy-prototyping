use crate::asset_loader::ImageAssets;
use crate::movement::{Acceleration, MovingObjectBundle, Rotation, Velocity};
use crate::schedule::InGameSet;
use crate::state::GameState;
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

const MOVEMENT_SPEED: f32 = 280.0;
const PROJECTILE_SPEED: f32 = 500.0;
const PROJECTILE_FORWARD_SPAWN_SCALAR: f32 = 20.0;
const PROJECTILE_DESPAWN_TIME_SECONDS: f32 = 2.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LoadingGame), spawn_player)
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
    info!("Spawning Player");

    // Add player sprite
    let player_transform: Transform =
        Transform::from_xyz(0., 0., 1.).with_scale(Vec3::new(0.2, 0.2, 1.));

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            sprite: SpriteBundle {
                texture: image_assets.player.clone(),
                transform: player_transform,
                ..default()
            },
        },
        Player,
        Rotation::new(),
    ));

    // // Add player shield and scale up sprite
    // let shield_transform: Transform =
    //     Transform::from_translation(player_transform.translation).with_scale(Vec3::splat(0.4));

    // commands.spawn((
    //     MovingObjectBundle {
    //         velocity: Velocity::new(Vec3::ZERO),
    //         acceleration: Acceleration::new(Vec3::ZERO),
    //         sprite: SpriteBundle {
    //             texture: image_assets.shield.clone(),
    //             transform: shield_transform,
    //             ..default()
    //         },
    //     },
    //     Player,
    //     Shield,
    //     Rotation::new(),
    // ));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Transform, &mut Velocity, &mut Rotation), With<Player>>,
) {
    for (transform, mut velocity, mut rotation) in query.iter_mut() {
        let mut default_rotation_factor = 0.0;
        let mut movement_factor = 0.0;

        if keyboard_input.pressed(KeyCode::A) {
            default_rotation_factor += 1.0;
        }

        if keyboard_input.pressed(KeyCode::D) {
            default_rotation_factor -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::W) {
            movement_factor += 1.0;
        }

        rotation.factor = default_rotation_factor;

        // get the ship's forward vector by applying the current rotation to the ships initial facing
        // vector
        let movement_direction = transform.rotation * Vec3::Y;
        // get the distance the ship will move based on direction, the ship's movement speed and delta
        // time
        let movement_distance = movement_factor * MOVEMENT_SPEED;
        // create the change in translation using the new movement direction and distance
        let translation_delta = movement_direction * movement_distance;
        // update the ship translation with our new translation delta
        velocity.value = translation_delta;
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
        projectile_transform.scale = Vec3::new(0.03, 0.03, 0.);

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
