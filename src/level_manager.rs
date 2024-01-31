use crate::asset_loader::ImageAssets;
use crate::state::GameState;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

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
        app.add_systems(OnEnter(GameState::InGame), setup);
    }
}

fn setup(mut commands: Commands, image_assets: Res<ImageAssets>) {
    info!("level manager is setting up level");

    // Start spawning game object

    commands.spawn((
        SpriteBundle {
            texture: image_assets.projectile.clone(),
            transform: Transform::from_xyz(200.0, 200.0, 1.0).with_scale(Vec3::splat(0.6)),
            ..default()
        },
        Station::new("Tade Station".to_owned()),
        Clickable,
        On::<Pointer<Click>>::run(|event: Listener<Pointer<Click>>, query: Query<&Station>| {
            for station in query.iter() {
                info!("{:?}", station)
            }

            info!("The pointer clicked entity {:?}", event.target);
        }),
    ));
}
