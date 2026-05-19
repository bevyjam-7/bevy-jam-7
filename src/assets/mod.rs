use bevy::prelude::*;

pub mod animation;
pub mod asset_tracking;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((asset_tracking::plugin, animation::plugin));
}
