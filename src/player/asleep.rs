use bevy::{ecs::system::entity_command::insert, prelude::*};

use crate::player::{PlayerState, animation::{PlayerAnimation, PlayerAnimationState}, action::{MovementController, PlayerAction}, player::Player, player_ghost::player_ghost::{GhostPlayerAssets, ghost_player}};
use leafwing_input_manager::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(PlayerState::Asleep),
        (
            remove_initial_player_movement,
            reset_player_animation_to_idle,
            spawn_ghost_player,
        ).chain(),
    );
}

// Take movement away from original player
pub fn remove_initial_player_movement(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    for entity in &player_query {
        commands.entity(entity)
            .remove::<MovementController>()
            .remove::<ActionState<PlayerAction>>()
            .remove::<InputMap<PlayerAction>>();
    }
    println!("\nRemoved movement and input from player");
}

// Spawn the ghost player when player enter asleep state
fn spawn_ghost_player(
    mut commands: Commands,
    ghost_player_assets: Res<GhostPlayerAssets>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let mut entity_commands = commands.spawn((
        ghost_player(
            100.0,    // originally 100 
            ghost_player_assets,
            texture_atlas_layouts,
        ),
    ));

    // Override the Transfrom component
    if let Ok(player_transform) = player_query.single() {
        let mut ghost_transform = *player_transform;
        ghost_transform.translation.z += 1.0; // Spawn one layer above the player
        entity_commands.insert(ghost_transform);
    }
    
    println!("\nSpawned ghost player with input controls");
}

// Reset player animation to idle when falling asleep
fn reset_player_animation_to_idle(
    mut player_query: Query<&mut PlayerAnimation, With<Player>>,
) {
    for mut animation in &mut player_query {
        // Reset to idle state while preserving the facing direction
        animation.update_state(PlayerAnimationState::Idling);
        println!("\nReset player animation to idle (asleep)");
    }
}