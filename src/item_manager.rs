use crate::state::GameState;
use bevy::prelude::*;

#[derive(Debug)]
pub enum ItemType {
    EnergyCell,
    IronOre,
    Silica,
}

#[derive(Debug)]
pub struct Item {
    pub item_type: ItemType,
    pub name: String,
    pub value: usize,
    pub quantity: usize,
}

#[derive(Debug)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub size: usize,
}

impl Item {
    pub fn new(item_type: ItemType, value: usize, quantity: usize) -> Self {
        let name: String = match item_type {
            ItemType::EnergyCell => String::from("Energy Cell"),
            ItemType::IronOre => String::from("Iron Ore"),
            ItemType::Silica => String::from("Silica"),
        };

        return Self {
            item_type,
            name,
            value,
            quantity,
        };
    }
}

impl Inventory {
    pub fn new(items: Vec<Item>) -> Self {
        let size: usize = items.len();

        return Self { items, size };
    }
}

pub struct ItemManagerPlugin;

impl Plugin for ItemManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LoadingGame), setup);
    }
}

fn setup() {
    info!("Setting up items")
}
