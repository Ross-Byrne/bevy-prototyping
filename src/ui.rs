mod station_menu;

use crate::state::{GameState, OnGameStart};
use crate::ui::station_menu::StationMenuPlugin;
use crate::util::despawn_components;
use bevy::{app::AppExit, prelude::*};
pub struct UIPlugin;

#[derive(Component, Debug)]
pub struct StartUIRoot;

#[derive(Component, Debug)]
pub struct StartButton;

#[derive(Component, Debug)]
pub struct SettingsButton;

#[derive(Component, Debug)]
pub struct ExitButton;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StationMenuPlugin)
            .add_systems(OnEnter(GameState::StartMenu), spawn_start_menu)
            .add_systems(
                Update,
                (button_system, on_click_start, on_click_exit)
                    .chain()
                    .run_if(in_state(GameState::StartMenu)),
            )
            .add_systems(
                OnExit(GameState::StartMenu),
                despawn_components::<StartUIRoot>,
            );
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::rgb(0.15, 0.15, 0.6);
const HOVERED_BUTTON: Color = Color::rgb(0.15, 0.3, 0.15);
const TEXT_COLOUR: Color = Color::rgb(0.9, 0.9, 0.9);

const BUTTON_HEIGHT: Val = Val::Px(65.0);
const BUTTON_WIDTH: Val = Val::Px(240.0);

fn get_button_bundle() -> ButtonBundle {
    return ButtonBundle {
        style: Style {
            width: BUTTON_WIDTH,
            height: BUTTON_HEIGHT,
            margin: UiRect::all(Val::Px(4.0)),
            border: UiRect::all(Val::Px(1.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: NORMAL_BUTTON.into(),
        ..default()
    };
}

fn get_text_bundle(value: String, font_size: f32) -> TextBundle {
    return TextBundle::from_section(
        value,
        TextStyle {
            font_size,
            color: TEXT_COLOUR,
            ..default()
        },
    );
}

fn get_flex_child(basis_percent: f32, justify_content: JustifyContent) -> NodeBundle {
    return NodeBundle {
        style: Style {
            width: Val::Auto,
            height: Val::Auto,
            justify_content,
            flex_basis: Val::Percent(basis_percent),
            padding: UiRect::all(Val::Px(2.0)),
            // border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        // border_color: BorderColor(Color::RED),
        ..default()
    };
}

pub fn spawn_ui_row(commands: &mut Commands, width: Val, height: Val) -> Entity {
    let entity: Entity = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width,
                height,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                // border: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            // border_color: BorderColor(Color::BLACK),
            ..default()
        })
        .id();

    return entity;
}

// pub fn spawn_ui_row_style(commands: &mut Commands, style: Style) -> Entity {
//     let default_style: Style = Style {
//         display: Display::Flex,
//         flex_direction: FlexDirection::Row,
//         width: Val::Percent(100.),
//         height: Val::Auto,
//         padding: UiRect::all(Val::Px(2.0)),

//         // for debugging
//         border: UiRect::all(Val::Px(1.0)),

//         ..default()
//     };

//     let style: Style = style.merge(default_style);

//     let entity: Entity = commands
//         .spawn(NodeBundle {
//             style: Style {
//                 ..style,
//                 ..default_style
//             },
//             border_color: BorderColor(Color::BLACK),
//             ..default()
//         })
//         .id();

//     return entity;
// }

pub fn spawn_ui_col(commands: &mut Commands, width: Val, height: Val) -> Entity {
    let entity: Entity = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width,
                height,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(6.0)),
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            ..default()
        })
        .id();

    return entity;
}

fn spawn_start_menu(mut commands: Commands) {
    // Create and spawn main UI container
    let container_node = NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };
    let container = commands.spawn((container_node, StartUIRoot)).id();

    // Create and spawn Start Game Button
    let button = commands.spawn((get_button_bundle(), StartButton)).id();
    let button_text = commands
        .spawn(get_text_bundle("Start Game".to_string(), 40.0))
        .id();
    commands.entity(button).push_children(&[button_text]);
    commands.entity(container).push_children(&[button]);

    // Create and spawn Settings Button
    let button = commands.spawn((get_button_bundle(), SettingsButton)).id();
    let button_text = commands
        .spawn(get_text_bundle("Settings".to_string(), 40.0))
        .id();
    commands.entity(button).push_children(&[button_text]);
    commands.entity(container).push_children(&[button]);

    // Create and spawn Quit Button
    let button = commands.spawn((get_button_bundle(), ExitButton)).id();
    let button_text = commands
        .spawn(get_text_bundle("Quit".to_string(), 40.0))
        .id();
    commands.entity(button).push_children(&[button_text]);
    commands.entity(container).push_children(&[button]);
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn on_click_start(
    mut event_writer: EventWriter<OnGameStart>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("clicked Start!");

                // Send game start event.
                event_writer.send(OnGameStart);
            }
            _ => {}
        }
    }
}

fn on_click_exit(
    mut app_exit_events: EventWriter<AppExit>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                app_exit_events.send(AppExit);
            }
            _ => {}
        }
    }
}
