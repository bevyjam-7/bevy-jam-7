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

use avian2d::math::AdjustPrecision;
use avian2d::prelude::LinearVelocity;
use bevy::{prelude::*, window::PrimaryWindow};
use leafwing_input_manager::prelude::*;
use crate::game_consts::PLAYER_SPEED;
use crate::player::action;
use crate::screens::Screen::Gameplay;
use crate::{AppSystems, PausableSystems, inventory::{inventory::{Inventory, ObjectPickable}, systems::{drop_item, handle_pickups}}, map::teleporter::{self, Teleporter}, player::{PlayerState, player::{ActionTimer, Player}, player_ghost::player_ghost::{GhostPlayer, GhostPlayerAssets}}};
use crate::map::teleporter::Teleportable;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        apply_movement
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
    
    app.add_systems(Update, 
        (apply_state_switch, pick_up_item_action, drop_item_action, teleport_entity)
        .chain()
        .in_set(AppSystems::RecordInput)
        .in_set(PausableSystems)
    );

    app.add_systems(OnExit(Gameplay), (force_awake_state, reset_action_timer));
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Debug, Hash, Reflect)]
pub enum PlayerAction {
    #[actionlike(DualAxis)]
    Move,
    Honkshoo,
    Teleport,
    Pickup,
    Drop,
}

impl PlayerAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad mapping for movement
        input_map.insert_dual_axis(Self::Move, GamepadStick::LEFT);

        // Default gamepad mapping for interactions
        input_map.insert(Self::Honkshoo, GamepadButton::South);
        input_map.insert(Self::Teleport, GamepadButton::East);
        input_map.insert(Self::Pickup, GamepadButton::West);
        input_map.insert(Self::Drop, GamepadButton::North);

        // Default keyboard mapping for movement
        input_map.insert_dual_axis(Self::Move, VirtualDPad::wasd());
        input_map.insert_dual_axis(Self::Move, VirtualDPad::arrow_keys());

        // Default keyboard mapping for interactions
        input_map.insert(Self::Honkshoo, KeyCode::Space);
        input_map.insert(Self::Teleport, KeyCode::KeyQ);
        input_map.insert(Self::Pickup, KeyCode::KeyE);
        input_map.insert(Self::Drop, KeyCode::KeyF);
        
        input_map
    }

    
}

fn drop_item_action(
    commands: Commands,
    inventory: ResMut<Inventory>,
    player_query: Query<&Transform, With<Player>>,
    action_query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok(action_state) = action_query.single() {
        if action_state.just_pressed(&PlayerAction::Drop) {
            drop_item(commands, inventory, player_query, meshes, materials);
        }
    }
}

fn teleport_entity(
    mut commands: Commands,
    action_query: Query<&ActionState<PlayerAction>, With<GhostPlayer>>,
    teleportable_query: Query<(Entity, &Teleportable)>,
    mut teleporter_query: Query<&mut Teleporter>,
) {
    let Ok(action_state) = action_query.single() else {
        return;
    };
    
    if !action_state.just_pressed(&PlayerAction::Teleport) {
        return;
    }
    
    // Collect teleportation data first (immutable pass)
    let mut teleports_to_perform = Vec::new();
    
    for (item_entity, teleportable) in &teleportable_query {
        let Ok(teleporter) = teleporter_query.get(teleportable.on_teleporter_entity) else {
            continue;
        };
        
        if !teleporter.can_teleport {
            continue;
        }
        
        let Ok(buddy_teleporter) = teleporter_query.get(teleporter.tele_buddy) else {
            continue;
        };
        
        if buddy_teleporter.containing_entity != Entity::PLACEHOLDER {
            info!("Buddy teleporter occupied by entity {:?}, cannot teleport", buddy_teleporter.containing_entity);
            continue;
        }
        
        // Store the data we need for teleportation
        teleports_to_perform.push((
            item_entity,
            teleportable.on_teleporter_entity,
            teleporter.tele_buddy,
            teleporter.destination,
        ));
    }
    
    // Now perform the teleportations (mutable pass)
    for (item_entity, source_tp, buddy_tp, destination) in teleports_to_perform {
        // Clear source teleporter
        if let Ok(mut teleporter) = teleporter_query.get_mut(source_tp) {
            teleporter.containing_entity = Entity::PLACEHOLDER;
        }
        
        // Set buddy teleporter
        if let Ok(mut buddy) = teleporter_query.get_mut(buddy_tp) {
            buddy.containing_entity = item_entity;
        }
        
        // Update item
        commands.entity(item_entity).insert(Teleportable { 
            on_teleporter_entity: buddy_tp 
        });
        commands.entity(item_entity).insert(Transform::from_translation(destination));
        
        info!("Teleported item {:?} to {:?}", item_entity, destination);
    }
}

