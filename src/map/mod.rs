use bevy::prelude::*;

pub mod level;
pub mod teleporter;
mod object;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        level::plugin,
        teleporter::plugin,

    ));
}