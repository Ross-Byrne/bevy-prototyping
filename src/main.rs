use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Shield;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Add player sprite
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Player,
    ));

    // Add player shield and scale up sprite
    let mut shield_transform: Transform = Transform::from_xyz(100., 0., 0.);
    shield_transform.scale = Vec3::new(1.3, 1.3, 0.);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("shield.png"),
            transform: shield_transform,
            ..default()
        },
        Player,
        Shield,
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut sprite_position: Query<(&mut Transform, With<Player>)>,
) {
    for (mut transform, ()) in &mut sprite_position {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += 150. * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= 150. * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 150. * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 150. * time.delta_seconds();
        }
    }
}