fn pick_up_item_action(
    mut commands: Commands,
    inventory: ResMut<Inventory>,
    player_query: Query<&Transform, With<Player>>,
    pickables: Query<(Entity, &GlobalTransform, &ObjectPickable)>,
    action_query: Query<&ActionState<PlayerAction>, With<Player>>,
    teleportable_query: Query<&Teleportable>,
    mut teleporter_query: Query<&mut Teleporter>,
) {
    if let Ok(action_state) = action_query.single() {
        if action_state.just_pressed(&PlayerAction::Pickup) {
            // Before picking up, check if item is on a teleporter
            for (entity, _, _) in &pickables {
                if let Ok(teleportable) = teleportable_query.get(entity) {
                    // Item is on a teleporter - unlink it first
                    if let Ok(mut teleporter) = teleporter_query.get_mut(teleportable.on_teleporter_entity) {
                        teleporter.containing_entity = Entity::PLACEHOLDER;
                        info!("Cleared teleporter before pickup");
                    }
                    commands.entity(entity).remove::<Teleportable>();
                }
            }
            
            handle_pickups(commands, inventory, player_query, pickables);
        }
    }
}

fn apply_state_switch(
    player_state: ResMut<State<PlayerState>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    mut action_query: Query<(&ActionState<PlayerAction>, &mut MovementController, &mut ActionTimer), Or<(With<Player>, With<GhostPlayer>)>>,
    player_query: Query<&GlobalTransform, (With<Player>, Without<GhostPlayer>)>,
    ghost_query: Query<&GlobalTransform, (With<GhostPlayer>, Without<Player>)>,
    time: Res<Time>,
) {
    const WAKE_UP_DISTANCE: f32 = 50.0;

    for (action_state, mut controller, mut action_timer) in action_query.iter_mut() {
        // Set movement intent based on input actions, which are normalized to length 1.
        // If no input is pressed, this will be the zero vector.
        controller.intent = action_state.clamped_axis_pair(&PlayerAction::Move).xy().normalize_or_zero();

        action_timer.timer.tick(time.delta());

        // State Swtiching with 0.5 cool down timer
        if action_state.just_pressed(&PlayerAction::Honkshoo) && action_timer.timer.is_finished() {
            if player_state.get() == &PlayerState::Awake {
                next_player_state.set(PlayerState::Asleep);
                println!("\nAsleep state applied")
            } else if player_state.get() == &PlayerState::Asleep {
                if let (Ok(player_transform), Ok(ghost_transform)) = (player_query.single(), ghost_query.single()) {
                    let distance = player_transform.translation().distance(ghost_transform.translation());

                    if distance <= WAKE_UP_DISTANCE {
                        next_player_state.set(PlayerState::Awake);
                        info!("\nAwake state applied")
                    } else {
                        info!("\nToo far from ghost to wake up!");
                    }
                }
                else {
                    info!("\nGhost not found - waking up anyway");
                    next_player_state.set(PlayerState::Awake);
                }
            }
            action_timer.timer.reset();
        }
    }
}


fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&mut LinearVelocity, &MovementController)>,
) {
    for (mut velocity, controller) in &mut movement_query {
        
        let delta_time = time.delta_secs_f64().adjust_precision();

        velocity.x = controller.intent.x * controller.max_speed * delta_time;
        velocity.y = controller.intent.y * controller.max_speed * delta_time;
    }
}

// Force player to Awake state when exiting gameplay
fn force_awake_state(
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    next_player_state.set(PlayerState::Awake);
    info!("\nForced player state to Awake on exit");
}

// resets action timer when exiting gameplay
fn reset_action_timer(
    mut timer_query: Query<&mut ActionTimer, With<Player>>,
) {
    for mut action_timer in &mut timer_query {
        // Set the timer as finished by setting elapsed time to duration
        let duration = action_timer.timer.duration();
        action_timer.timer.set_elapsed(duration);
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
            max_speed: PLAYER_SPEED,
        }
    }
}