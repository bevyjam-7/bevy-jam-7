use bevy::prelude::*;

// Player related constants
pub const PLAYER_ATLAS_COLS: usize = 4;
pub const PLAYER_ATLAS_ROWS: usize = 2;
// The "z" position determines the rendering order for entities.
pub const PLAYER_Z_POSITION: f32 = 2.0;

// Teleporter related constants
pub const TELEPORTER_A_LOCATION: Vec3 = Vec3::new(200., 0., 1.);
pub const TELEPORTER_B_LOCATION: Vec3 = Vec3::new(-200., 0., 1.);
pub const TELEPORTER_PROXIMITY_RADIUS: f32 = 50.0;