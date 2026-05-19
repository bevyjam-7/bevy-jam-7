use bevy::prelude::*;

use crate::inventory::inventory::{ItemKind, ObjectPickable};

pub fn spawn_object(
    kind: ItemKind,
    position: Vec3,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) -> impl Bundle {
    (
        Transform::from_translation(position),
        Mesh2d(mesh),
        MeshMaterial2d(material),
        ObjectPickable {
            kind,
            radius: crate::game_consts::pickup::DEFAULT_RADIUS,
        },
    )
}
