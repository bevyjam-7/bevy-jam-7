use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {

}

#[derive(Component, Debug, Reflect)]
struct Pickable;


// Input transform, mesh, and material to spawn a pickable item
pub fn spawn_pickable(
    transform: Vec3,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) -> impl Bundle {
    (
        Pickable,
        Transform::from_translation(transform),
        Mesh2d(mesh),
        MeshMaterial2d(material),
    )
}