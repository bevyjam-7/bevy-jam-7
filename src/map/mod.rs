use bevy::prelude::*;

pub mod level;
pub mod teleporter;
pub mod object;
pub mod events;
pub mod animation;
pub mod physics;
mod borders;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        level::plugin,
        teleporter::plugin,
        animation::plugin,
        events::plugin,
    ));
}