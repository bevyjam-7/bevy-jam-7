use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::{
    AppSystems, PausableSystems,
    audio::sound_effect,
    player::{action::MovementController, player_ghost::player_ghost::GhostPlayerAssets},
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
pub struct GhostPlayerAnimation {
    timer: Timer,
    frame: usize,
    state: GhostPlayerAnimationState,
    facing: GhostFacingDirection,
    state_changed: bool,
}

#[derive(Reflect, PartialEq)]
pub enum GhostPlayerAnimationState {
    FloatingIdling,
    FloatingSide,
    FloatingUp,
    FloatingDown,
}

#[derive(Reflect, PartialEq, Clone, Copy)]
pub enum GhostFacingDirection {
    Side,
    Up,
    Down,
}

/// Update the animation timer.
fn update_animation_timer(time: Res<Time>, mut query: Query<&mut GhostPlayerAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

fn update_animation_movement(
    mut player_query: Query<(&MovementController, &mut Sprite, &mut GhostPlayerAnimation)>,
) {
    for (controller, mut sprite, mut animation) in &mut player_query {
        let dx = controller.intent.x;
        if dx != 0.0 {
            sprite.flip_x = dx < 0.0;
        }

        let animation_state = if controller.intent == Vec2::ZERO {
            GhostPlayerAnimationState::FloatingIdling
        } else {
            // If the player is moving in both x and y direction, prioritize vertical animation. 
            if controller.intent.y > 0.0 {
                animation.facing = GhostFacingDirection::Up;
                GhostPlayerAnimationState::FloatingUp
            } else if controller.intent.y < 0.0 {
                animation.facing = GhostFacingDirection::Down;
                GhostPlayerAnimationState::FloatingDown
            } else {
                animation.facing = GhostFacingDirection::Side;
                GhostPlayerAnimationState::FloatingSide
            }
        };
        animation.update_state(animation_state);
    }
}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(mut query: Query<(&mut GhostPlayerAnimation, &mut Sprite)>) {
    for (mut animation, mut sprite) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        if animation.should_update_sprite() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

fn trigger_step_sound_effect(
    mut commands: Commands,
    player_assets: If<Res<GhostPlayerAssets>>,
    mut float_query: Query<&mut GhostPlayerAnimation>,
) {
    for mut animation in &mut float_query {
        if (animation.state == GhostPlayerAnimationState::FloatingSide || animation.state == GhostPlayerAnimationState::FloatingUp || animation.state == GhostPlayerAnimationState::FloatingDown)
            && animation.should_update_sprite()
            && (animation.frame == 1 || animation.frame == 3)
        {
            let rng = &mut rand::rng();
            let random_float = player_assets.ghost_ducky_float_audio.choose(rng).unwrap().clone();
            commands.spawn(sound_effect(random_float));
        }
    }
}

impl GhostPlayerAnimation {
    /// The number of idle frames.
    const FLOATING_IDLE_FRAMES: usize = 4;
    /// The duration of each idle frame.
    const FLOATING_IDLE_INTERVAL: Duration = Duration::from_millis(170); // originally 170
    /// The number of walking frames.
    const FLOATING_FRAMES: usize = 4;
    /// The duration of each walking frame.
    const FLOATING_INTERVAL: Duration = Duration::from_millis(170); // originally 170

    fn floating_idling() -> Self {
        Self {
            timer: Timer::new(Self::FLOATING_IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: GhostPlayerAnimationState::FloatingIdling,
            facing: GhostFacingDirection::Down,
            state_changed: true,
        }
    }

    fn floating_side() -> Self {
        Self {
            timer: Timer::new(Self::FLOATING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: GhostPlayerAnimationState::FloatingSide,
            facing: GhostFacingDirection::Side,
            state_changed: true,
        }
    }

    fn floating_up() -> Self {
        Self {
            timer: Timer::new(Self::FLOATING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: GhostPlayerAnimationState::FloatingUp,
            facing: GhostFacingDirection::Up,
            state_changed: true,
        }
    }

    fn floating_down() -> Self {
        Self {
            timer: Timer::new(Self::FLOATING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: GhostPlayerAnimationState::FloatingDown,
            facing: GhostFacingDirection::Down,
            state_changed: true,
        }
    }

    pub fn new() -> Self {
        Self::floating_idling()
    }

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.is_finished() {
            return;
        }
        self.frame = (self.frame + 1)
            % match self.state {
                GhostPlayerAnimationState::FloatingIdling => Self::FLOATING_IDLE_FRAMES,
                GhostPlayerAnimationState::FloatingSide => Self::FLOATING_FRAMES,
                GhostPlayerAnimationState::FloatingUp => Self::FLOATING_FRAMES,
                GhostPlayerAnimationState::FloatingDown => Self::FLOATING_FRAMES,
            };
    }

    /// Update animation state if it changes.
    pub fn update_state(&mut self, state: GhostPlayerAnimationState) {
        if self.state != state {
            *self = match state {
                GhostPlayerAnimationState::FloatingIdling => Self::floating_idling(),
                GhostPlayerAnimationState::FloatingSide => Self::floating_side(),
                GhostPlayerAnimationState::FloatingUp => Self::floating_up(),
                GhostPlayerAnimationState::FloatingDown => Self::floating_down(),
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
            GhostPlayerAnimationState::FloatingIdling => match self.facing {
                GhostFacingDirection::Side => 0,
                GhostFacingDirection::Up => 0,
                GhostFacingDirection::Down => 0,
            },
            GhostPlayerAnimationState::FloatingIdling => 0 + self.frame,
            GhostPlayerAnimationState::FloatingSide => 0 + self.frame,
            GhostPlayerAnimationState::FloatingUp => 0 + self.frame,
            GhostPlayerAnimationState::FloatingDown => 0 + self.frame,
        }
    }
}

