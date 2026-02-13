use std::time::Duration;

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, game_consts::WITCH_HOUSE_ATLAS_COLS};

pub(super) fn plugin (app: &mut App) {
    app.add_systems(
        Update,
        (
        update_house_animation_timer.in_set(AppSystems::TickTimers),
        (
            update_house_animation_timer, 
            update_house_animation_atlas,
        )
            .chain()
            .in_set(AppSystems::Update)
        ).in_set(PausableSystems)
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HouseAnimation {
    timer: Timer,
    frame: usize,
    state_changed: bool,
}

fn update_house_animation_timer(
    time: Res<Time>,
    mut query: Query<&mut HouseAnimation>,
) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

fn update_house_animation_atlas(
    mut query: Query<(&mut HouseAnimation, &mut Sprite)>,
) {
    for (mut animation, mut sprite) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        if animation.should_update_sprite() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

impl HouseAnimation {
    const HOUSE_FRAME_DURATION: Duration = Duration::from_millis(200);

    pub fn new() -> Self {
        Self {
            timer: Timer::new(Self::HOUSE_FRAME_DURATION, TimerMode::Repeating),
            frame: 0,
            state_changed: true,
        }
    }

    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.just_finished() {
            return;
        }
        // println!("House animation frame: {}", self.frame);
        self.frame = (self.frame + 1) % WITCH_HOUSE_ATLAS_COLS as usize;
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