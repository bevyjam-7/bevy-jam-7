use bevy::prelude::*;

use crate::{
    inventory::inventory::{Inventory, ItemKind, ObjectPickable},
    map::{events::DialogueSelected, object::spawn_object},
    player::player::AwakePlayer,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_inventory_ui)
        .add_systems(Update, update_inventory_ui);
}

pub fn drop_item(
    mut commands: Commands,
    mut inventory: ResMut<Inventory>,
    player_query: Query<&Transform, With<AwakePlayer>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Attempting to drop item...");
    let Ok(player_transform) = player_query.single() else {
        println!("Player not found, cannot drop item");
        return;
    };

    let player_pos = player_transform.translation;

    if let Some(item_to_drop) = inventory.get(ItemKind::Food1) {
        println!("Dropping item: {}", ItemKind::Food1);

        // For demonstration, we'll just drop a bread item
        let dropped_item = spawn_object(
            item_to_drop,
            player_pos + Vec3::new(0., 0., 1.), // Drop slightly above the player
            meshes.add(Rectangle::new(30., 20.)),
            materials.add(ColorMaterial::from(Color::hsv(0., 1., 1.))),
        );
        // Debug for location of where the item is dropped
        println!(
            "Dropped item at position: {:?}",
            player_pos + Vec3::new(0., 0., 1.)
        );

        inventory.remove(item_to_drop);
        commands.spawn(dropped_item);
        return;
    }

    if let Some(item_to_drop) = inventory.get(ItemKind::Object1) {
        println!("Dropping item: {}", ItemKind::Object1);

        let dropped_item = spawn_object(
            item_to_drop,
            player_pos + Vec3::new(0., 0., 1.),
            meshes.add(Rectangle::new(40., 15.)),
            materials.add(ColorMaterial::from(Color::srgb(0.55, 0.27, 0.07))), // Brown color
        );

        println!(
            "Dropped item at position: {:?}",
            player_pos + Vec3::new(0., 0., 1.)
        );
        inventory.remove(item_to_drop);
        commands.spawn(dropped_item);
        return;
    }

    info!("No item to drop")
}

/// System that checks for and processes item pickups.
pub fn handle_pickups(
    mut commands: Commands,
    mut inventory: ResMut<Inventory>,
    player_query: Query<&Transform, With<AwakePlayer>>,
    pickables: Query<(Entity, &GlobalTransform, &ObjectPickable)>,
) {
    println!("Checking for pickups...");
    let Ok(player_transform) = player_query.single() else {
        println!("Player not found, cannot pick up items");
        return;
    };

    let player_pos = player_transform.translation.truncate();
    let mut collected = Vec::new();

    // Check distance to each pickable
    for (entity, global_transform, pickable) in pickables.iter() {
        let item_pos = global_transform.translation().truncate();
        let distance_sq = player_pos.distance_squared(item_pos);
        // Prints player position, item position, and distance for debugging
        println!(
            "Player position: {:?}, Item position: {:?}, Distance squared: {}",
            player_pos, item_pos, distance_sq
        );
        if distance_sq <= pickable.radius * pickable.radius {
            collected.push((entity, pickable.kind));
        }
    }

    // Process collected items
    for (entity, kind) in collected {
        commands.entity(entity).despawn();
        let count = inventory.add(kind);
        commands.trigger(DialogueSelected {
            s: "AcquireFirstItem".to_string(),
        });
        info!(
            " Picked up {} (total: {}) — inventory: {}",
            kind,
            count,
            inventory.summary()
        );
    }
}

#[derive(Component)]
struct InventoryText;

fn setup_inventory_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Items Collectd: "),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        InventoryText,
    ));
}

fn update_inventory_ui(
    inventory: Res<Inventory>,
    mut query: Query<&mut Text, With<InventoryText>>,
) {
    if !inventory.is_changed() {
        return;
    }

    let Ok(mut text) = query.single_mut() else {
        return;
    };
    let items_text = inventory.summary();

    **text = format!("Items Collected: {}", items_text);
}
