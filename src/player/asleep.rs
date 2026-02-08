use bevy::prelude::*;

use crate::player::{PlayerState, movement::MovementController, player::{PlayerAssets, Player, player}};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(PlayerState::Asleep), spawn_astral_projection_player);
    app.add_systems(OnEnter(PlayerState::Asleep), remove_initial_player_movement);
    app.add_systems(OnEnter(PlayerState::Asleep), give_astral_projection_player_movement);
}

// Marker component for the astral projection player
#[derive(Component)]
pub struct AstralProjectionPlayer;

// Spawn the player in astral projection state
fn spawn_astral_projection_player(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Name::new("Astral Projection Player"),
        AstralProjectionPlayer,
        player(400.0, &player_assets, &mut texture_atlas_layouts),
    ));
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

// Give movement to astral projection player (find astral projection player entity through query)
fn give_astral_projection_player_movement(
    mut commands: Commands,
    astral_projection_query: Query<Entity, With<AstralProjectionPlayer>>,
) {
    for entity in &astral_projection_query {
        commands.entity(entity).insert(MovementController {
            max_speed: 400.0,
            ..default()
        });
    }
}