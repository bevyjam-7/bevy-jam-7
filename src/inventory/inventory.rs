use bevy::prelude::*;
use std::collections::HashMap;
use std::fmt;

use crate::game_consts::pickup::DEFAULT_RADIUS;

pub(super) fn plugin(app: &mut App) {}

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

#[derive(Component, Debug)]
pub struct ObjectPickable {
    pub kind: ItemKind,
    pub radius: f32,
}

impl ObjectPickable {
    pub fn new(kind: ItemKind) -> Self {
        Self {
            kind,
            radius: DEFAULT_RADIUS,
        }
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
