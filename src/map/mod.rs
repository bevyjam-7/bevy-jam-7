use bevy::prelude::*;

pub mod borders;
pub mod events;
pub mod level_spawn;
pub mod map_sections;
pub mod npc;
pub mod object;
pub mod old_bookshelf;
pub mod physics;
pub mod teleporter;
pub mod interaction_box;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        map_sections::plugin,
        level_spawn::plugin,
        teleporter::plugin,
        events::plugin,
        npc::plugin,
        object::plugin,
    ));
}

// Distance,
// Get every entity position
// query for specific entity for every thing that wants to use this distance
// function?
//
