use crate::asset_loader::ImageAssets;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use crate::schedule::InGameSet;
use bevy::prelude::*;
use rand::prelude::*;
use std::ops::Range;

const VELOCITY_SCALAR: f32 = 50.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -100.0..100.0;
const SPAWN_RANGE_Y: Range<f32> = -100.0..100.0;
const SPAWN_TIME_SECONDS: f32 = 3.0;
const DESPAWN_TIME_SECONDS: f32 = 20.0;

#[derive(Component, Debug)]
pub struct Enemy {
    pub despawn_timer: Timer,
}

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(Update, spawn_enemy.in_set(InGameSet::UserInput))
        .add_systems(Update, despawn_enemy.in_set(InGameSet::DespawnEntities));
    }
}

fn spawn_enemy(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    image_assets: Res<ImageAssets>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        rng.gen_range(SPAWN_RANGE_Y),
        0.0,
    );

    let velocity = random_unit_vector(&mut rng) * VELOCITY_SCALAR;
    let acceleration = random_unit_vector(&mut rng) * ACCELERATION_SCALAR;

    commands.spawn((
        MovingObjectBundle {
            acceleration: Acceleration::new(acceleration),
            velocity: Velocity::new(velocity),
            sprite: SpriteBundle {
                texture: image_assets.enemy.clone(),
                transform: Transform::from_translation(translation),
                ..default()
            },
        },
        Enemy {
            despawn_timer: Timer::from_seconds(DESPAWN_TIME_SECONDS, TimerMode::Once),
        },
    ));
}

fn despawn_enemy(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Enemy), With<Enemy>>,
    time: Res<Time>,
) {
    for (entity, mut enemy) in query.iter_mut() {
        enemy.despawn_timer.tick(time.delta());

        if enemy.despawn_timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn random_unit_vector(rng: &mut ThreadRng) -> Vec3 {
    return Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0).normalize_or_zero();
}
