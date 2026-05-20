use bevy::prelude::*;

use crate::{
    game_consts::PLAYER_SPEED,
    player::{
        PlayerState,
        actions::{MovementController, PlayerAction},
        player::{AwakePlayer, GhostPlayer},
    },
};
use ::leafwing_input_manager::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(PlayerState::Awake), despawn_ghost_player);
    app.add_systems(
        OnEnter(PlayerState::Awake),
        restore_player_movement.after(despawn_ghost_player),
    );
}

// Despawn ghost player
fn despawn_ghost_player(
    mut commands: Commands,
    ghost_player_query: Query<Entity, With<GhostPlayer>>,
) {
    for entity in &ghost_player_query {
        commands.entity(entity).despawn();
    }
}

// Restores player movement when player is awake.
fn restore_player_movement(mut commands: Commands, player_query: Query<Entity, With<AwakePlayer>>) {
    for entity in &player_query {
        commands.entity(entity).insert((
            MovementController {
                max_speed: PLAYER_SPEED,
                ..default()
            },
            ActionState::<PlayerAction>::default(),
            PlayerAction::default_input_map(),
        ));
    }
    info!("\nRestored movement and input to player");
}
