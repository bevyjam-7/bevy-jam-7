use bevy::{ecs::system::command, math::ops::sqrt, prelude::*};
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

use crate::{AppSystems, PausableSystems, game_consts::{BRIDGE_SECTION_LOCATION, TELEPORTER_PROXIMITY_RADIUS, TRIPWIRE_BRIDGE_TO_HOUSE_POSITION, TRIPWIRE_HOUSE_TO_BRIDGE_POSITION, WITCH_HOUSE_LOCATION}, player::{player::Player, player_ghost::player_ghost::GhostPlayer}};
pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update, 
        player_transition_between_sections
            .chain()
            .in_set(PausableSystems),
    );

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapSection {
    WitchHouse,
    Bridge,
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
    mut transition_query: Query<(&Transform, &mut TranstionBetweenSections), (Without<Player>, Without<Camera>, Without<GhostPlayer>)>,
    mut player_query: Query<&mut Transform, (Or<(With<Player>, With<GhostPlayer>)>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>, Without<GhostPlayer>)>,
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
        
        if distance_sq <= sqrt(transtion_section.transition_radius) {
            match transtion_section.next_section {
                MapSection::WitchHouse => {
                    player_transform.translation = TRIPWIRE_HOUSE_TO_BRIDGE_POSITION + Vec3::new(-50., 0., 0.);
                    camera_transform.translation = WITCH_HOUSE_LOCATION;
                },
                MapSection::Bridge => {
                    player_transform.translation = TRIPWIRE_BRIDGE_TO_HOUSE_POSITION + BRIDGE_SECTION_LOCATION + Vec3::new(50., 0., 0.);
                    camera_transform.translation = BRIDGE_SECTION_LOCATION;
                },
            }
            println!("Player transitioned to section: {:?}", transtion_section.next_section);
            transtion_section.can_transition = false;
        } else {
            transtion_section.can_transition = true;
        }
        
    }
}