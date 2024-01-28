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
            .add_systems(Update, on_game_start.run_if(on_event::<OnGameStart>()))
            .add_systems(Update, game_state_input_events)
            .add_systems(OnEnter(GameState::LoadingGame), on_enter_loading_game);
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

fn on_game_start(mut next_state: ResMut<NextState<GameState>>) {
    // Transition to LoadingGame state,
    // to allow game setup
    next_state.set(GameState::LoadingGame);
}

fn on_enter_loading_game(mut next_state: ResMut<NextState<GameState>>) {
    // After entering LoadingGame state, queue up change to InGame.
    // Is allows assets and setup to happen before game starts.
    next_state.set(GameState::InGame);
}
