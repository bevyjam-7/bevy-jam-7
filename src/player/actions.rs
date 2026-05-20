use avian2d::{dynamics::rigid_body::LinearVelocity, math::AdjustPrecision};
use bevy::{ecs::relationship::RelationshipSourceCollection, prelude::*};
use leafwing_input_manager::prelude::*;

use crate::{
    AppSystems, PausableSystems, game_consts::PLAYER_SPEED, game_gui::menus::Menu, map::{
        borders::BrokenBridgeCollider, events::{DialogueSelected, GameProgressTracker, SpawnRewardEvent}, interaction_box::{InteractableObject, ObjectInteractionType}, map_sections::MapAssets, npc::NpcInteractionBox, object::{Inventory, ItemKind, ObjectAssets, spawn_object}, teleporter::{self, Teleportable, Teleporter}
    }, player::{
        PlayerState,
        player::{ActionTimer, AwakePlayer, GhostPlayer},
    }
};

pub(super) fn plugin(app: &mut App) {

    app.add_systems(
        Update,
        (apply_state_switch, drop_item_action, interact_options, apply_movement)
            .chain()
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );

    app.add_systems(
        OnExit(Menu::Main),
        (force_awake_state, reset_action_timer)
        .chain()
    );
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

// Actions which are shared by both awake and ghost player
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Debug, Hash, Reflect)]
pub enum PlayerAction {
    #[actionlike(DualAxis)]
    Move,
    Honkshoo,
    Drop,
    Interact,
}

impl PlayerAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad mapping for movement
        input_map.insert_dual_axis(Self::Move, GamepadStick::LEFT);

        // Default gamepad mapping for interactions
        input_map.insert(Self::Honkshoo, GamepadButton::East);
        input_map.insert(Self::Drop, GamepadButton::DPadDown);
        input_map.insert(Self::Interact, GamepadButton::South);

        // Default keyboard mapping for movement
        input_map.insert_dual_axis(Self::Move, VirtualDPad::wasd());
        input_map.insert_dual_axis(Self::Move, VirtualDPad::arrow_keys());

        // Default keyboard mapping for interactions
        input_map.insert(Self::Honkshoo, KeyCode::Space);
        input_map.insert(Self::Drop, KeyCode::KeyR);
        input_map.insert(Self::Interact, KeyCode::KeyQ);

        input_map
    }
}

/// Both awake and ghost player types are able to perform a state switch
fn apply_state_switch(
    player_state: ResMut<State<PlayerState>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    mut action_query: Query<
        (
            &ActionState<PlayerAction>,
            &mut MovementController,
            &mut ActionTimer,
        ),
        Or<(With<AwakePlayer>, With<GhostPlayer>)>,
    >,
    player_query: Query<&GlobalTransform, (With<AwakePlayer>, Without<GhostPlayer>)>,
    ghost_query: Query<&GlobalTransform, (With<GhostPlayer>, Without<AwakePlayer>)>,
    time: Res<Time>,
) {
    const WAKE_UP_DISTANCE: f32 = 50.0;

    for (action_state, mut controller, mut action_timer) in action_query.iter_mut() {
        // Set movement intent based on input actions, which are normalized to length 1.
        // If no input is pressed, this will be the zero vector.
        controller.intent = action_state
            .clamped_axis_pair(&PlayerAction::Move)
            .xy()
            .normalize_or_zero();

        action_timer.timer.tick(time.delta());

        // State Swtiching with 0.5 cool down timer
        if action_state.just_pressed(&PlayerAction::Honkshoo) && action_timer.timer.is_finished() {
            if player_state.get() == &PlayerState::Awake {
                next_player_state.set(PlayerState::Asleep);
                println!("\nAsleep state applied")
            } else if player_state.get() == &PlayerState::Asleep {
                if let (Ok(player_transform), Ok(ghost_transform)) =
                    (player_query.single(), ghost_query.single())
                {
                    let distance = player_transform
                        .translation()
                        .distance(ghost_transform.translation());

                    if distance <= WAKE_UP_DISTANCE {
                        next_player_state.set(PlayerState::Awake);
                        info!("\nAwake state applied")
                    } else {
                        info!("\nToo far from ghost to wake up!");
                    }
                } else {
                    info!("\nGhost not found - waking up anyway");
                    next_player_state.set(PlayerState::Awake);
                }
            }
            action_timer.timer.reset();
        }
    }
}

