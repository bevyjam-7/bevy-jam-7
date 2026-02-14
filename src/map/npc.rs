use bevy::{image::{ImageLoaderSettings, ImageSampler}, prelude::*};

use crate::{asset_tracking::LoadResource, game_consts::{NPC_ATLAS_COLS, NPC_ATLAS_ROWS, NPC_LOCATION, OLD_HOUSE_LOCATION}, map::animation::NpcAnimation, player::action::MovementController};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<NpcAssets>();
}

pub fn spawn_npc(
    npc_assets: &NpcAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(128), NPC_ATLAS_COLS as u32, NPC_ATLAS_ROWS as u32, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let npc_animation = NpcAnimation::new();
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


#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Npc;

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