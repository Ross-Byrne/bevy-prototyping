use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    StartMenu,
    LoadingGame,
    InGame,
    Paused,
}

#[derive(Event, Debug)]
pub struct OnGameStart;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_event::<OnGameStart>()
            .add_systems(Update, (on_game_start, game_state_input_events).chain())
            .add_systems(OnEnter(GameState::LoadingGame), on_enter_loading);
    }
}

fn game_state_input_events(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::InGame => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::InGame),
            _ => (),
        }
    }
}

fn on_game_start(
    mut next_state: ResMut<NextState<GameState>>,
    mut event_reader: EventReader<OnGameStart>,
) {
    for _ in event_reader.read() {
        info!("State Transition to in game");
        // transition to in game state
        next_state.set(GameState::LoadingGame);
        return;
    }
}

fn on_enter_loading(mut next_state: ResMut<NextState<GameState>>, state: Res<State<GameState>>) {
    match state.get() {
        GameState::LoadingGame => {
            info!("Transitioning to in game");
            next_state.set(GameState::InGame)
        }
        _ => (),
    }
}