fn interact_options(
    mut commands: Commands,
    action_query: Query<(&ActionState<PlayerAction>, &Transform), With<MovementController>>,
    object_interaction: Query<(Entity, &Transform, &InteractableObject)>,
    mut game_progress: Query<&mut GameProgressTracker>,
    mut inventory: ResMut<Inventory>,
    mut sprite_query: Query<&mut Sprite>,
    map_assets: Option<Res<MapAssets>>,
    player_state: ResMut<State<PlayerState>>,
    broken_bridge_collider: Query<Entity, With<BrokenBridgeCollider>>,
    teleportable_query: Query<&Teleportable>,
    mut teleporter_query: Query<&mut Teleporter>,
) {
    for (action_state, player_transform) in action_query {
        let player_pos = player_transform.translation.truncate();
        if action_state.just_pressed(&PlayerAction::Interact) {
            info!("Interaction key just pressed!");
            for (entity, transform, interaction_type) in object_interaction {
                let object_position = transform.translation.truncate();
                let distance_sq = player_pos.distance_squared(object_position);
                info!{"Entity {} pos is {}, player pos is {}, distance is {}.", entity, object_position, player_pos, distance_sq}
                // If the player is not close enough to the interaction object,
                // then this interaction type will not happen.
                if distance_sq > interaction_type.interaction_radius {
                    info!("Interaction too far out");
                    continue;
                }
                let player_state = player_state.get();
                match interaction_type.object_type {
                    // Interaction pick up
                    ObjectInteractionType::Pickable(kind) => {
                        info!("Interaction type is pickable.");
                        // Player cannot pick up items whilst asleep
                        if *player_state == PlayerState::Asleep {continue};
                        if let Ok(teleportable) = teleportable_query.get(entity) {
                            // Item is on a teleporter - unlink it first
                            if let Ok(mut teleporter) =
                                teleporter_query.get_mut(teleportable.on_teleporter_entity)
                            {
                                teleporter.containing_entity = Entity::PLACEHOLDER;
                                info!("\nCleared teleporter before pickup");
                            }
                            commands.entity(entity).remove::<Teleportable>();
                        }
                        commands.entity(entity).despawn();
                        let count = inventory.add(kind);
                        commands.trigger(DialogueSelected {
                            s: "AcquireFirstItem".to_string(),
                        });
                        info!(
                            " Picked up {} (total: {}) — inventory: {}",
                            kind,
                            count,
                            inventory.summary()
                        );
                    },
                    ObjectInteractionType::NPC => {
                        // let Ok(progress) = game_progress.single_mut() else {
                        //     return;
                        // };
                        commands.trigger(DialogueSelected {
                            s: "TalkNpc".to_string(),
                        });
                    },
                    ObjectInteractionType::Teleporter => {
                        if *player_state == PlayerState::Awake {continue};
                        info!("Interaction Teleporter");
                        // Collect teleportation data first (immutable pass)
                        let object_in_teleporter: Entity;
                        let teleporter_component: Teleporter;
                        // The entity is a teleport, so check if there is anything on the teleporter to teleport
                        if let Ok(mut teleporter) = teleporter_query.get_mut(entity) {
                            if teleporter.containing_entity.is_empty() {
                                continue;
                            }
                            teleporter_component = *teleporter;
                            object_in_teleporter = teleporter.containing_entity;

                            teleporter.containing_entity = Entity::PLACEHOLDER;
                        } else {
                            continue;
                        };
                        if let Ok(mut buddy_teleporter) = teleporter_query.get_mut(teleporter_component.tele_buddy) {
                            buddy_teleporter.containing_entity = object_in_teleporter;
                            commands.entity(object_in_teleporter).insert((
                                Teleportable {
                                    on_teleporter_entity: teleporter_component.tele_buddy,
                                },
                                Transform::from_translation(teleporter_component.destination),
                            ));
                        };
                    },
                    ObjectInteractionType::Bookshelf => {
                        
                    },
                    ObjectInteractionType::Bridge => {
                        if *player_state == PlayerState::Asleep {break};
                        if inventory.get(ItemKind::Object1).is_none() {
                            info!("You need a bridge piece to fix this!");
                            continue;
                        }
                        // Remove bridge piece from inventory
                        inventory.remove(ItemKind::Object1);
                        let bridge_collider = broken_bridge_collider.single_inner();
                        commands.entity(bridge_collider.unwrap()).despawn();
                        let Some(ref map_asset) = map_assets else {
                            continue;
                        };
                        let Ok(mut object_sprite) = sprite_query.get_mut(entity) else {
                            continue;
                        };
                        object_sprite.image = map_asset.fixed_bridge.clone();
                    },
                }
            }
        }
    }

}

