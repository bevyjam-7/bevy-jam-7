use bevy::prelude::*;

// Map related constants
pub const WITCH_HOUSE_ATLAS_COLS: u32 = 15;
pub const WITCH_HOUSE_ATLAS_ROWS: u32 = 1;

// Player related constants
pub const PLAYER_ATLAS_COLS: usize = 4;
pub const PLAYER_ATLAS_ROWS: usize = 2;
// The "z" position determines the rendering order for entities.
pub const PLAYER_Z_POSITION: f32 = 2.0;

// Teleporter related constants
pub const TELEPORTER_A_LOCATION: Vec3 = Vec3::new(200., 0., 1.);
pub const TELEPORTER_B_LOCATION: Vec3 = Vec3::new(-200., 0., 1.);
pub const TELEPORTER_PROXIMITY_RADIUS: f32 = 50.0;

pub mod pickup {
    pub const DEFAULT_RADIUS: f32 = 30.0;
}