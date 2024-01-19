use bevy::{app::AppExit, prelude::*};

pub struct UIPlugin;

#[derive(Component)]
pub struct StartUI;

#[derive(Component)]
pub struct ExitButton;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Startup, spawn_buttons)
            .add_systems(Update, (button_system, on_click_exit).chain());
    }
}

fn setup(mut _commands: Commands, _asset_server: Res<AssetServer>) {
    println!("Init game UI");
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::rgb(0.15, 0.15, 0.6);
const HOVERED_BUTTON: Color = Color::rgb(0.15, 0.3, 0.15);

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

fn get_text_bundle(text: &str) -> TextBundle {
    return TextBundle::from_section(
        text,
        TextStyle {
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
        },
    );
}

fn spawn_buttons(mut commands: Commands) {
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
    let container = commands.spawn((container_node, StartUI)).id();

    // Create and spawn Start Game Button
    let button = commands.spawn((get_button_bundle(), StartUI)).id();
    let button_text = commands
        .spawn((get_text_bundle("Start Game"), StartUI))
        .id();
    commands.entity(button).push_children(&[button_text]);
    commands.entity(container).push_children(&[button]);

    // Create and spawn Settings Button
    let button = commands.spawn((get_button_bundle(), StartUI)).id();
    let button_text = commands.spawn((get_text_bundle("Settings"), StartUI)).id();
    commands.entity(button).push_children(&[button_text]);
    commands.entity(container).push_children(&[button]);

    // Create and spawn Quit Button
    let button = commands
        .spawn((get_button_bundle(), StartUI, ExitButton))
        .id();
    let button_text = commands.spawn((get_text_bundle("Quit"), StartUI)).id();
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

fn on_click_exit(
    mut app_exit_events: EventWriter<AppExit>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                app_exit_events.send(AppExit);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
