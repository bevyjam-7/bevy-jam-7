use bevy::app::App;

pub mod menus;
pub mod screens;
pub mod theme;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((screens::plugin, menus::plugin));
}
