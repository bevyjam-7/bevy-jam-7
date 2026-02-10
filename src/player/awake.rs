use bevy::prelude::*;

use crate::player::{PlayerState, action::MovementController, player::Player, player_ghost::player_ghost::GhostPlayer,asleep::remove_initial_player_movement};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(PlayerState::Awake),
        (
            remove_initial_player_movement,
        ).chain(),
    );
}

// Restores player movement when player is awake.
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