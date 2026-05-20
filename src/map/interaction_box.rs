use bevy::prelude::*;

use crate::map::object::ItemKind;

pub(super) fn plugin(app: &mut App) {

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectInteractionType {
    Pickable(ItemKind),
    NPC,
    Teleporter,
    Bookshelf,
    Bridge,
}

#[derive(Component)]
pub struct InteractableObject {
    pub object_type: ObjectInteractionType,
    pub interaction_radius: f32,
}

