//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::{
    AppSystems, PausableSystems,
    audio::sound_effect,
    player::{action::MovementController, player::PlayerAssets},
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
}

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
fn update_animation_timer(time: Res<Time>, mut query: Query<&mut PlayerAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Update the sprite direction and animation state (idling/walking).
fn update_animation_movement(
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
                animation.facing = FacingDirection::Up;
                PlayerAnimationState::WalkingUp
            } else if controller.intent.y < 0.0 {
                animation.facing = FacingDirection::Down;
                PlayerAnimationState::WalkingDown
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
    mut step_query: Query<&mut PlayerAnimation>,
) {
    for mut animation in &mut step_query {
        if (animation.state == PlayerAnimationState::WalkingSide || animation.state == PlayerAnimationState::WalkingUp || animation.state == PlayerAnimationState::WalkingDown)
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
    const IDLE_INTERVAL: Duration = Duration::from_millis(170);
    /// The number of walking frames.
    const WALKING_FRAMES: usize = 4;
    /// The duration of each walking frame.
    const WALKING_INTERVAL: Duration = Duration::from_millis(170);

    fn idling() -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Idling,
            facing: FacingDirection::Down,
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

    pub fn new() -> Self {
        Self::idling()
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
                PlayerAnimationState::Idling => Self::idling(),
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
                FacingDirection::Up => 4,
                FacingDirection::Down => 4,
            },
            PlayerAnimationState::Idling => 0 + self.frame,
            PlayerAnimationState::WalkingSide => 0 + self.frame,
            PlayerAnimationState::WalkingUp => 4 + self.frame,
            PlayerAnimationState::WalkingDown => 4 + self.frame,
        }
    }
}


