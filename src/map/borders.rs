use bevy::{ecs::event::Trigger, prelude::*};
use avian2d::prelude::*;

use crate::{game_consts::{BORDER_THICKNESS, BROKEN_BRIDGE_COLLISION_LOCATION, SCENE_TILE_SIZES}, map::{self, physics::GameLayer}};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Border;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct BrokenBridgeCollider;

/// Offsets are provided in Vec4 coordinates
/// x coord -> right border offset
/// y coord -> left border offset
/// z coord -> top border offset
/// w coord -> bottom border offset
pub fn spawn_box_borders(map_location: Vec3, x_offset: Vec4, y_offset: Vec4) -> Vec<(Name, Border, LockedAxes, RigidBody, Collider, CollisionLayers, Transform)> {

    let borders = [
        // Right border
        (
            Vec3::new(map_location.x + SCENE_TILE_SIZES.x as f32 + BORDER_THICKNESS / 2. + x_offset.x, 0. + y_offset.x, 0.),
            Vec2::new(BORDER_THICKNESS, SCENE_TILE_SIZES.y as f32 * 2.),
        ),
        // Left border
        (
            Vec3::new(map_location.x - SCENE_TILE_SIZES.x as f32 - BORDER_THICKNESS / 2. + x_offset.y, 0. + y_offset.y, 0.),
            Vec2::new(BORDER_THICKNESS, SCENE_TILE_SIZES.y as f32 * 2.),
        ),
        // Top border
        (
            Vec3::new(map_location.x + x_offset.z, map_location.y + SCENE_TILE_SIZES.y as f32 + BORDER_THICKNESS / 2. + y_offset.z, 0.),
            Vec2::new(SCENE_TILE_SIZES.x as f32 * 2. + BORDER_THICKNESS * 2., BORDER_THICKNESS),
        ),
        // Bottom border
        (
            Vec3::new(map_location.x + x_offset.w, map_location.y - SCENE_TILE_SIZES.y as f32 - BORDER_THICKNESS / 2. + y_offset.w, 0.),
            Vec2::new(SCENE_TILE_SIZES.x as f32 * 2. + BORDER_THICKNESS * 2., BORDER_THICKNESS),
        ),
    ];

    let mut border_entities = Vec::new();

    for (position, scale) in borders.iter() {
        border_entities.push(
            (
                Name::new("Border"),
                Border,
                LockedAxes::ROTATION_LOCKED,
                RigidBody::Static,
                Collider::rectangle(scale.x, scale.y),
                CollisionLayers::new(GameLayer::Border, [GameLayer::Player, GameLayer::GhostPlayer]),
                Transform {
                    translation: *position,
                    ..default()
                },
            )
        )
    }
    border_entities
}

pub fn spawn_broken_bridge_collision(
    map_location: Vec3
) -> impl Bundle {
    (
        Name::new("Broken Bridge Collsion"),
        BrokenBridgeCollider,
        LockedAxes::ROTATION_LOCKED,
        RigidBody::Static,
        Collider::rectangle(50., 200.),
        CollisionLayers::new(GameLayer::BrokenBridge, [
            GameLayer::Player,
        ]),
        Transform::from_translation(map_location + BROKEN_BRIDGE_COLLISION_LOCATION),
    )
}