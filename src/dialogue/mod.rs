use bevy::prelude::*;

pub mod assets;
pub mod option_selection;
pub mod setup;
pub mod typewriter;
pub mod updating;

#[derive(Debug, Default, Clone, Copy, SystemSet, Eq, PartialEq, Hash)]
pub struct YarnSpinnerDialogueViewSystemSet;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        assets::ui_assets_plugin,
        setup::ui_setup_plugin,
        updating::ui_updating_plugin,
        typewriter::typewriter_plugin,
        option_selection::option_selection_plugin,
    ));
}
