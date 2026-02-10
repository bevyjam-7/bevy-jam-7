use bevy::{ecs::system::entity_command::insert, prelude::*};

use crate::player::{PlayerState, player_ghost::player_ghost::GhostPlayer, action::{MovementController, PlayerAction}, player::Player, player_ghost::player_ghost::{GhostPlayerAssets, ghost_player}};
use leafwing_input_manager::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(PlayerState::Asleep),
        (
            remove_initial_player_movement,
            spawn_ghost_player,
        ).chain(),
    );
}

// Take movement away from original player
pub fn remove_initial_player_movement(
    mut commands: Commands,
    player_query: Query<Entity, Or<(With<Player>, With<GhostPlayer>)>>,
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
) {
    commands.spawn((
        ghost_player(
            200.0,
            ghost_player_assets,
            texture_atlas_layouts,
        ),
    ));
    
    println!("\nSpawned ghost player with input controls");
}