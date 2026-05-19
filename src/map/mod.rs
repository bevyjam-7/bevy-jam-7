use bevy::prelude::*;

pub mod borders;
pub mod events;
pub mod level;
pub mod npc;
pub mod object;
pub mod old_bookshelf;
pub mod physics;
pub mod teleporter;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        level::plugin,
        teleporter::plugin,
        events::plugin,
        npc::plugin,
    ));
}
