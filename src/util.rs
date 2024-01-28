// Module for common functions

use bevy::prelude::*;

pub fn despawn_components<T: Component>(
    mut commands: Commands,
    to_despawn: Query<Entity, With<T>>,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
