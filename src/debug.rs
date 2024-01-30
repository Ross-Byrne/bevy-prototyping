use crate::schedule::InGameSet;
use crate::state::GameState;
use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct PrintoutTimer(pub Timer);

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrintoutTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
        .add_systems(Update, log_entity_count.after(InGameSet::EntityUpdates))
        .add_systems(OnEnter(GameState::StartMenu), log_on_enter_game_state)
        .add_systems(OnEnter(GameState::LoadingGame), log_on_enter_game_state)
        .add_systems(OnEnter(GameState::InGame), log_on_enter_game_state)
        .add_systems(OnEnter(GameState::Paused), log_on_enter_game_state);
    }
}

fn log_entity_count(
    query: Query<(Entity, &Transform)>,
    mut timer: ResMut<PrintoutTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        // Print number of live entities
        info!("Number of Entities: {:?}", query.iter().len());
    }
}

fn log_on_enter_game_state(state: Res<State<GameState>>) {
    info!("On Enter GameState::{:?}", state.get())
}
