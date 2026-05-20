use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{
    assets::asset_tracking::LoadResource,
    game_consts::*,
    map::events::{MapSection, TranstionBetweenSections},
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<MapAssets>();
}
pub enum MapType {
    WitchMap,
    BridgeMap,
    OldHouseMap,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct WitchHouse;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct BridgeSection;

pub fn spawn_map_layout(
    map_assets: &MapAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    map_type: MapType,
) -> impl Bundle {
    let layout = TextureAtlasLayout::from_grid(
        SCENE_TILE_SIZES,
        WITCH_HOUSE_ATLAS_COLS,
        WITCH_HOUSE_ATLAS_ROWS,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    (match map_type {
        MapType::WitchMap => (
            Name::new("Witch House"),
            Sprite::from_atlas_image(
                map_assets.witch_house.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                },
            ),
            Transform {
                // make map twice the normal size
                scale: Vec2::splat(2.0).extend(0.0),
                translation: Vec3::new(0., 0., -10.),
                ..default()
            },
        ),
        MapType::BridgeMap => (
            Name::new("Bridge Section"),
            Sprite::from_image(map_assets.broken_bridge.clone()),
            Transform {
                // make map twice the normal size
                scale: Vec2::splat(2.0).extend(-10.0),
                translation: BRIDGE_SECTION_LOCATION,
                ..default()
            },
        ),
        MapType::OldHouseMap => (
            Name::new("Old House"),
            Sprite::from_image(map_assets.old_house.clone()),
            Transform {
                // make map twice the normal size
                scale: Vec2::splat(2.0).extend(-10.0),
                translation: OLD_HOUSE_LOCATION,
                ..default()
            },
        ),
    },)
}

pub fn teleportation_tripwire_entity(
    position: Vec3,
    next_section: MapSection,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    let tripwire_mesh = meshes.add(Rectangle::new(32., 32.));
    let tripwire_material = materials.add(ColorMaterial::from(Color::BLACK));
    (
        Name::new("Teleporter Tripwire"),
        Mesh2d(tripwire_mesh),
        MeshMaterial2d(tripwire_material),
        Transform {
            translation: position,
            ..default()
        },
        TranstionBetweenSections::new(next_section),
    )
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct MapAssets {
    #[dependency]
    pub witch_house: Handle<Image>,
    #[dependency]
    pub broken_bridge: Handle<Image>,
    #[dependency]
    pub fixed_bridge: Handle<Image>,
    #[dependency]
    pub old_house: Handle<Image>,
}

impl FromWorld for MapAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        MapAssets {
            witch_house: assets.load_with_settings(
                "images/witch_house.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            broken_bridge: assets.load_with_settings(
                "images/broken_bridge.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            fixed_bridge: assets.load_with_settings(
                "images/fixed_bridge.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            old_house: assets.load_with_settings(
                "images/old_house.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}
