use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Startup, (spawn_box, spawn_text));
    }
}

fn setup(mut _commands: Commands, _asset_server: Res<AssetServer>) {
    println!("Init game UI");
}

fn spawn_box(mut commands: Commands) {
    let container = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_content: AlignContent::Start,
            ..default()
        },
        ..default()
    };

    let square = NodeBundle {
        style: Style {
            width: Val::Px(100.),
            height: Val::Px(80.),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        background_color: Color::rgb(0.65, 0.65, 0.65).into(),
        ..default()
    };

    let parent = commands.spawn(container).id();
    let child = commands.spawn(square).id();

    commands.entity(parent).push_children(&[child]);
}

fn spawn_text(mut commands: Commands) {
    let text = "Hello world!";

    commands.spawn(
        TextBundle::from_section(
            text,
            TextStyle {
                font_size: 100.0,
                color: Color::WHITE,
                ..default()
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        })
    );
}