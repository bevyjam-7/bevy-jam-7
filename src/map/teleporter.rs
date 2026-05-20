use bevy::prelude::*;

use crate::map::interaction_box::{InteractableObject, ObjectInteractionType};
use crate::{
    game_consts::TELEPORTER_PROXIMITY_RADIUS,
    game_gui::screens::Screen
};

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(
    //     Update,
    //     (ghost_player_near_teleporter, detect_items_on_teleporters)
    //         .chain()
    //         .in_set(AppSystems::Update)
    //         .in_set(PausableSystems),
    // );
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

#[derive(Component, Debug, Reflect)]
pub struct Teleportable {
    pub on_teleporter_entity: Entity,
}

fn link_object_to_tp(
    entity: Entity,
    teleporter_entity: Entity,
    commands: &mut Commands,
    teleporter_query: &mut Query<(Entity, &Transform, &mut Teleporter)>,
) {
    let Ok((_, _, mut teleporter)) = teleporter_query.get_mut(teleporter_entity) else {
        info!("\nTeleporter entity not found");
        return;
    };

    if teleporter.containing_entity != Entity::PLACEHOLDER {
        info!("\nTeleporter is already occupied, cannot link object.");
        return;
    }

    commands.entity(entity).insert(Teleportable {
        on_teleporter_entity: teleporter_entity,
    });

    teleporter.containing_entity = entity;
    info!(
        "\nLinked entity {:?} to teleporter {:?}",
        entity, teleporter_entity
    );
}

fn unlink_object_from_tp(
    entity: Entity,
    teleporter_entity: Entity,
    commands: &mut Commands,
    teleporter_query: &mut Query<(Entity, &Transform, &mut Teleporter)>,
) {
    let Ok((_, _, mut teleporter)) = teleporter_query.get_mut(teleporter_entity) else {
        info!("\nTeleporter entity not found");
        return;
    };

    if teleporter.containing_entity != entity {
        info!("\nEntity is not on this teleporter, cannot unlink.");
        return;
    }

    commands.entity(entity).remove::<Teleportable>();
    teleporter.containing_entity = Entity::PLACEHOLDER;
    info!(
        "\nUnlinked entity {:?} from teleporter {:?}",
        entity, teleporter_entity
    );
}

/// Bundle creation for a pair of teleporters
pub fn create_teleporter_pair(
    position_a: Vec3,
    position_b: Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> (impl Bundle, impl Bundle) {
    let teleporter_mesh = &meshes.add(Rectangle::new(200., 200.));
    let teleporter_material = materials.add(ColorMaterial::from(Color::BLACK.with_alpha(0.5)));
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
    teleporter_b: impl Bundle,
) {
    let teleporter_a_entity = commands.spawn(teleporter_a).id();
    let teleporter_b_entity = commands.spawn(teleporter_b).id();

    commands.entity(teleporter_a_entity).insert(Teleporter {
        destination: position_b,
        containing_entity: Entity::PLACEHOLDER,
        tele_buddy: teleporter_b_entity,
        can_teleport: false,
    }).insert(InteractableObject {
        object_type: ObjectInteractionType::Teleporter,
        interaction_radius: TELEPORTER_PROXIMITY_RADIUS,
    });
    commands.entity(teleporter_b_entity).insert(Teleporter {
        destination: position_a,
        containing_entity: Entity::PLACEHOLDER,
        tele_buddy: teleporter_a_entity,
        can_teleport: false,
    }).insert(InteractableObject {
        object_type: ObjectInteractionType::Teleporter,
        interaction_radius: TELEPORTER_PROXIMITY_RADIUS,
    });
}
