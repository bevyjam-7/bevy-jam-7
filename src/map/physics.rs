use avian2d::prelude::*;
use bevy::prelude::*;


#[derive(PhysicsLayer, Clone, Copy, Debug, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    GhostPlayer,
    Border,
    Ground,
    BrokenBridge,
}