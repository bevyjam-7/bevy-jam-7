pub mod inventory;
pub mod systems;

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, inventory::{inventory::Inventory, systems::handle_pickups}};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Inventory>();

    app.add_plugins((
        inventory::plugin,
        systems::plugin,
    ));
}