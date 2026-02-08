//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

use bevy::{prelude::*, ui::debug};
use rand::prelude::*;
use std::{clone, time::Duration};

use crate::{
    AppSystems, PausableSystems,
    audio::sound_effect,
    player::{movement::MovementController, player::PlayerAssets},
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

/// Update the animation timer.
fn update_animation_timer(time: Res<Time>, mut query: Query<&mut PlayerAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Update the sprite direction and animation state (idling/walking).
fn update_animation_movement(
    mut player_query: Query<(&MovementController, &mut Sprite, &mut PlayerAnimation, &mut Facing)>,
) {
    for (controller, mut sprite, mut animation, mut facing) in &mut player_query {
    let intent = controller.intent;

    if intent != Vec2::ZERO {
        *facing = Facing(FacingDirection::from_vec2(intent));
    }

    // sprite flip (optional, you may later remove this)
    if intent.x != 0.0 {
        sprite.flip_x = intent.x < 0.0;
    }

    let animation_state = if intent == Vec2::ZERO {
        PlayerAnimationState::Idling
    } else {
        PlayerAnimationState::Walking
    };

    animation.update_state(animation_state);
}

}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(
    mut query: Query<(&PlayerAnimation, &Facing, &mut Sprite)>
) {
    for (animation, facing, mut sprite) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };

        if animation.changed() {
            atlas.index = animation.get_atlas_index(facing.0);
        }
    }
}


/// If the player is moving, play a step sound effect synchronized with the
/// animation.
fn trigger_step_sound_effect(
    mut commands: Commands,
    player_assets: If<Res<PlayerAssets>>,
    mut step_query: Query<&PlayerAnimation>,
) {
    for animation in &mut step_query {
        if animation.state == PlayerAnimationState::Walking
            && animation.changed()
            && (animation.frame == 2 || animation.frame == 4)
        {
            let rng = &mut rand::rng();
            let random_step = player_assets.steps.choose(rng).unwrap().clone();
            commands.spawn(sound_effect(random_step));
        }
    }
}

/// Component that tracks player's animation state.
/// It is tightly bound to the texture atlas we use.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerAnimation {
    timer: Timer,
    frame: usize,
    state: PlayerAnimationState,
}

#[derive(Reflect, PartialEq)]
pub enum PlayerAnimationState {
    Idling,
    Walking,
}

impl PlayerAnimation {
    /// The number of idle frames.
    const IDLE_FRAMES: usize = 3;
    /// The duration of each idle frame.
    const IDLE_INTERVAL: Duration = Duration::from_millis(500);
    /// The number of walking frames.
    const WALKING_FRAMES: usize = 4;
    /// The duration of each walking frame.
    const WALKING_INTERVAL: Duration = Duration::from_millis(50);

    fn idling() -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Idling,
        }
    }

    fn walking() -> Self {
        Self {
            timer: Timer::new(Self::WALKING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Walking,
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
                PlayerAnimationState::Walking => Self::WALKING_FRAMES,
            };
    }

    /// Update animation state if it changes.
    pub fn update_state(&mut self, state: PlayerAnimationState) {
        if self.state != state {
            match state {
                PlayerAnimationState::Idling => *self = Self::idling(),
                PlayerAnimationState::Walking => *self = Self::walking(),
            }
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.is_finished()
    }

    /// Return sprite index in the atlas depending on faced direction
    pub fn get_atlas_index(
        &self,
        direction: FacingDirection,
    ) -> usize {
        match direction {
            FacingDirection::Up => match self.state {
                PlayerAnimationState::Idling => 0,
                PlayerAnimationState::Walking => 1 + self.frame,
            },

            FacingDirection::Left => match self.state {
                PlayerAnimationState::Idling => 4,
                PlayerAnimationState::Walking => 5 + self.frame,
            },

            FacingDirection::Right => match self.state {
                PlayerAnimationState::Idling => 8,
                PlayerAnimationState::Walking => 9 + self.frame,
            },

            FacingDirection::Down => match self.state {
                PlayerAnimationState::Idling => 4,
                PlayerAnimationState::Walking => 5 + self.frame,
            },
        }
    }
}

// possible directions that can be faced
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FacingDirection 
{
    Up,
    Down,
    Left,
    Right,
}

impl FacingDirection {
    pub fn from_vec2(v: Vec2) -> Self {
        if v.y > 0.0 {
            FacingDirection::Up
        } else if v.x < 0.0 {
            FacingDirection::Left
        } else if v.x > 0.0 {
            FacingDirection::Right
        } else {
            FacingDirection::Down
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Facing(pub FacingDirection);


