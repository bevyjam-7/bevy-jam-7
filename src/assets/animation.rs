use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::{
    AppSystems, PausableSystems,
    audio::sound_effect,
    map::npc::Npc,
    player::{PlayerState, action::MovementController, player::PlayerAssets},
};

pub(super) fn plugin(app: &mut App) {
    // Animate and play sound effects based on controls.
    app.add_systems(
        Update,
        (
            update_animation_timer.in_set(AppSystems::TickTimers),
            (
                update_animation_movement,
                update_animation_atlas,
                trigger_step_sound_effect,
            )
                .chain()
                .in_set(AppSystems::Update),
        )
            .in_set(PausableSystems),
    );

    app.add_systems(
        Update,
        (
            update_static_animation_timer.in_set(AppSystems::TickTimers),
            (update_static_animation_atlas, gaze_follow_player)
                .chain()
                .in_set(AppSystems::Update),
        )
            .in_set(PausableSystems),
    );
}

// Player and Ghost animations are very similar, with them sharing basically everything.

/// Component that tracks player's animation state.
/// It is tightly bound to the texture atlas we use.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerAnimation {
    timer: Timer,
    frame: usize,
    state: PlayerAnimationState,
    facing: FacingDirection,
    state_changed: bool,
}

#[derive(Reflect, PartialEq, Component)]
pub enum PlayerAnimationState {
    Idling,
    WalkingSide,
    WalkingUp,
    WalkingDown,
}

#[derive(Reflect, PartialEq, Clone, Copy)]
pub enum FacingDirection {
    Side,
    Up,
    Down,
}

