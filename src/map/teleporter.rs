use bevy::prelude::*;

use crate::player::{PlayerState, player::Player};

pub(super) fn plugin(app: &mut App) {

}

struct LinkedTelePairs {
    teleporter_a: Entity,
    teleporter_b: Entity,
}

// Teleporter component.
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
struct Teleporter {
    pub destination: Vec3,
    // The entity which is currently inside the teleporter.
    pub containing_entity: Vec<Entity>,
    pub tele_buddy: Option<Entity>,
}


/// Query if player is near a teleporter
fn player_near_teleporter(
    player_state: Res<State<PlayerState>>,
    player_query: Query<&Transform, With<Player>>,
    teleporter_query: Query<(&Transform, &Teleporter)>,
) {
    if player_state.get() != &PlayerState::Awake {
        return; // Only check for teleportation when the player is awake.
    }
    for player_transform in &player_query {
        for (teleporter_transform, teleporter) in &teleporter_query {
            let distance = player_transform.translation.distance(teleporter_transform.translation);
            if distance < 50.0 {
                
            }
        }
    }
}

impl Teleporter {
    /// Teleport the entity inside the teleporter to the teleporter's destination.
    fn teleport(&mut self, commands: &mut Commands) {
        // remove the entity from the teleporter's containing_entity list
        for entity in &self.containing_entity {
            commands.entity(*entity).insert(Transform::from_translation(self.destination));
        }
        self.containing_entity.clear();
    }
}


/// Creates a pair of teleporters
pub fn create_teleporter_pair(
    position_a: Vec3,
    position_b: Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> (impl Bundle, impl Bundle) {
    let teleporter_mesh = &meshes.add(Rectangle::new(50., 50.));
    let teleporter_material = materials.add(ColorMaterial::from(Color::BLACK));
    let teleporter_a = (
        Teleporter { destination: position_b, containing_entity: Vec::with_capacity(1), tele_buddy: None },
        Transform::from_translation(position_a),
        GlobalTransform::default(),
        Mesh2d(teleporter_mesh.clone()),
        MeshMaterial2d(teleporter_material.clone()),
    );
    
    let teleporter_b = (
        Teleporter { destination: position_a, containing_entity: Vec::with_capacity(1), tele_buddy: None },
        Transform::from_translation(position_b),
        GlobalTransform::default(),
        Mesh2d(teleporter_mesh.clone()),
        MeshMaterial2d(teleporter_material.clone()),
    );

    (teleporter_a, teleporter_b)
}

pub fn link_teleporters(
    mut commands: Commands,
    mut teleporter_query: Query<(Entity, &mut Teleporter)>,
    
) {
    let mut teleporters = teleporter_query.iter_mut().collect::<Vec<_>>();
    if teleporters.len() != 2 {
        panic!("Expected exactly 2 teleporters, found {}", teleporters.len());
    }
    let (entity_a, mut teleporter_a) = teleporters.remove(0);
    let (entity_b, mut teleporter_b) = teleporters.remove(0);

    teleporter_a.tele_buddy = Some(entity_b);
    teleporter_b.tele_buddy = Some(entity_a);
}
