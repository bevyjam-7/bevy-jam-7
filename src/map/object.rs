use bevy::prelude::*;

use crate::inventory::inventory::ItemKind;

pub(super) fn plugin(app: &mut App) {

}



fn spawn_object(
    position: Vec3,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) -> impl Bundle {
    (

        Transform::from_translation(position),
        Mesh2d(mesh),
        MeshMaterial2d(material),

    )
}