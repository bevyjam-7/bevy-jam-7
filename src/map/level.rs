//! Spawn the main level.

use bevy::{image::{ImageLoaderSettings, ImageSampler}, mesh::RectangleMeshBuilder, prelude::*, render::render_resource::Texture};

use crate::{
    asset_tracking::LoadResource, audio::music, game_consts::{BRIDGE_SECTION_LOCATION, OLD_HOUSE_LOCATION, SCENE_TILE_SIZES, TELEPORTER_A_LOCATION, TELEPORTER_B_LOCATION, TRIPWIRE_BRIDGE_TO_HOUSE_POSITION, TRIPWIRE_BRIDGE_TO_OLD_POSITION, TRIPWIRE_HOUSE_TO_BRIDGE_POSITION, TRIPWIRE_OLD_TO_BRIDGE_POSITION, WITCH_HOUSE_ATLAS_COLS, WITCH_HOUSE_ATLAS_ROWS, Y_BRIDGE_SECTION_BORDER_OFFSETS}, map::{animation::HouseAnimation, borders::{spawn_box_borders, spawn_broken_bridge_collision}, events::{MapSection, TranstionBetweenSections}, npc::{NpcAssets, spawn_npc}, object::spawn_object, teleporter::{create_teleporter_pair, link_teleporters}}, player::{animation::FacingDirection, player::{PlayerAssets, player}}, screens::Screen,
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

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Level;

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
    npc_assets: Res<NpcAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    let (teleporter_1, teleporter_2) = create_teleporter_pair(
        TELEPORTER_A_LOCATION,
        TELEPORTER_B_LOCATION + OLD_HOUSE_LOCATION,
        &mut meshes,
        &mut materials,
    );
    link_teleporters(
        &mut commands, 
        TELEPORTER_A_LOCATION, 
        TELEPORTER_B_LOCATION + OLD_HOUSE_LOCATION, 
        teleporter_1, 
        teleporter_2
    );
    
    let mut level_entity = commands.spawn((
        Name::new("Level"),
        Level,
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
    ));
    
    level_entity.with_children(|children| {
        // Witch house map section
        children.spawn(witch_house_map(&map_assets, &mut texture_atlas_layouts));
        children.spawn(teleportation_tripwire_entity(
            TRIPWIRE_HOUSE_TO_BRIDGE_POSITION,
            MapSection::BridgeLeft,
            &mut meshes,
            &mut materials,
        ));
        children.spawn(player(5000.0, &player_assets, &mut texture_atlas_layouts, FacingDirection::Down));
        children.spawn(spawn_object(
            crate::inventory::inventory::ItemKind::Food1,
            Vec3::new(0., 200., 10.),
            meshes.add(Rectangle::new(30., 20.)),
            materials.add(ColorMaterial::from(Color::hsv(0.,1.,1.))),
        ));

        for border in spawn_box_borders(Vec3::ZERO, Vec4::ZERO, Vec4::ZERO).into_iter() {
            println!("Spawning border from position: {:?}", border.6.translation);
            children.spawn((
                border.0,
                border.1,
                border.2,
                border.3,
                border.4,
                border.5,
                border.6,
            ));
        }

        // Bridge map section
        children.spawn(bridge_section_map(&mut meshes, &mut materials));
        children.spawn(teleportation_tripwire_entity(
            TRIPWIRE_BRIDGE_TO_HOUSE_POSITION + BRIDGE_SECTION_LOCATION,
            MapSection::WitchHouse,
            &mut meshes,
            &mut materials,
        ));
        children.spawn(teleportation_tripwire_entity(
            TRIPWIRE_BRIDGE_TO_OLD_POSITION + BRIDGE_SECTION_LOCATION,
            MapSection::OldHouse,
            &mut meshes,
            &mut materials,
        ));
        children.spawn(spawn_broken_bridge_collision(BRIDGE_SECTION_LOCATION));
        for border in spawn_box_borders(BRIDGE_SECTION_LOCATION, Vec4::ZERO, Y_BRIDGE_SECTION_BORDER_OFFSETS).into_iter() {
            println!("Spawning border from position: {:?}", border.6.translation);
            children.spawn((
                border.0,
                border.1,
                border.2,
                border.3,
                border.4,
                border.5,
                border.6,
            ));
        }

        // Old house map section
        children.spawn(old_house_map(&mut meshes, &mut materials));
        children.spawn(spawn_npc(&npc_assets, &mut texture_atlas_layouts));
        children.spawn(teleportation_tripwire_entity(
            TRIPWIRE_OLD_TO_BRIDGE_POSITION,
            MapSection::BridgeRight,
            &mut meshes, 
            &mut materials
        ));
        for border in spawn_box_borders(OLD_HOUSE_LOCATION, Vec4::ZERO, Vec4::ZERO).into_iter() {
            println!("Spawning border from position: {:?}", border.6.translation);
            children.spawn((
                border.0,
                border.1,
                border.2,
                border.3,
                border.4,
                border.5,
                border.6,
            ));
        }



        children.spawn((
            Name::new("Gameplay Music"),
            music(level_assets.music.clone())
        ));
    });
}

/// A square entity that will be the background of the level
pub fn witch_house_map(
    map_assets: &MapAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let layout = TextureAtlasLayout::from_grid(SCENE_TILE_SIZES, WITCH_HOUSE_ATLAS_COLS, WITCH_HOUSE_ATLAS_ROWS, None, None);
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
        Transform {
            // make map twice the normal size
            scale: Vec2::splat(2.0).extend(0.0),
            translation: Vec3::new(0., 0., -10.),
            ..default()
        },
        house_animation
    )
}

pub fn bridge_section_map(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    let bridge_mesh = meshes.add(Rectangle::new(SCENE_TILE_SIZES.x as f32, SCENE_TILE_SIZES.y as f32));
    let bridge_material = materials.add(ColorMaterial::from(Color::hsv(0.,1.,1.)));
    (
        Name::new("Bridge Section"),
        Mesh2d(bridge_mesh),
        MeshMaterial2d(bridge_material),
        Transform {
            // make map twice the normal size
            scale: Vec2::splat(2.0).extend(0.0),
            translation: BRIDGE_SECTION_LOCATION,
            ..default()
        }
    )
}

pub fn old_house_map(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    let house_mesh = meshes.add(Rectangle::new(SCENE_TILE_SIZES.x as f32, SCENE_TILE_SIZES.y as f32));
    let house_material = materials.add(ColorMaterial::from(Color::hsv(0.35,1.,0.6)));
    (
        Name::new("Old House"),
        Mesh2d(house_mesh),
        MeshMaterial2d(house_material),
        Transform {
            // make map twice the normal size
            scale: Vec2::splat(2.0).extend(0.0),
            translation: OLD_HOUSE_LOCATION,
            ..default()
        }
    )
}

pub fn teleportation_tripwire_entity(
    position: Vec3,
    next_section: MapSection,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    let tripwire_mesh = meshes.add(Rectangle::new(32., 32.));
    let tripwire_material = materials.add(ColorMaterial::from(Color::hsv(0.,1.,0.5)));
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