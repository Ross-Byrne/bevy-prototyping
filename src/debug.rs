use crate::schedule::InGameSet;
use bevy::prelude::*;

#[derive(Resource, Debug)]
struct PrintoutTimer(Timer);

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrintoutTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
        .add_systems(Update, print_position.after(InGameSet::EntityUpdates));
    }
}

fn print_position(
    query: Query<(Entity, &Transform)>,
    mut timer: ResMut<PrintoutTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        // Print number of live entities
        info!("Number of Entities: {:?}", query.iter().len());
    }
}