fn drop_item_action(
    mut commands: Commands,
    mut inventory: ResMut<Inventory>,
    object_assets: Option<Res<ObjectAssets>>,
    mut teleporter_query: Query<(Entity, &mut Teleporter, &Transform, &InteractableObject)>,
    action_query: Query<(&ActionState<PlayerAction>, &Transform), With<AwakePlayer>>,
) {
    for (action_state, player_transform) in action_query {
        if action_state.just_pressed(&PlayerAction::Drop) {
            let player_pos = player_transform.translation;
            let Some(ref obj_assets) = object_assets else {
                continue;
            };
            let mut item: EntityCommands<'_>;
            if let Some(item_to_drop) = inventory.get(ItemKind::Food1) {
                let dropped_item = spawn_object(
                    item_to_drop,
                    ObjectInteractionType::Pickable(item_to_drop),
                    &obj_assets,
                    player_pos.clone() + Vec3::new(0., 0., 1.),
                );
                inventory.remove(item_to_drop);
                item = commands.spawn(dropped_item);
            } else if let Some(item_to_drop) = inventory.get(ItemKind::Object1) {
                let dropped_item = spawn_object(
                    item_to_drop,
                    ObjectInteractionType::Pickable(item_to_drop),
                    &obj_assets,
                    player_pos.clone() + Vec3::new(0., 0., 1.),
                );
                inventory.remove(item_to_drop);
                item = commands.spawn(dropped_item);
            } else {
                continue;
            }
            for (tp_entity, mut teleporter, teleporter_transform, tele_interact) in &mut teleporter_query {
                let object_position = teleporter_transform.translation.truncate();
                let distance_sq = player_pos.clone().truncate().distance_squared(object_position);
                // If the player is not close enough to the interaction object,
                // then this interaction type will not happen.
                if distance_sq > tele_interact.interaction_radius {
                    info!("object not placed on teleporter, {} away", distance_sq);
                    continue;
                }
                item.insert(Teleportable {
                    on_teleporter_entity: tp_entity
                });
                teleporter.containing_entity = item.id();
            }
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
fn force_awake_state(mut next_player_state: ResMut<NextState<PlayerState>>) {
    next_player_state.set(PlayerState::Awake);
    info!("\nForced player state to Awake on exit");
}

// resets action timer when exiting gameplay
fn reset_action_timer(mut timer_query: Query<&mut ActionTimer, With<AwakePlayer>>) {
    for mut action_timer in &mut timer_query {
        // Set the timer as finished by setting elapsed time to duration
        let duration = action_timer.timer.duration();
        action_timer.timer.set_elapsed(duration);
    }
}