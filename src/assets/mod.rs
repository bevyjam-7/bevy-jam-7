use bevy::prelude::*;

pub mod asset_tracking;
pub mod animation;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        asset_tracking::plugin,
        animation::plugin,
    ));
}