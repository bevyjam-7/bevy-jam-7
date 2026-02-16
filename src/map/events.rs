use std::string;

use bevy::{ecs::system::command, math::ops::sqrt, prelude::*};
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

use crate::{PausableSystems, game_consts::{BRIDGE_SECTION_LOCATION, OLD_HOUSE_LOCATION, TELEPORTER_B_LOCATION, TELEPORTER_PROXIMITY_RADIUS, TRIPWIRE_BRIDGE_TO_HOUSE_POSITION, TRIPWIRE_BRIDGE_TO_OLD_POSITION, TRIPWIRE_HOUSE_TO_BRIDGE_POSITION, TRIPWIRE_OLD_TO_BRIDGE_POSITION, TRIPWIRE_PROXIMITY_RADIUS, WITCH_HOUSE_LOCATION}, inventory::inventory::{ItemKind, ObjectPickable}, map::{npc::NpcInteractionBox, teleporter::Teleportable}, player::{action::MovementController, player::Player, player_ghost::player_ghost::GhostPlayer}, screens::Screen};


pub(super) fn plugin(app: &mut App) {
    app.init_resource::<BreadOnTeleporterB>()
        .add_systems(
            Update, 
            (
                player_transition_between_sections, 
                interaction_box_detect_player, 
                check_bread_on_teleporter_b,
            ) 
            .chain()
            .in_set(PausableSystems),
        ).add_observer(run_dialogue).add_observer(spawn_reward_item);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapSection {
    WitchHouse,
    BridgeLeft,
    BridgeRight,
    OldHouse,
}

#[derive(Component, Debug)]
pub struct TranstionBetweenSections {
    pub next_section: MapSection,
    pub transition_radius: f32,
    pub can_transition: bool,
}

impl TranstionBetweenSections {
    pub fn new(next_section: MapSection) -> Self {
        Self {
            next_section,
            transition_radius: TRIPWIRE_PROXIMITY_RADIUS,
            can_transition: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect, Event)]
pub struct DialogueSelected{
    pub s: String,
}

pub fn run_dialogue(
    node: On<DialogueSelected>,
    mut commands: Commands, project: Res<YarnProject>
) {
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);

    dialogue_runner
        .commands_mut()
        .add_command("set_progress_bool", commands.register_system(set_progress_bool));
    dialogue_runner
        .library_mut()
        .add_function("get_progress_bool", commands.register_system(get_progress_bool));
    dialogue_runner.start_node(node.s.clone());
    commands.spawn(dialogue_runner);
}

// Everything dealing this ts is not pretty but it works, shield your eyes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect, Component)]
pub struct GameProgressTracker {
    pub first_entry_bridge: bool,
    pub first_entry_ghost: bool,
    pub first_entry_old_house: bool,
    pub first_talk_npc: bool,
    pub returned_home_after_talk_npc: bool,
    pub bread_in_old_house: bool,
    pub item_been_acquired: bool,
    pub teleporter_guide: bool,
}

impl Default for GameProgressTracker {
    fn default() -> Self {
        GameProgressTracker { 
            first_entry_bridge: false, 
            first_entry_ghost: false, 
            first_entry_old_house: false, 
            first_talk_npc: false, 
            returned_home_after_talk_npc: false, 
            bread_in_old_house: false, 
            item_been_acquired: false,
            teleporter_guide: false,
        }
    }
}
#[derive(Resource)]
struct YarnSpinnerSpecificCheck {
    int: u32,
}

pub fn get_progress_bool (
    In(num): In<(u32)>,
    mut game_progress: Query<&mut GameProgressTracker>
) -> bool {
    let Ok(progress) = game_progress.single_mut() else {
        return false;
    };
    match num {
        0 => progress.first_entry_bridge,
        1 => progress.first_entry_ghost,
        2 => progress.first_entry_old_house,
        3 => progress.first_talk_npc,
        4 => progress.returned_home_after_talk_npc,
        5 => progress.bread_in_old_house,
        6 => progress.item_been_acquired,
        7 => progress.teleporter_guide,
        _ => false,
    }
}

pub fn set_progress_bool (
    In(int): In<u32>,
    mut game_progress: Query<&mut GameProgressTracker>
) -> bool {
    let Ok(mut progress) = game_progress.single_mut() else {
        return false;
    };

    match int {
        0 => progress.first_entry_bridge = true,
        1 => progress.first_entry_ghost = true,
        2 => progress.first_entry_old_house = true,
        3 => progress.first_talk_npc = true,
        4 => progress.returned_home_after_talk_npc = true,
        5 => progress.bread_in_old_house = true,
        6 => progress.item_been_acquired = true,
        7 => progress.teleporter_guide = true,
        _ => return false,
    }
    true
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect, Event)]
pub struct SpawnRewardEvent;

// function to spawn the reward item
pub fn spawn_reward_item(
    _: On<SpawnRewardEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn a Bridge Piece
    let reward_mesh = meshes.add(Rectangle::new(40.0, 15.0)); // Bridge-like shape
    let reward_material = materials.add(ColorMaterial::from(Color::srgb(0.55, 0.27, 0.07))); // Brown/wood color
    
    commands.spawn((
        Name::new("Bridge Piece"),
        ObjectPickable::new(ItemKind::Object1),
        Transform::from_translation(TELEPORTER_B_LOCATION + OLD_HOUSE_LOCATION),
        GlobalTransform::default(),
        Mesh2d(reward_mesh),
        MeshMaterial2d(reward_material),
        DespawnOnExit(Screen::Gameplay), // Add if needed
    ));
    
    info!("Spawned Bridge Piece at position: {:?}", TELEPORTER_B_LOCATION + OLD_HOUSE_LOCATION);

}

