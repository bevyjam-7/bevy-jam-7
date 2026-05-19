pub mod inventory;
pub mod systems;

use bevy::prelude::*;

use crate::{
    inventory::{inventory::Inventory},
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Inventory>();

    app.add_plugins(
        systems::plugin
    );
}
