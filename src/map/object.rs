use std::{collections::HashMap, fmt};

use bevy::{image::{ImageLoaderSettings, ImageSampler}, prelude::*};

use crate::{assets::asset_tracking::LoadResource, map::interaction_box::{InteractableObject, ObjectInteractionType}};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<ObjectAssets>();
    app.init_resource::<Inventory>();

    app.add_systems(Startup, setup_inventory_ui)
        .add_systems(Update, update_inventory_ui);
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemKind {
    Food1,
    Object1,
}

impl ItemKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            ItemKind::Food1 => "Bread",
            ItemKind::Object1 => "Bridge Piece",
        }
    }
}

impl fmt::Display for ItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[derive(Resource, Default, Debug)]
pub struct Inventory {
    items: HashMap<ItemKind, u32>,
}

impl Inventory {
    /// Add an item to the inventory, returns new count.
    pub fn add(&mut self, kind: ItemKind) -> u32 {
        let entry = self.items.entry(kind).or_insert(0);
        *entry += 1;
        *entry
    }

    /// Remove an item from the inventory, returns new count
    pub fn remove(&mut self, kind: ItemKind) {
        // get the count for the item, decrement it, and remove it if the count reaches zero
        if let Some(count) = self.items.get_mut(&kind) {
            if *count > 0 {
                *count -= 1;
                if *count == 0 {
                    self.items.remove(&kind);
                }
            }
        }
    }

    /// Get the item type
    pub fn get(&self, kind: ItemKind) -> Option<ItemKind> {
        if self.items.contains_key(&kind) {
            Some(kind)
        } else {
            None
        }
    }

    /// Get a summary string of inventory contents.
    pub fn summary(&self) -> String {
        if self.items.is_empty() {
            return "empty".to_string();
        }

        let mut parts: Vec<String> = self
            .items
            .iter()
            .map(|(kind, count)| format!("{}: {}", kind, count))
            .collect();
        parts.sort();
        parts.join(", ")
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

pub fn spawn_object(
    kind: ItemKind,
    interaction_object: ObjectInteractionType,
    object_assets: &ObjectAssets,
    position: Vec3,
) -> impl Bundle {
    (
        Transform::from_translation(position),
        InteractableObject {
            object_type: interaction_object,
            interaction_radius: crate::game_consts::pickup::DEFAULT_RADIUS,
        },
        match kind {
            ItemKind::Food1 => 
            Sprite::from_image(
                object_assets.bread.clone(),
            ),
            ItemKind::Object1 => 
            Sprite::from_image(
                object_assets.plank.clone(),
            ),
        },
    )
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct ObjectAssets {
    #[dependency]
    pub bread: Handle<Image>,
    #[dependency]
    pub plank: Handle<Image>,
}

impl FromWorld for ObjectAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            bread: assets.load_with_settings(
                "images/bread.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            plank: assets.load_with_settings(
                "images/plank.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}
