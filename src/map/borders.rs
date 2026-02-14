use bevy::{ecs::event::Trigger, prelude::*};
use avian2d::prelude::*;

use crate::{game_consts::{BORDER_THICKNESS, SCENE_TILE_SIZES}, map::{self, physics::GameLayer}};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Border;

pub fn spawn_box_borders(map_location: Vec3) -> Vec<(Name, Border, LockedAxes, RigidBody, Collider, CollisionLayers, Transform)> {

    let borders = [
        // Right border
        (
            Vec3::new(map_location.x + SCENE_TILE_SIZES.x as f32 + BORDER_THICKNESS / 2., 0., 0.),
            Vec2::new(BORDER_THICKNESS, map_location.y + SCENE_TILE_SIZES.y as f32 * 2.),
        ),
        // Left border
        (
            Vec3::new(map_location.x - SCENE_TILE_SIZES.x as f32 - BORDER_THICKNESS / 2., 0., 0.),
            Vec2::new(BORDER_THICKNESS, map_location.y + SCENE_TILE_SIZES.y as f32 * 2.),
        ),
        // Top border
        (
            Vec3::new(0., map_location.y + SCENE_TILE_SIZES.y as f32 + BORDER_THICKNESS / 2., 0.),
            Vec2::new(map_location.x + SCENE_TILE_SIZES.x as f32 * 2. + BORDER_THICKNESS * 2., BORDER_THICKNESS),
        ),
        // Bottom border
        (
            Vec3::new(0., map_location.y - SCENE_TILE_SIZES.y as f32 - BORDER_THICKNESS / 2., 0.),
            Vec2::new(map_location.x + SCENE_TILE_SIZES.x as f32 * 2. + BORDER_THICKNESS * 2., BORDER_THICKNESS),
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
                CollisionLayers::new(GameLayer::Border, [GameLayer::Player]),
                Transform {
                    translation: *position,
                    ..default()
                },
            )
        )
    }
    border_entities
}