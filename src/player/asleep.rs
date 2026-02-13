use bevy::prelude::*;

use crate::player::{PlayerState, animation::{PlayerAnimation, PlayerAnimationState}, action::{MovementController, PlayerAction}, player::Player, player_ghost::player_ghost::{GhostPlayerAssets, ghost_player}};
use leafwing_input_manager::prelude::*;
use crate::map::level::Level;

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
    level_query: Query<Entity, With<Level>>,
    player_query: Query<&Transform, With<Player>>, // Add player query
) {
    if let Ok(level_entity) = level_query.single() {
        if let Ok(player_transform) = player_query.single() {
            commands.entity(level_entity).with_children(|parent| {
                parent.spawn(
                    ghost_player(
                        100.0,
                        ghost_player_assets,
                        texture_atlas_layouts,
                    )
                ).insert(Transform {
                    translation: player_transform.translation.with_z(1.0), // Player position with custom z
                    scale: Vec2::splat(2.0).extend(1.0), // Preserve the scale from ghost_player
                    ..default()
                });
            });
            println!("\nSpawned ghost player as child of level entity");
        }
    }
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