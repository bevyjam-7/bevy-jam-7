use bevy::{ecs::system::command, math::ops::sqrt, prelude::*};
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

use crate::{AppSystems, PausableSystems, Pause, game_consts::{BRIDGE_SECTION_LOCATION, OLD_HOUSE_LOCATION, TELEPORTER_PROXIMITY_RADIUS, TRIPWIRE_BRIDGE_TO_HOUSE_POSITION, TRIPWIRE_BRIDGE_TO_OLD_POSITION, TRIPWIRE_HOUSE_TO_BRIDGE_POSITION, TRIPWIRE_OLD_TO_BRIDGE_POSITION, WITCH_HOUSE_LOCATION}, inventory::inventory::{ItemKind, ObjectPickable}, map::{npc::{NpcInteractionBox, PlayerOnInteractionBox}, teleporter::{Teleportable, Teleporter}}, player::{action::MovementController, player::Player, player_ghost::player_ghost::GhostPlayer}};
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
        );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapSection {
    WitchHouse,
    BridgeLeft,
    BridgeRight,
    OldHouse,
}


pub struct EventProgress {
    has_gone_bridge_section: bool,
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
            transition_radius: TELEPORTER_PROXIMITY_RADIUS,
            can_transition: false,
        }
    }
}

// Dialogue when the game first starts. The game is in pause state when dialogue is in action
pub fn start_dialogue(mut commands: Commands, project: Res<YarnProject>) { 
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);

    dialogue_runner.start_node("WakeUp");
    commands.spawn(dialogue_runner);
}

// A system that checks if the player is within a certain radius of a tripwire, and if so, teleports the player to the next section.
// Tripwires have a `TranstionBetweenSections` component which specifies which section they lead to and the radius within which they can be activated.
pub fn player_transition_between_sections(
    mut transition_query: Query<(&Transform, &mut TranstionBetweenSections), (Without<Camera>, Without<MovementController>)>,
    mut player_query: Query<&mut Transform, (With<MovementController>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<MovementController>)>,
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
                },
                MapSection::BridgeRight => {
                    player_transform.translation = TRIPWIRE_BRIDGE_TO_OLD_POSITION + BRIDGE_SECTION_LOCATION + Vec3::new(-50., 0., 0.);
                    camera_transform.translation = BRIDGE_SECTION_LOCATION;
                }
                MapSection::OldHouse => {
                    player_transform.translation = TRIPWIRE_OLD_TO_BRIDGE_POSITION + OLD_HOUSE_LOCATION + Vec3::new(50., 0., 0.);
                    camera_transform.translation = OLD_HOUSE_LOCATION;
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
) {
    let teleporter_b_id = teleporter_query
        .iter()
        .find(|(name, _)| name.as_str() == "Teleporter B")
        .map(|(_, entity)| entity);
    
    let Some(tp_b_entity) = teleporter_b_id else {
        bread_state.is_present = false;
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
    
    bread_state.is_present = bread_on_teleporter;
}