// Dialogue when the game first starts.
pub fn start_dialogue(mut commands: Commands) { 
    commands.trigger(DialogueSelected { s: "WakeUp".to_string() });   
}

// A system that checks if the player is within a certain radius of a tripwire, and if so, teleports the player to the next section.
// Tripwires have a `TranstionBetweenSections` component which specifies which section they lead to and the radius within which they can be activated.
pub fn player_transition_between_sections(
    mut transition_query: Query<(&Transform, &mut TranstionBetweenSections), (Without<Camera>, Without<MovementController>)>,
    mut player_query: Query<&mut Transform, (With<MovementController>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<MovementController>)>,
    mut commands: Commands,
) {
    let Ok(mut player_transform) = player_query.single_mut() else {
        return;
    };

    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();

    for (tripwire_translation, mut transtion_section) in transition_query.iter_mut() {
        let tripwire_pos = tripwire_translation.translation.truncate();
        let distance_sq = player_pos.distance_squared(tripwire_pos);
        
        if distance_sq <= (transtion_section.transition_radius * transtion_section.transition_radius) {
            match transtion_section.next_section {
                MapSection::WitchHouse => {
                    player_transform.translation = TRIPWIRE_HOUSE_TO_BRIDGE_POSITION + Vec3::new(-50., 0., 0.);
                    camera_transform.translation = WITCH_HOUSE_LOCATION;
                    
                },
                MapSection::BridgeLeft => {
                    player_transform.translation = TRIPWIRE_BRIDGE_TO_HOUSE_POSITION + BRIDGE_SECTION_LOCATION + Vec3::new(50., 0., 0.);
                    camera_transform.translation = BRIDGE_SECTION_LOCATION;
                    commands.trigger(DialogueSelected { s: "BridgeLeft".to_string() });
                },
                MapSection::BridgeRight => {
                    player_transform.translation = TRIPWIRE_BRIDGE_TO_OLD_POSITION + BRIDGE_SECTION_LOCATION + Vec3::new(-50., 0., 0.);
                    camera_transform.translation = BRIDGE_SECTION_LOCATION;
                }
                MapSection::OldHouse => {
                    player_transform.translation = TRIPWIRE_OLD_TO_BRIDGE_POSITION + OLD_HOUSE_LOCATION + Vec3::new(50., 0., 0.);
                    camera_transform.translation = OLD_HOUSE_LOCATION;
                    commands.trigger(DialogueSelected { s: "OldHouse".to_string() });
                }
            }
            info!("Player transitioned to section: {:?}", transtion_section.next_section);
            transtion_section.can_transition = false;
        } else {
            transtion_section.can_transition = true;
        }
        
    }
}

// NPC related events / interactions

// checks if player is in range to interact with the npc
fn interaction_box_detect_player(
    player_query: Query<(Entity, &Transform), Or<(With<Player>, With<GhostPlayer>)>>,
    mut interaction_box_query: Query<(Entity, &GlobalTransform, &mut NpcInteractionBox)>,
) {
    for (_box_entity, box_transform, mut interaction_box) in interaction_box_query.iter_mut() {
        let box_pos = box_transform.translation().truncate();
        let box_size = interaction_box.size;
        let box_min = box_pos - box_size / 2.0;
        let box_max = box_pos + box_size / 2.0;

        let mut _player_in_range = false;
        
        for (_player_entity, player_transform) in player_query.iter() {
            let player_pos = player_transform.translation.truncate();
            
            if player_pos.x >= box_min.x && player_pos.x <= box_max.x
                && player_pos.y >= box_min.y && player_pos.y <= box_max.y
            {
                _player_in_range = true;
                info!("Player on interaction box, can interact");
                break;
            }
        }
        interaction_box.can_talk = _player_in_range;
    }
}

#[derive(Resource, Default)]
pub struct BreadOnTeleporterB {
    pub is_present: bool,
}

// finds and checks for specific item on teleporter b in old house
fn check_bread_on_teleporter_b(
    mut has_printed: Local<bool>,
    mut bread_state: ResMut<BreadOnTeleporterB>,
    item_query: Query<(&ObjectPickable, Option<&Teleportable>)>,
    teleporter_query: Query<(&Name, Entity)>,
    mut game_progress: Query<&mut GameProgressTracker>,
) {
    let Ok(mut progress) = game_progress.single_mut() else {
        return;
    };
    let teleporter_b_id = teleporter_query
        .iter()
        .find(|(name, _)| name.as_str() == "Teleporter B")
        .map(|(_, entity)| entity);
    
    let Some(tp_b_entity) = teleporter_b_id else {
        progress.bread_in_old_house = false;
        return;
    };
    
    let mut bread_on_teleporter = false;
    
    for (pickable, teleportable) in &item_query {
        if pickable.kind == ItemKind::Food1 {
            if let Some(tp) = teleportable {
                if tp.on_teleporter_entity == tp_b_entity {
                    if !*has_printed {
                        info!("\nBread is on teleporter B!");
                        *has_printed = true;
                    }
                    bread_on_teleporter = true;
                    break;
                }
            }
        }
    }
    
    progress.bread_in_old_house = bread_on_teleporter;
}
