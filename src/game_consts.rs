use bevy::prelude::*;

// Map related constants
pub const WITCH_HOUSE_ATLAS_COLS: u32 = 15;
pub const WITCH_HOUSE_ATLAS_ROWS: u32 = 1;
pub const SCENE_TILE_SIZES: UVec2 = UVec2::new(256, 320);
pub const WITCH_HOUSE_LOCATION: Vec3 = Vec3::new(0., 0., 0.);
pub const BRIDGE_SECTION_LOCATION: Vec3 = Vec3::new(1200., 0., 0.);

// Player related constants
pub const PLAYER_ATLAS_COLS: usize = 4;
pub const PLAYER_ATLAS_ROWS: usize = 2;
// The "z" position determines the rendering order for entities.
pub const PLAYER_Z_POSITION: f32 = 2.0;

// Teleporter related constants
pub const TELEPORTER_A_LOCATION: Vec3 = Vec3::new(-70., -70., 1.);
pub const TELEPORTER_B_LOCATION: Vec3 = Vec3::new(-400., 0., 1.);
pub const TELEPORTER_PROXIMITY_RADIUS: f32 = 50.0;

pub const TRIPWIRE_HOUSE_TO_BRIDGE_POSITION: Vec3 = Vec3::new(235., 78., 1.);

pub mod pickup {
    pub const DEFAULT_RADIUS: f32 = 30.0;
}