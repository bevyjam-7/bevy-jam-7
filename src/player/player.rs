//! Player-specific behavior.

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*, sprite::Anchor,
};

use leafwing_input_manager::prelude::*;

use crate::{
    assets::{animation::{FacingDirection, PlayerAnimation}, asset_tracking::LoadResource}, game_consts::PLAYER_SPEED, map::physics::GameLayer, player::{PlayerState, action::{MovementController, PlayerAction, TouchingBrokenBridge}}
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();
    app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
}

use avian2d::prelude::*;

/// The player character. Be sure to change state to asleep before spawning new player, or else will error
pub fn player(
    player_state: &Res<State<PlayerState>>,
    player_assets: &Res<PlayerAssets>,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    facing: FacingDirection,
) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout= TextureAtlasLayout::from_grid(UVec2::splat(64), 4, 3, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new(facing);

    let name: &str;
    match player_state.get() {
        PlayerState::Awake => name = "AwakePlayer",
        PlayerState::Asleep => name = "AsleepPlayer",
    }

    (
        Name::new(name),
        TouchingBrokenBridge::default(),
        Sprite::from_atlas_image(
            match player_state.get() {
                PlayerState::Awake => player_assets.player_awake.clone(),
                PlayerState::Asleep => player_assets.player_ghost.clone(),
            },
            TextureAtlas {
                layout: texture_atlas_layout,
                index:0,
            },
        ),
        Transform::from_scale(Vec2::splat(2.0).extend(1.0)),
        PlayerAction::default_input_map(),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::rectangle(30., 20.),
        match player_state.get() {
            PlayerState::Awake => CollisionLayers::new(
                GameLayer::Player,
                [
                    GameLayer::Default,
                    GameLayer::Border,
                    GameLayer::BrokenBridge,
                    GameLayer::OldBookshelf,
                ],
            ),
            PlayerState::Asleep => CollisionLayers::new(
                GameLayer::GhostPlayer,
                [
                    GameLayer::Default,
                    GameLayer::Border,
                ],
            ),
        },

        LinearVelocity::default(),
        ActionTimer {
            timer: Timer::from_seconds(0.5, TimerMode::Once)
        },
        Anchor::BOTTOM_CENTER,
        MovementController {
            max_speed: PLAYER_SPEED,
            ..default()
        },
        player_animation,
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct AwakePlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct GhostPlayer;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    player_awake: Handle<Image>,
    #[dependency]
    player_ghost: Handle<Image>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

#[derive(Component)]
pub struct ActionTimer{
    pub timer: Timer
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            player_awake: assets.load_with_settings(
                "images/Witch(Awake)-Sheet.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            player_ghost: assets.load_with_settings(
                "images/Witch(Asleep)-Sheet.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
        }
    }
}

