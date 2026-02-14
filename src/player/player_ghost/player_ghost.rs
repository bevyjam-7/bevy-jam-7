// Ghost_Player behavior.

use avian2d::prelude::{Collider, CollisionLayers, LinearVelocity, LockedAxes, RigidBody};
use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*, sprite::Anchor,
};
use crate::{
    AppSystems, PausableSystems, asset_tracking::LoadResource, map::physics::GameLayer, player::{action::{MovementController, PlayerAction}, player::ActionTimer}
};
use super::player_ghost_animation::GhostPlayerAnimation;

pub (super) fn plugin(app: &mut App) {
    app.load_resource::<GhostPlayerAssets>();
}

pub fn ghost_player(
    max_speed: f32,
    ghost_player_assets: Res<GhostPlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) -> impl Bundle {
    // Similar to the player spawn function, but with ghost-specific assets and properties
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 4, 1, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let ghost_player_animation = GhostPlayerAnimation::new();

    (
        Name::new("Ghost Player"),
        GhostPlayer,
        Sprite::from_atlas_image(
            ghost_player_assets.ghost_ducky.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index:0,
            },
            
        ),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::rectangle(30., 20.),
        CollisionLayers::new(
            GameLayer::GhostPlayer,
            [
                GameLayer::Default,
                GameLayer::Border,
            ],
        ),
        LinearVelocity::default(),
        Anchor::BOTTOM_CENTER,
        Transform::from_scale(Vec2::splat(2.0).extend(1.0)),
        PlayerAction::default_input_map(),
        ActionTimer {
            timer: Timer::from_seconds(0.5, TimerMode::Once)
        },
        MovementController {
            max_speed,
            ..default()
        },
        ghost_player_animation,
        // Additional components for ghost player behavior can be added here
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct GhostPlayer;


#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct GhostPlayerAssets {
    #[dependency]
    pub ghost_ducky: Handle<Image>,
    #[dependency]
    pub ghost_ducky_float_audio: Vec<Handle<AudioSource>>,
}

impl FromWorld for GhostPlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        GhostPlayerAssets {
            ghost_ducky: asset_server.load_with_settings("images/Witch(Asleep)-Sheet.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            ghost_ducky_float_audio: vec![
                // using the same step audio for the ghost player for demonstration purposes, but you can replace these with ghost-specific audio if desired
                asset_server.load("audio/sound_effects/step1.ogg"),
                asset_server.load("audio/sound_effects/step2.ogg"),
                asset_server.load("audio/sound_effects/step3.ogg"),
                asset_server.load("audio/sound_effects/step4.ogg"),
            ],
            
        }
    }
}

