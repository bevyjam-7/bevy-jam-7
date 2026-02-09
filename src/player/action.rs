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

use crate::{AppSystems, PausableSystems, player::player::Player};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        apply_movement
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
    
    app.add_systems(Update, 
        (apply_actions)
        .chain()
        .in_set(AppSystems::RecordInput)
        .in_set(PausableSystems)
    );
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Debug, Hash, Reflect)]
pub enum PlayerAction {
    #[actionlike(DualAxis)]
    Move,
}

impl PlayerAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad mapping for movement
        input_map.insert_dual_axis(Self::Move, GamepadStick::LEFT);

        // Default keyboard mapping for movement
        input_map.insert_dual_axis(Self::Move, VirtualDPad::wasd());
        input_map.insert_dual_axis(Self::Move, VirtualDPad::arrow_keys());
        
        input_map
    }

    
}

fn apply_actions(
    mut action_query: Query<(&ActionState<PlayerAction>, &mut MovementController), With<Player>>,
) {
    for (action_state, mut controller) in action_query.iter_mut() {
        // Set movement intent based on input actions, which are normalized to length 1.
        // If no input is pressed, this will be the zero vector.
        controller.intent = action_state.axis_pair(&PlayerAction::Move).normalize_or_zero();

        // Other actions can be handled here as well.
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