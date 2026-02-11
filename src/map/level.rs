//! Spawn the main level.

use bevy::{mesh::RectangleMeshBuilder, prelude::*};

use crate::{
    asset_tracking::LoadResource, audio::music, game_consts::{TELEPORTER_A_LOCATION, TELEPORTER_B_LOCATION}, inventory::inventory::ItemKind, map::{object::spawn_object, teleporter::{create_teleporter_pair, link_teleporters}}, player::{animation::FacingDirection, player::{PlayerAssets, player}}, screens::Screen
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
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
            player(100.0, &player_assets, &mut texture_atlas_layouts, FacingDirection::Down), // original speed was 100.0, 0.0 to see sprite alignment better
            witch_house_map(&mut meshes, &mut materials),
            spawn_object(
                ItemKind::Food1, // Bread
                Vec3::new(0., 100., 1.), 
                meshes.add(Rectangle::new(30., 20.)), 
                materials.add(ColorMaterial::from(Color::BLACK))
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
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    let house_mesh = meshes.add(Rectangle::new(800., 400.));
    let house_material = materials.add(ColorMaterial::from(Color::WHITE));
    (
        Name::new("Witch House"),
        WitchHouse,
        Mesh2d(house_mesh),
        MeshMaterial2d(house_material),
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct WitchHouse;