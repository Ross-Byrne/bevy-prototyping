use super::{get_flex_child, get_text_bundle, spawn_ui_col, spawn_ui_row};
use crate::level_manager::{OnStationClicked, Station};
use crate::state::GameState;
use crate::util::despawn_components;
use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;

#[derive(Event, Debug)]
struct OnStationMenuExit;

#[derive(Component, Debug)]
pub struct StationMenuRoot;

#[derive(Component, Debug)]
struct ExitButton;

pub struct StationMenuPlugin;

impl Plugin for StationMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnStationMenuExit>()
            .add_systems(Update, on_click_exit)
            .add_systems(
                Update,
                spawn_station_menu.run_if(on_event::<OnStationClicked>()),
            )
            .add_systems(
                Update,
                despawn_components::<StationMenuRoot>.run_if(on_event::<OnStationMenuExit>()),
            );
    }
}

const MENU_BG_COLOUR: Color = Color::hsl(227., 0.37, 0.22);

fn spawn_station_menu(
    mut commands: Commands,
    mut event_reader: EventReader<OnStationClicked>,
    query: Query<&Station>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Don't spawn menu if already in game menu
    match state.get() {
        GameState::InGameMenu => return,
        _ => (),
    }

    let mut station_option: Option<Result<&Station, QueryEntityError>> = None;

    // Get station result from event
    for event in event_reader.read() {
        station_option = Some(query.get(event.entity));
        break;
    }

    // Get result from event reader or return
    let Some(station_result) = station_option else {
        return info!("Failed to get event from event reader");
    };

    // Get station from result or return
    let Ok(station) = station_result else {
        return info!("Failed to get entity from event");
    };

    info!("Attempting to spawn station menu for: {:?}", station);

    // Set game state to InGameMenu
    next_state.set(GameState::InGameMenu);

    // Create and spawn main UI container
    let container: Entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(50.0),
                    height: Val::Percent(70.0),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Start,
                    border: UiRect::all(Val::Px(1.0)),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                background_color: MENU_BG_COLOUR.into(),
                border_color: BorderColor(Color::BLACK),
                ..default()
            },
            StationMenuRoot,
        ))
        .id();

    // spawn exit button row
    let exit_button_row: Entity = spawn_ui_row(&mut commands, Val::Percent(100.), Val::Auto);

    // Create exit button
    let button: Entity = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(32.0),
                    height: Val::Px(32.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    // border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0., 0., 0., 0.)),
                // border_color: BorderColor(Color::BLACK),
                ..default()
            },
            ExitButton,
        ))
        .id();

    let button_text: Entity = commands.spawn(get_text_bundle("X".to_string(), 30.)).id();

    commands.entity(button).push_children(&[button_text]);

    let menu_title_text: Entity = commands
        .spawn(get_text_bundle(station.name.to_string(), 30.))
        .id();

    let offset: Entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                // border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            // border_color: BorderColor(Color::BLACK),
            ..default()
        })
        .id();

    commands
        .entity(exit_button_row)
        .push_children(&[offset, menu_title_text, button]);
    commands.entity(container).push_children(&[exit_button_row]);

    let content_container: Entity =
        spawn_ui_col(&mut commands, Val::Percent(100.), Val::Percent(100.));
    commands
        .entity(container)
        .push_children(&[content_container]);

    // Create header for listing items
    let header_row = spawn_text_row(
        &mut commands,
        24.0,
        "Name".to_string(),
        "Quantity".to_string(),
        "Value".to_string(),
    );
    commands
        .entity(content_container)
        .push_children(&[header_row]);

    // Loop through items in station
    let mut item_rows: Vec<Entity> = Vec::new();

    for item in station.inventory.items.iter() {
        let row: Entity = spawn_text_row(
            &mut commands,
            17.0,
            item.name.to_string(),
            item.quantity.to_string(),
            item.value.to_string(),
        );
        item_rows.push(row);
    }

    commands.entity(content_container).push_children(&item_rows);
}

fn spawn_text_row(
    commands: &mut Commands,
    font_size: f32,
    name: String,
    quantity: String,
    value: String,
) -> Entity {
    let basis_percent: f32 = 33.333;
    let row_entity: Entity = spawn_ui_row(commands, Val::Percent(100.), Val::Auto);

    let name_container: Entity = commands
        .spawn(get_flex_child(basis_percent, JustifyContent::Start))
        .id();
    let name_text: Entity = commands.spawn(get_text_bundle(name, font_size)).id();
    commands.entity(name_container).push_children(&[name_text]);

    let quantity_text: Entity = commands.spawn(get_text_bundle(quantity, font_size)).id();
    let quantity_container: Entity = commands
        .spawn(get_flex_child(basis_percent, JustifyContent::End))
        .id();
    commands
        .entity(quantity_container)
        .push_children(&[quantity_text]);

    let value_text: Entity = commands.spawn(get_text_bundle(value, font_size)).id();
    let value_container: Entity = commands
        .spawn(get_flex_child(basis_percent, JustifyContent::End))
        .id();
    commands
        .entity(value_container)
        .push_children(&[value_text]);

    commands.entity(row_entity).push_children(&[
        name_container,
        quantity_container,
        value_container,
    ]);

    return row_entity;
}

fn on_click_exit(
    mut event_writer: EventWriter<OnStationMenuExit>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("clicked Exit!");

                // Send exit menu event.
                event_writer.send(OnStationMenuExit);

                // set state to be InGame
                next_state.set(GameState::InGame);
            }
            _ => {}
        }
    }
}
