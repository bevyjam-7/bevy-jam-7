use bevy::prelude::*;

use crate::player::PlayerState;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        
    ));
}

const INVENTORY_SIZE: usize = 3;

// When the player is awake, they are able to interact with the world

// Function which allows player to interact with the world

/// Inventory for the player (only visible when awake)
fn inventory_system(mut commands: Commands) {
    // Spawn inventory UI
    commands.spawn((
        Node {
            display: Display::Grid,
            align_self: AlignSelf::FlexStart,
            justify_self: JustifySelf::Center,
            ..Default::default()
        }, 
        Pickable::IGNORE,
        DespawnOnExit(PlayerState::Awake),
        Name::new("Inventory"),
        children![
            // Generate Inventory slots
            
        ]
    ));
}