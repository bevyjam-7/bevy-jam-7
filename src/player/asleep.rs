use bevy::prelude::*;

use crate::player::{PlayerState, player_ghost::player_ghost::GhostPlayer, action::MovementController, player::Player, player_ghost::player_ghost::{GhostPlayerAssets, ghost_player}};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(PlayerState::Asleep), remove_initial_player_movement);
    app.add_systems(OnEnter(PlayerState::Asleep), spawn_ghost_player);
    app.add_systems(OnEnter(PlayerState::Asleep), give_ghost_player_movement);
}

// Take movement away from original player
fn remove_initial_player_movement(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    for entity in &player_query {
        commands.entity(entity).remove::<MovementController>();
    }
}

// Spawn the ghost player when player enter asleep state
fn spawn_ghost_player(
    mut commands: Commands,
    ghost_player_assets: Res<GhostPlayerAssets>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(ghost_player(
        200.0,
        ghost_player_assets,
        texture_atlas_layouts,
    ));
    // check if the ghost player was spawned correctly
    println!("Spawned ghost player");
}

// Give the ghost player a movement controller.
fn give_ghost_player_movement(
    mut commands: Commands,
    ghost_player_query: Query<Entity, With<GhostPlayer>>,
) {
    for entity in &ghost_player_query {
        commands.entity(entity).insert(MovementController::default());
    }
}
