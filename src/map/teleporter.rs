use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, game_consts::TELEPORTER_PROXIMITY_RADIUS, player::{PlayerState, player::Player}, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, player_near_teleporter
        .chain()
        .in_set(AppSystems::Update)
        .in_set(PausableSystems),
    );
}
// Teleporter component.
#[derive(Component, Debug, Clone, PartialEq, Copy, Reflect)]
pub struct Teleporter {
    pub destination: Vec3,
    // The entity which is currently inside the teleporter.
    pub containing_entity: Entity,
    // The teleporter that this teleporter is linked to.
    pub tele_buddy: Entity,
    // Should only be true when the player is initiating the teleportation.
    pub can_teleport: bool,
}

/// Query if player is near a teleporter
fn player_near_teleporter(
    player_state: Res<State<PlayerState>>,
    player_query: Query<&Transform, With<Player>>,
    mut teleporter_query: Query<(&Transform, &mut Teleporter)>,
) {
    if player_state.get() != &PlayerState::Awake {
        return; // Only check for teleportation when the player is awake.
    }
    for player_transform in &player_query {
        for (teleporter_transform, mut teleporter) in &mut teleporter_query {
            let distance = player_transform.translation.distance(teleporter_transform.translation);
            if distance < TELEPORTER_PROXIMITY_RADIUS {
                teleporter.can_teleport = true;
                info!("Player is near teleporter, can teleport.");
                return;
            }
        }
    }
}

impl Teleporter {
    /// Teleports the containing entity to the teleporter's buddy's location.
    // pub fn teleport(
    //     &mut self, commands: &mut Commands, 
    //     mut teleporter_query: Query<(&Transform, &mut Teleporter)>
    // ) {
    //     if self.containing_entity == Entity::PLACEHOLDER && self.can_teleport {
    //         info!("Teleporter has no containing entity or cannot teleport.");
    //         return; // No entity to teleport.
    //     }
    //     let Ok((buddy_transform, mut buddy_teleporter)) = teleporter_query.get_mut(self.tele_buddy) else {
    //         info!("Buddy teleporter doesn't exist.");
    //         return; // Buddy teleporter doesn't exist.
    //     };
    //     // Check if buddy teleporter is currently occupied, if so, do not teleport.
    //     if buddy_teleporter.containing_entity != Entity::PLACEHOLDER {
    //         info!("Buddy teleporter is currently occupied, cannot teleport.");
    //         return; // Buddy teleporter is currently occupied.
    //     }
    //     let teleporter_item = self.containing_entity;
    //     // Clear self's containing entity
    //     self.containing_entity = Entity::PLACEHOLDER;
    //     commands.entity(teleporter_item)
    //         .insert(Transform::from_translation(buddy_transform.translation));

    //     // Update the buddy teleporter's containing entity.
    //     buddy_teleporter.set_containing_entity(teleporter_item);
    // }

    /// Sets the containing entity of the teleporter.
    fn set_containing_entity(&mut self, entity: Entity) {
        self.containing_entity = entity;
    }


}


/// Bundle creation for a pair of teleporters
pub fn create_teleporter_pair(
    position_a: Vec3,
    position_b: Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> (impl Bundle, impl Bundle) {
    let teleporter_mesh = &meshes.add(Rectangle::new(50., 50.));
    let teleporter_material = materials.add(ColorMaterial::from(Color::BLACK));
    let teleporter_a = (
        Name::new("Teleporter A"),
        DespawnOnExit(Screen::Gameplay),
        Transform::from_translation(position_a),
        GlobalTransform::default(),
        Mesh2d(teleporter_mesh.clone()),
        MeshMaterial2d(teleporter_material.clone()),
    );
    
    let teleporter_b = (
        Name::new("Teleporter B"),
        DespawnOnExit(Screen::Gameplay),
        Transform::from_translation(position_b),
        GlobalTransform::default(),
        Mesh2d(teleporter_mesh.clone()),
        MeshMaterial2d(teleporter_material.clone()),
    );

    (teleporter_a, teleporter_b)
}

/// Links two teleporters together so that they teleport to each other.
/// kinda hacky because im passing the position for both this and the create_teleporter_pair function, but it works for now.
pub fn link_teleporters(
    commands: &mut Commands,
    position_a: Vec3,
    position_b: Vec3, 
    teleporter_a: impl Bundle, 
    teleporter_b: impl Bundle
) {
    let teleporter_a_entity = commands.spawn(teleporter_a).id();
    let teleporter_b_entity = commands.spawn(teleporter_b).id();

    commands.entity(teleporter_a_entity).insert(Teleporter { destination: position_b, containing_entity: Entity::PLACEHOLDER, tele_buddy: teleporter_b_entity, can_teleport: false });
    commands.entity(teleporter_b_entity).insert(Teleporter { destination: position_a, containing_entity: Entity::PLACEHOLDER, tele_buddy: teleporter_a_entity, can_teleport: false });
}