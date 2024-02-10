use crate::level_manager::{OnStationClicked, Station};

use super::{get_button_bundle, get_text_bundle};
use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct StationMenuRoot;

#[derive(Component, Debug)]
struct ExitButton;

pub struct StationMenuPlugin;

impl Plugin for StationMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_station_menu.run_if(on_event::<OnStationClicked>()),
        );
    }
}

const MENU_BG_COLOUR: Color = Color::hsl(227., 0.37, 0.22);

fn spawn_station_menu(
    mut commands: Commands,
    mut event_reader: EventReader<OnStationClicked>,
    query: Query<&Station>,
) {
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

    let exit_button_row: Entity = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                height: Val::Auto,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            ..default()
        })
        .id();

    // Create exit button
    let button: Entity = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(32.0),
                    height: Val::Px(32.0),
                    justify_self: JustifySelf::End,
                    align_self: AlignSelf::End,
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0., 0., 0., 0.)),
                border_color: BorderColor(Color::BLACK),
                ..default()
            },
            ExitButton,
        ))
        .id();

    let button_text: Entity = commands
        .spawn(TextBundle::from_section(
            "X",
            TextStyle {
                font_size: 30.0,
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
        ))
        .id();

    commands.entity(button).push_children(&[button_text]);
    commands.entity(exit_button_row).push_children(&[button]);
    commands.entity(container).push_children(&[exit_button_row]);

    let content_container: Entity = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                border: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(6.0)),
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            ..default()
        })
        .id();

    commands
        .entity(container)
        .push_children(&[content_container]);

    // // Create and spawn Settings Button
    // let button = commands.spawn((get_button_bundle(), SettingsButton)).id();
    // let button_text = commands.spawn(get_text_bundle("Settings")).id();
    // commands.entity(button).push_children(&[button_text]);
    // commands.entity(container).push_children(&[button]);

    // // Create and spawn Quit Button
    // let button = commands.spawn((get_button_bundle(), ExitButton)).id();
    // let button_text = commands.spawn(get_text_bundle("Quit")).id();
    // commands.entity(button).push_children(&[button_text]);
    // commands.entity(container).push_children(&[button]);
}