/// Update the animation timer.
fn update_animation_timer(
    time: Res<Time>,
    mut query: Query<&mut PlayerAnimation, With<MovementController>>,
) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Update the sprite direction and animation state (idling/walking).
/// Player has 4 directions of movement, but Ghost only has 2
/// So we take in the player's state to figure out how we should be moving.
fn update_animation_movement(
    player_state: Res<State<PlayerState>>,
    mut player_query: Query<(&MovementController, &mut Sprite, &mut PlayerAnimation)>,
) {
    for (controller, mut sprite, mut animation) in &mut player_query {
        let dx = controller.intent.x;
        if dx != 0.0 {
            sprite.flip_x = dx < 0.0;
        }

        let animation_state = if controller.intent == Vec2::ZERO {
            PlayerAnimationState::Idling
        } else {
            // If the player is moving in both x and y direction, prioritize vertical animation.
            if controller.intent.y > 0.0 {
                match player_state.get() {
                    PlayerState::Awake => {
                        animation.facing = FacingDirection::Up;
                        PlayerAnimationState::WalkingUp
                    }
                    PlayerState::Asleep => PlayerAnimationState::WalkingSide,
                }
            } else if controller.intent.y < 0.0 {
                match player_state.get() {
                    PlayerState::Awake => {
                        animation.facing = FacingDirection::Down;
                        PlayerAnimationState::WalkingDown
                    }
                    PlayerState::Asleep => PlayerAnimationState::WalkingSide,
                }
            } else {
                animation.facing = FacingDirection::Side;
                PlayerAnimationState::WalkingSide
            }
        };
        animation.update_state(animation_state);
    }
}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(mut query: Query<(&mut PlayerAnimation, &mut Sprite)>) {
    for (mut animation, mut sprite) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        if animation.should_update_sprite() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

/// If the player is moving, play a step sound effect synchronized with the
/// animation.
fn trigger_step_sound_effect(
    mut commands: Commands,
    player_assets: If<Res<PlayerAssets>>,
    mut step_query: Query<&mut PlayerAnimation, With<MovementController>>,
) {
    for mut animation in &mut step_query {
        if (animation.state == PlayerAnimationState::WalkingSide
            || animation.state == PlayerAnimationState::WalkingUp
            || animation.state == PlayerAnimationState::WalkingDown)
            && animation.should_update_sprite()
            && (animation.frame == 1 || animation.frame == 3)
        {
            let rng = &mut rand::rng();
            let random_step = player_assets.steps.choose(rng).unwrap().clone();
            commands.spawn(sound_effect(random_step));
        }
    }
}

impl PlayerAnimation {
    /// The number of idle frames.
    const IDLE_FRAMES: usize = 1;
    /// The duration of each idle frame.
    const IDLE_INTERVAL: Duration = Duration::from_millis(170); //originally 170, 1000 doing this to see sprite alignment better
    /// The number of walking frames.
    const WALKING_FRAMES: usize = 4;
    /// The duration of each walking frame.
    const WALKING_INTERVAL: Duration = Duration::from_millis(170); //originally 170, 1000 doing this to see sprite alignment better

    fn idling(facing: FacingDirection) -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Idling,
            facing,
            state_changed: true,
        }
    }

    fn walking_side() -> Self {
        Self {
            timer: Timer::new(Self::WALKING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::WalkingSide,
            facing: FacingDirection::Side,
            state_changed: true,
        }
    }

    fn walking_up() -> Self {
        Self {
            timer: Timer::new(Self::WALKING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::WalkingUp,
            facing: FacingDirection::Up,
            state_changed: true,
        }
    }

    fn walking_down() -> Self {
        Self {
            timer: Timer::new(Self::WALKING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::WalkingDown,
            facing: FacingDirection::Down,
            state_changed: true,
        }
    }

    pub fn new(facing: FacingDirection) -> Self {
        Self::idling(facing)
    }

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.is_finished() {
            return;
        }
        self.frame = (self.frame + 1)
            % match self.state {
                PlayerAnimationState::Idling => Self::IDLE_FRAMES,
                PlayerAnimationState::WalkingSide => Self::WALKING_FRAMES,
                PlayerAnimationState::WalkingUp => Self::WALKING_FRAMES,
                PlayerAnimationState::WalkingDown => Self::WALKING_FRAMES,
            };
    }

    /// Update animation state if it changes.
    pub fn update_state(&mut self, state: PlayerAnimationState) {
        if self.state != state {
            *self = match state {
                PlayerAnimationState::Idling => Self::idling(self.facing),
                PlayerAnimationState::WalkingSide => Self::walking_side(),
                PlayerAnimationState::WalkingUp => Self::walking_up(),
                PlayerAnimationState::WalkingDown => Self::walking_down(),
            };
            self.state_changed = true;
        }
    }

    /// Whether animation changed this tick.
    pub fn should_update_sprite(&mut self) -> bool {
        let frame_advanced = self.timer.is_finished();
        let changed = frame_advanced || self.state_changed;
        self.state_changed = false;
        changed
    }

    /// Return sprite index in the atlas.
    pub fn get_atlas_index(&self) -> usize {
        match self.state {
            PlayerAnimationState::Idling => match self.facing {
                FacingDirection::Side => 0,
                FacingDirection::Up => 8,
                FacingDirection::Down => 4,
            },
            PlayerAnimationState::WalkingSide => 0 + self.frame,
            PlayerAnimationState::WalkingUp => 8 + self.frame,
            PlayerAnimationState::WalkingDown => 4 + self.frame,
        }
    }
}

/// Static animator, for simple objects who have animations
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct StaticAnimation {
    timer: Timer,
    frame: usize,
    state_changed: bool,
    atlas_cols: usize,
}

fn update_static_animation_timer(time: Res<Time>, mut query: Query<&mut StaticAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

fn update_static_animation_atlas(mut query: Query<(&mut StaticAnimation, &mut Sprite)>) {
    for (mut animation, mut sprite) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        if animation.should_update_sprite() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

impl StaticAnimation {
    const STATIC_FRAME_DURATION: Duration = Duration::from_millis(500);

    pub fn new(cols: usize) -> Self {
        Self {
            timer: Timer::new(Self::STATIC_FRAME_DURATION, TimerMode::Repeating),
            frame: 0,
            state_changed: true,
            atlas_cols: cols,
        }
    }

    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.just_finished() {
            return;
        }
        // println!("npc animation frame: {}", self.frame);
        self.frame = (self.frame + 1) % self.atlas_cols;
    }

    /// Whether animation changed this tick.
    pub fn should_update_sprite(&mut self) -> bool {
        let frame_advanced = self.timer.is_finished();
        let changed = frame_advanced || self.state_changed;
        self.state_changed = false;
        changed
    }

    pub fn get_atlas_index(&self) -> usize {
        self.frame
    }
}

fn gaze_follow_player(
    player_query: Query<&Transform, (With<MovementController>, Without<Npc>)>,
    mut npc_query: Query<(&Transform, &mut Sprite), (With<Npc>, Without<MovementController>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for (npc_transform, mut npc_sprite) in npc_query.iter_mut() {
        // player is to the left of the npc
        if player_transform.translation.x < npc_transform.translation.x {
            npc_sprite.flip_x = false;
        } else {
            npc_sprite.flip_x = true;
        }
    }
}
