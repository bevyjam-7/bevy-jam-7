//! Handle player input and translate it into movement through a character
//! controller. A character controller is the collection of systems that govern
//! the movement of characters.
//!
//! In our case, the character controller has the following logic:
//! - Set [`MovementController`] intent based on directional keyboard input.
//!   This is done in the `player` module, as it is specific to the player
//!   character.
//! - Apply movement based on [`MovementController`] intent and maximum speed.
//! - Wrap the character within the window.
//!
//! Note that the implementation used here is limited for demonstration
//! purposes. If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs).

use bevy::{prelude::*, window::PrimaryWindow};
use leafwing_input_manager::prelude::*;

use crate::{AppSystems, PausableSystems, inventory::{inventory::{Inventory, ObjectPickable}, systems::handle_pickups}, map::teleporter::{self, Teleporter}, player::{PlayerState, player::Player, player_ghost::player_ghost::GhostPlayer, player::ActionTimer}};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (apply_movement, pick_up_item_action)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
    
    app.add_systems(Update, 
        apply_actions
        .chain()
        .in_set(AppSystems::RecordInput)
        .in_set(PausableSystems)
    );


}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Debug, Hash, Reflect)]
pub enum PlayerAction {
    #[actionlike(DualAxis)]
    Move,
    Honkshoo,
    Teleport,
    Pickup,
}

impl PlayerAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad mapping for movement
        input_map.insert_dual_axis(Self::Move, GamepadStick::LEFT);
        input_map.insert(Self::Honkshoo, GamepadButton::South);
        input_map.insert(Self::Teleport, GamepadButton::East);
        input_map.insert(Self::Pickup, GamepadButton::West);

        // Default keyboard mapping for movement
        input_map.insert_dual_axis(Self::Move, VirtualDPad::wasd());
        input_map.insert_dual_axis(Self::Move, VirtualDPad::arrow_keys());
        input_map.insert(Self::Honkshoo, KeyCode::Space);
        input_map.insert(Self::Teleport, KeyCode::KeyQ);
        input_map.insert(Self::Pickup, KeyCode::KeyE);
        
        input_map
    }

    
}

fn pick_up_item_action(
    mut commands: Commands,
    mut inventory: ResMut<Inventory>,
    player_query: Query<&Transform, With<Player>>,
    pickables: Query<(Entity, &GlobalTransform, &ObjectPickable)>,
    mut action_query: Query<&ActionState<PlayerAction>, With<Player>>,
) {
    if let Ok(action_state) = action_query.single() {
        if action_state.just_pressed(&PlayerAction::Pickup) {
            handle_pickups(commands, inventory, player_query, pickables);
        }
    }
}

fn apply_actions(
    player_state: ResMut<State<PlayerState>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    mut action_query: Query<(&ActionState<PlayerAction>, &mut MovementController, &mut ActionTimer), Or<(With<Player>, With<GhostPlayer>)>>,
    player_query: Query<&Transform, (With<Player>, Without<GhostPlayer>)>,
    ghost_query: Query<&Transform, (With<GhostPlayer>, Without<Player>)>,
    time: Res<Time>,
) {
    const WAKE_UP_DISTANCE: f32 = 50.0;

    for (action_state, mut controller, mut action_timer) in action_query.iter_mut() {
        // Set movement intent based on input actions, which are normalized to length 1.
        // If no input is pressed, this will be the zero vector.
        controller.intent = action_state.axis_pair(&PlayerAction::Move).normalize_or_zero();

        action_timer.timer.tick(time.delta());

        // Other actions can be handled here as well.
        if action_state.just_pressed(&PlayerAction::Honkshoo) && action_timer.timer.is_finished() {
            if player_state.get() == &PlayerState::Awake {
                next_player_state.set(PlayerState::Asleep);
                println!("\nAsleep state applied")
            } else if player_state.get() == &PlayerState::Asleep {
                if let (Ok(player_transform), Ok(ghost_transform)) = (player_query.single(), ghost_query.single()) {
                    let distance = player_transform.translation.distance(ghost_transform.translation);

                    if distance <= WAKE_UP_DISTANCE {
                        next_player_state.set(PlayerState::Awake);
                        print!("\nAwake state applied")
                    } else {
                        println!("\nToo far from ghost to wake up!");
                    }
                }
                
            }
            action_timer.timer.reset();
        }

        if action_state.just_pressed(&PlayerAction::Pickup) {
            info!("Pickup button pressed");
            // If player is on a pickable item, pick it up.
            // If player is already holding an item, drop it.

        }

        if action_state.just_pressed(&PlayerAction::Teleport) {
            info!("Teleport button pressed");
            // If player is holding an item, place the item down onto the teleporter.
            // If player is not holding an item and is on a teleporter, pick the item up.

            // fn place_item_on_tp() 

            // If player is in astral form, they are unable to pick up item
            // If in astral form, player is able to teleport the linked item.
            
        }
    }
}


fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&MovementController, &mut Transform)>,
) {
    for (controller, mut transform) in &mut movement_query {
        let velocity = controller.max_speed * controller.intent;
        transform.translation += velocity.extend(0.0) * time.delta_secs();
    }
}

/// These are the movement parameters for our character controller.
/// For now, this is only used for a single player, but it could power NPCs or
/// other players as well.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    /// The direction the character wants to move in.
    pub intent: Vec2,

    /// Maximum speed in world units per second.
    /// 1 world unit = 1 pixel when using the default 2D camera and no physics engine.
    pub max_speed: f32,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intent: Vec2::ZERO,
            // 400 pixels per second is a nice default, but we can still vary this per character.
            max_speed: 400.0,
        }
    }
}