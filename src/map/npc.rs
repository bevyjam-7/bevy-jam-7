use bevy::{image::{ImageLoaderSettings, ImageSampler}, prelude::*};

use crate::{assets::{animation::StaticAnimation, asset_tracking::LoadResource}, game_consts::{NPC_ATLAS_COLS, NPC_ATLAS_ROWS, NPC_LOCATION, OLD_HOUSE_LOCATION}, player::action::MovementController};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<NpcAssets>();
}

pub fn spawn_npc(
    npc_assets: &NpcAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(128), NPC_ATLAS_COLS as u32, NPC_ATLAS_ROWS as u32, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let npc_animation = StaticAnimation::new(NPC_ATLAS_COLS);
    (
        Name::new("Npc"),   
        Npc,
        Sprite::from_atlas_image(
            npc_assets.npc.clone(), 
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        Transform {
            translation: OLD_HOUSE_LOCATION + NPC_LOCATION,
            scale: Vec2::splat(2.0).extend(1.0),
            ..default()
        },
        npc_animation,
    )
}

// Bundle for the interaction box under npc
pub fn interaction_box_bundle() -> impl Bundle {
    let box_size = Vec2::new(50.0, 32.0);
    (
        NpcInteractionBox {
            can_talk: false,
            size: box_size,
        },
        Sprite {
            color: Color::srgba(0.0, 1.0, 0.0, 0.5),
            custom_size: Some(box_size),
            ..default()
        },
        Transform::from_xyz(-43.0, -48.0, 0.0),
    )
}



#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Npc;

#[derive(Component)]
pub struct NpcInteractionBox {
    pub can_talk: bool,
    pub size: Vec2,
}

#[derive(Event)]
pub struct PlayerOnInteractionBox {
    pub player: Entity,
    pub box_entity: Entity,
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct NpcAssets {
    #[dependency]
    npc: Handle<Image>,
}

impl FromWorld for NpcAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            npc: assets.load_with_settings(
                "images/lord_sheet.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                }
            ),
        }
    }
}

