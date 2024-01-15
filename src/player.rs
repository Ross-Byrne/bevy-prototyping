use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Shield;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, player_movement);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Add player sprite
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("circle.png"),
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
            transform.translation.y += 180. * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= 180. * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 180. * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 180. * time.delta_seconds();
        }
    }
}
