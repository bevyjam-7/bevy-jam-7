use bevy::prelude::*;

use crate::player::{PlayerState, action::MovementController, player::Player};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnExit(PlayerState::Asleep), restore_player_movement);
}


// Gives player movement when player is awake.
fn restore_player_movement(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    for entity in &player_query {
        commands.entity(entity)
            .insert(MovementController {
                max_speed: 400.0,
                ..default()
            });
    }
}