use crate::asset_loader::ImageAssets;
use crate::state::GameState;
use bevy::prelude::*;
// use bevy_mod_picking::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component, Debug)]
pub struct Station {
    pub name: String,
}

impl Station {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Component, Debug)]
pub struct Clickable;

pub struct LevelManagerPlugin;

impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup)
            .add_systems(Update, detect_station_clicks);
    }
}

fn setup(mut commands: Commands, image_assets: Res<ImageAssets>) {
    info!("level manager is setting up level");

    // Start spawning game object

    commands.spawn((
        SpriteBundle {
            texture: image_assets.projectile.clone(),
            transform: Transform::from_xyz(200.0, 200.0, 0.0).with_scale(Vec3::new(0.6, 0.6, 0.0)),
            ..default()
        },
        Station::new("Tade Station".to_owned()),
        Clickable,
    ));
}

fn detect_station_clicks(
    mut _commands: Commands,
    query: Query<&Transform, With<Clickable>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    info!("left mouse just pressed");
    info!("{:?}", mouse_button_input);

    // Games typically only have one window (the primary window)
    let Ok(window) = q_windows.get_single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    info!("Cursor is at {:?}", cursor_position);

    for transform in query.iter() {
        info!("{:?}", transform)
    }
}
