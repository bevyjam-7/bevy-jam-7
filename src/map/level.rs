//! Spawn the main level.

use bevy::{image::{ImageLoaderSettings, ImageSampler}, mesh::RectangleMeshBuilder, prelude::*, render::render_resource::Texture};

use crate::{
    asset_tracking::LoadResource, audio::music, game_consts::{TELEPORTER_A_LOCATION, TELEPORTER_B_LOCATION, WITCH_HOUSE_ATLAS_COLS, WITCH_HOUSE_ATLAS_ROWS}, map::{animation::HouseAnimation, object::spawn_object, teleporter::{create_teleporter_pair, link_teleporters}}, player::{animation::FacingDirection, player::{PlayerAssets, player}}, screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
    app.load_resource::<MapAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    player_assets: Res<PlayerAssets>,
    map_assets: Res<MapAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let teleporter_position_a = TELEPORTER_A_LOCATION;
    let teleporter_position_b = TELEPORTER_B_LOCATION;

    let (teleporter_1, teleporter_2) = create_teleporter_pair(
        teleporter_position_a,
        teleporter_position_b,
        &mut meshes,
        &mut materials,
    );
    link_teleporters(
        &mut commands, 
        teleporter_position_a, 
        teleporter_position_b, 
        teleporter_1, 
        teleporter_2
    );
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![
            witch_house_map(&map_assets, &mut texture_atlas_layouts),
            player(100.0, &player_assets, &mut texture_atlas_layouts, FacingDirection::Down), // original speed was 100.0, 0.0 to see sprite alignment better
            spawn_object(
                crate::inventory::inventory::ItemKind::Food1,
                Vec3::new(0., 200., 0.),
                meshes.add(Rectangle::new(30., 20.)),
                materials.add(ColorMaterial::from(Color::BLACK)),
            ),
            (
                Name::new("Gameplay Music"),
                music(level_assets.music.clone())
            )
        ],
    ));
}

/// A square entity that will be the background of the level
pub fn witch_house_map(
    map_assets: &MapAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(256, 320), WITCH_HOUSE_ATLAS_COLS, WITCH_HOUSE_ATLAS_ROWS, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let house_animation = HouseAnimation::new();
    (
        Name::new("Witch House"),
        WitchHouse,
        Sprite::from_atlas_image(
            map_assets.witch_house.clone(), 
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            }
        ),
        // make map twice the normal size
        Transform::from_scale(Vec2::splat(2.0).extend(0.0)),
        house_animation
    )
}

pub fn bridge_section_map(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    let bridge_mesh = meshes.add(Rectangle::new(400., 100.));
    let bridge_material = materials.add(ColorMaterial::from(Color::hsv(0.,1.,1.)));
    (
        Name::new("Bridge Section"),
        Mesh2d(bridge_mesh),
        MeshMaterial2d(bridge_material),
    )
}


#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct WitchHouse;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct MapAssets {
    #[dependency]
    witch_house: Handle<Image>,
}

impl FromWorld for MapAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        MapAssets {
            witch_house: assets.load_with_settings(
                "images/witch_house.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                }
            ),
        }
    }
}