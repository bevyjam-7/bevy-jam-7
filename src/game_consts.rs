use bevy::prelude::*;

// Map related constants
pub const WITCH_HOUSE_ATLAS_COLS: u32 = 15;
pub const WITCH_HOUSE_ATLAS_ROWS: u32 = 1;
pub const SCENE_TILE_SIZES: UVec2 = UVec2::new(256, 320);
pub const WITCH_HOUSE_LOCATION: Vec3 = Vec3::new(0., 0., 0.);
pub const BRIDGE_SECTION_LOCATION: Vec3 = Vec3::new(1400., 0., -10.);
pub const OLD_HOUSE_LOCATION: Vec3 = Vec3::new(2800., 0., -10.);

// Thickness of collision borders around the map
pub const BORDER_THICKNESS: f32 = 50.0;
// x right border offset, y left border offset, z top border offset, w bottom border offset FOR Y AXIS
pub const Y_BRIDGE_SECTION_BORDER_OFFSETS: Vec4 = Vec4::new(0.,0.,-265.,265.);

// Player related constants
pub const PLAYER_ATLAS_COLS: usize = 4;
pub const PLAYER_ATLAS_ROWS: usize = 2;
// The "z" position determines the rendering order for entities.
pub const PLAYER_Z_POSITION: f32 = 2.0;
pub const PLAYER_SPEED: f32 = 5000.0;

// Npc related constants
pub const NPC_ATLAS_COLS: usize = 2;
pub const NPC_ATLAS_ROWS: usize = 1;
pub const NPC_LOCATION: Vec3 = Vec3::new(137., -100., 0.0);

// Teleporter related constants
pub const TELEPORTER_A_LOCATION: Vec3 = Vec3::new(-70., -70., 1.);
pub const TELEPORTER_B_LOCATION: Vec3 = Vec3::new(0., 0., 1.);
pub const TELEPORTER_PROXIMITY_RADIUS: f32 = 30.0;

pub const TRIPWIRE_HOUSE_TO_BRIDGE_POSITION: Vec3 = Vec3::new(235., 78., 0.);
// The position is relative to the center of the bridge section
pub const TRIPWIRE_BRIDGE_TO_HOUSE_POSITION: Vec3 = Vec3::new(-225., 0., 0.);
pub const TRIPWIRE_BRIDGE_TO_OLD_POSITION: Vec3 = Vec3::new(225., 0., 0.);
pub const TRIPWIRE_OLD_TO_BRIDGE_POSITION: Vec3 = Vec3::new(-225., 0., 0.,);


pub mod pickup {
    pub const DEFAULT_RADIUS: f32 = 30.0;
}