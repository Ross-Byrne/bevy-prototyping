use crate::asset_loader::ImageAssets;
use crate::item_manager::*;
use crate::state::GameState;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Component, Debug)]
pub struct Station {
    pub name: String,
    pub inventory: Inventory,
}

impl Station {
    pub fn new(name: String, inventory: Inventory) -> Self {
        return Self { name, inventory };
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

    // Spawn station number 1
    let inventory: Inventory = Inventory::new(vec![
        Item::new(ItemType::EnergyCell, 1, 100),
        Item::new(ItemType::Silica, 2, 30),
    ]);
    let station: Station = Station::new(String::from("Trade Station 1"), inventory);

    commands.spawn((
        SpriteBundle {
            texture: image_assets.projectile.clone(),
            transform: Transform::from_xyz(200.0, 200.0, 1.0).with_scale(Vec3::splat(0.6)),
            ..default()
        },
        station,
        Clickable,
        On::<Pointer<Click>>::send_event::<OnStationClicked>(),
    ));

    // Spawn station number 2
    let inventory: Inventory = Inventory::new(vec![
        Item::new(ItemType::EnergyCell, 1, 200),
        Item::new(ItemType::IronOre, 2, 20),
    ]);
    let station: Station = Station::new(String::from("Trade Station 2"), inventory);

    commands.spawn((
        SpriteBundle {
            texture: image_assets.projectile.clone(),
            transform: Transform::from_xyz(100.0, 400.0, 1.0).with_scale(Vec3::splat(0.6)),
            ..default()
        },
        station,
        Clickable,
        On::<Pointer<Click>>::send_event::<OnStationClicked>(),
    ));
}

fn on_station_clicked(mut event_reader: EventReader<OnStationClicked>, query: Query<&Station>) {
    for event in event_reader.read() {
        let Ok(station) = query.get(event.entity) else {
            continue;
        };

        info!("Clicked: {:?}", station);
    }
}
