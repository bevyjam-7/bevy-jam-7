use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::map::level_spawn::Level;
use crate::player::actions::{MovementController, PlayerAction};
use crate::{
    assets::animation::{FacingDirection, PlayerAnimation, PlayerAnimationState},
    map::events::DialogueSelected,
    player::{
        PlayerState,
        player::{AwakePlayer, GhostPlayer, PlayerAssets, player},
    },
};
use leafwing_input_manager::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(PlayerState::Asleep),
        (
            remove_initial_player_movement,
            reset_player_animation_to_idle,
            spawn_ghost_player,
        )
            .chain(),
    );
}

// Take movement away from original player
pub fn remove_initial_player_movement(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut LinearVelocity), With<AwakePlayer>>,
) {
    for (entity, mut velocity) in player_query.iter_mut() {
        commands
            .entity(entity)
            .remove::<MovementController>()
            .remove::<ActionState<PlayerAction>>()
            .remove::<InputMap<PlayerAction>>();
        velocity.x = 0.0;
        velocity.y = 0.0;
    }
    info!("\nRemoved movement and input from player");
}

// Spawn the ghost player when player enter asleep state
fn spawn_ghost_player(
    mut commands: Commands,
    player_state: Res<State<PlayerState>>,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    level_query: Query<Entity, With<Level>>,
    player_query: Query<&Transform, With<AwakePlayer>>, // Add player query
) {
    if let Ok(level_entity) = level_query.single() {
        if let Ok(player_transform) = player_query.single() {
            commands.entity(level_entity).with_children(|parent| {
                parent
                    .spawn(player(
                        &player_state,
                        &player_assets,
                        &mut texture_atlas_layouts,
                        FacingDirection::Side,
                    ))
                    .insert(Transform {
                        translation: player_transform.translation.with_z(1.0), // Player position with custom z
                        scale: Vec2::splat(2.0).extend(1.0), // Preserve the scale from ghost_player
                        ..default()
                    })
                    .insert(GhostPlayer);
            });
            info!("\nSpawned ghost player as child of level entity");
        }
    }
    commands.trigger(DialogueSelected {
        s: "GhostForm".to_string(),
    });
}
// Reset player animation to idle when falling asleep
fn reset_player_animation_to_idle(
    mut player_query: Query<&mut PlayerAnimation, With<AwakePlayer>>,
) {
    for mut animation in &mut player_query {
        // Reset to idle state while preserving the facing direction
        animation.update_state(PlayerAnimationState::Idling);
        info!("\nReset player animation to idle (asleep)");
    }
}
