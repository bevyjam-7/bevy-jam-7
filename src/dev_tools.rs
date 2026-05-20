//! Development tools for the game. This plugin is only enabled in dev builds.

use crate::{Pause, game_gui::screens::Screen};
use avian2d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::{
    color::palettes::css::BLUE, dev_tools::states::log_transitions,
    input::common_conditions::input_just_pressed, prelude::*,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);
    app.add_systems(Update, log_transitions::<Pause>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
    app.add_plugins(PhysicsDebugPlugin::default())
        .insert_gizmo_config(
            PhysicsGizmos {
                aabb_color: Some(Color::from(BLUE)),
                ..default()
            },
            GizmoConfig::default(),
        );
    app.add_plugins(EguiPlugin::default());
    app.add_plugins(WorldInspectorPlugin::new());
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
