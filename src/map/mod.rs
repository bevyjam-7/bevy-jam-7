use bevy::prelude::*;

pub mod level;
pub mod teleporter;
pub mod bread;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        level::plugin,
        teleporter::plugin,

    ));
}