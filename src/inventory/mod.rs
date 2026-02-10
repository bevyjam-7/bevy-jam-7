pub mod inventory;
pub mod systems;

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, inventory::{inventory::Inventory, systems::handle_pickups}};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Inventory>()
        .add_systems(Update, handle_pickups        
        .chain()
        .in_set(AppSystems::RecordInput)
        .in_set(PausableSystems)
    );

    app.add_plugins((
        inventory::plugin,
        systems::plugin,
    ));
}