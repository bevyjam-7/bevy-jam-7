//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    assets::{
        animation::{FacingDirection, StaticAnimation},
        asset_tracking::LoadResource,
    }, audio::music, game_consts::{
        BRIDGE_SECTION_LOCATION, OLD_HOUSE_LOCATION, TELEPORTER_A_LOCATION, TELEPORTER_B_LOCATION,
        TRIPWIRE_BRIDGE_TO_HOUSE_POSITION, TRIPWIRE_BRIDGE_TO_OLD_POSITION,
        TRIPWIRE_HOUSE_TO_BRIDGE_POSITION, TRIPWIRE_OLD_TO_BRIDGE_POSITION, WITCH_HOUSE_ATLAS_COLS,
        Y_BRIDGE_SECTION_BORDER_OFFSETS,
    }, game_gui::screens::Screen, map::{
        borders::{spawn_box_borders, spawn_broken_bridge_collision}, events::{GameProgressTracker, MapSection}, interaction_box::{InteractableObject, ObjectInteractionType}, map_sections::{
            BridgeSection, MapAssets, MapType, WitchHouse, spawn_map_layout,
            teleportation_tripwire_entity,
        }, npc::{NpcAssets, interaction_box_bundle, spawn_npc}, object::{ItemKind, ObjectAssets, spawn_object}, old_bookshelf, teleporter::{create_teleporter_pair, link_teleporters}
    }, player::{
        PlayerState,
        player::{AwakePlayer, PlayerAssets, player},
    }
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
    player_state: Res<State<PlayerState>>,
    level_assets: Res<LevelAssets>,
    player_assets: Res<PlayerAssets>,
    object_assets: Res<ObjectAssets>,
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
        teleporter_2,
    );

    let mut level_entity = commands.spawn((
        Name::new("Level"),
        Level,
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        GameProgressTracker::default(),
    ));

    let house_animation = StaticAnimation::new(WITCH_HOUSE_ATLAS_COLS as usize);

    level_entity.with_children(|children| {
        // Witch house map section
        children
            .spawn(spawn_map_layout(
                &map_assets,
                &mut texture_atlas_layouts,
                MapType::WitchMap,
            ))
            .insert((house_animation, WitchHouse));

        children.spawn(teleportation_tripwire_entity(
            TRIPWIRE_HOUSE_TO_BRIDGE_POSITION,
            MapSection::BridgeLeft,
            &mut meshes,
            &mut materials,
        ));

        children
            .spawn(player(
                &player_state,
                &player_assets,
                &mut texture_atlas_layouts,
                FacingDirection::Down,
            ))
            .insert(AwakePlayer);

        children.spawn(spawn_object(
            ItemKind::Food1,
            ObjectInteractionType::Pickable(ItemKind::Food1),
            &object_assets,
            Vec3::new(0., 200., 10.),
        ));

        for border in spawn_box_borders(Vec3::ZERO, Vec4::ZERO, Vec4::ZERO).into_iter() {
            println!("Spawning border from position: {:?}", border.6.translation);
            children.spawn((
                border.0, border.1, border.2, border.3, border.4, border.5, border.6,
            ));
        }

        // Bridge map section
        children
            .spawn(spawn_map_layout(
                &map_assets,
                &mut texture_atlas_layouts,
                MapType::BridgeMap,
            ))
                .insert((
                    BridgeSection,
                    InteractableObject {
                        object_type: ObjectInteractionType::Bridge,
                        interaction_radius: 40000.
                    }
                ));
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
        for border in spawn_box_borders(
            BRIDGE_SECTION_LOCATION,
            Vec4::ZERO,
            Y_BRIDGE_SECTION_BORDER_OFFSETS,
        )
        .into_iter()
        {
            println!("Spawning border from position: {:?}", border.6.translation);
            children.spawn((
                border.0, border.1, border.2, border.3, border.4, border.5, border.6,
            ));
        }

        // Old house map section
        children.spawn(spawn_map_layout(
            &map_assets,
            &mut texture_atlas_layouts,
            MapType::OldHouseMap,
        ));
        children
            .spawn(spawn_npc(&npc_assets, &mut texture_atlas_layouts))
            .with_children(|children| {
                children.spawn(interaction_box_bundle());
            });
        children.spawn(old_bookshelf::spawn_old_bookshelf());
        children.spawn(old_bookshelf::spawn_bookshelf_collision());
        children.spawn(teleportation_tripwire_entity(
            TRIPWIRE_OLD_TO_BRIDGE_POSITION + OLD_HOUSE_LOCATION,
            MapSection::BridgeRight,
            &mut meshes,
            &mut materials,
        ));
        for border in spawn_box_borders(OLD_HOUSE_LOCATION, Vec4::ZERO, Vec4::ZERO).into_iter() {
            println!("Spawning border from position: {:?}", border.6.translation);
            children.spawn((
                border.0, border.1, border.2, border.3, border.4, border.5, border.6,
            ));
        }

        children.spawn((
            Name::new("Gameplay Music"),
            music(level_assets.music.clone()),
        ));
    });
}
