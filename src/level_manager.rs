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

#[derive(Event, Debug)]
pub struct OnStationClicked {
    pub entity: Entity,
}

impl From<ListenerInput<Pointer<Click>>> for OnStationClicked {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        OnStationClicked {
            entity: event.target,
        }
    }
}

#[derive(Component, Debug)]
pub struct Clickable;

pub struct LevelManagerPlugin;

impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnStationClicked>()
            .add_systems(OnEnter(GameState::InGame), setup)
            .add_systems(
                Update,
                on_station_clicked.run_if(on_event::<OnStationClicked>()),
            );
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
        Station::new("Tade Station 1".to_owned()),
        Clickable,
        On::<Pointer<Click>>::send_event::<OnStationClicked>(),
    ));

    commands.spawn((
        SpriteBundle {
            texture: image_assets.projectile.clone(),
            transform: Transform::from_xyz(100.0, 400.0, 1.0).with_scale(Vec3::splat(0.6)),
            ..default()
        },
        Station::new("Tade Station 2".to_owned()),
        Clickable,
        On::<Pointer<Click>>::send_event::<OnStationClicked>(),
    ));
}

/// Unlike callback systems, this is a normal system that can be run in parallel with other systems.
fn on_station_clicked(mut event_reader: EventReader<OnStationClicked>, query: Query<&Station>) {
    for event in event_reader.read() {
        let Ok(station) = query.get(event.entity) else {
            continue;
        };

        info!("Clicked: {:?}", station);
    }
}
