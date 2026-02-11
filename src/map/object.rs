use bevy::prelude::*;

use crate::inventory::inventory::{ItemKind, ObjectPickable};

pub(super) fn plugin(app: &mut App) {

}



pub fn spawn_object(
    input_kind: ItemKind,
    position: Vec3,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) -> impl Bundle {
    (
        Transform::from_translation(position),
        Mesh2d(mesh),
        MeshMaterial2d(material),
        ObjectPickable {
            kind: input_kind,
            radius: 40.0,
        },
    )
}