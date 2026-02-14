use std::time::Duration;

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, game_consts::{NPC_ATLAS_COLS, WITCH_HOUSE_ATLAS_COLS}, map::npc::Npc, player::action::MovementController};

pub(super) fn plugin (app: &mut App) {
    app.add_systems(
        Update,
        (
        update_house_animation_timer.in_set(AppSystems::TickTimers),
        (
            update_house_animation_atlas,
        )
            .chain()
            .in_set(AppSystems::Update)
        ).in_set(PausableSystems)
    );

    app.add_systems(
        Update,
        (
        update_npc_animation_timer.in_set(AppSystems::TickTimers),
        (
            update_npc_animation_atlas,
            gaze_follow_player,
        )
            .chain()
            .in_set(AppSystems::Update)
        ).in_set(PausableSystems)
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct NpcAnimation {
    timer: Timer,
    frame: usize,
    state_changed: bool,
}

fn update_npc_animation_timer(
    time: Res<Time>,
    mut query: Query<&mut NpcAnimation>,
) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

fn update_npc_animation_atlas(
    mut query: Query<(&mut NpcAnimation, &mut Sprite)>,
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

fn gaze_follow_player(
    player_query: Query<&Transform, (With<MovementController>, Without<Npc>)>,
    mut npc_query: Query<(&Transform, &mut Sprite), (With<Npc>, Without<MovementController>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for (npc_transform, mut npc_sprite) in npc_query.iter_mut() {
        // player is to the left of the npc
        if (player_transform.translation.x < npc_transform.translation.x) {
            npc_sprite.flip_x = false;
        } else {
            npc_sprite.flip_x = true;
        }
    }
}


impl NpcAnimation {
    const NPC_FRAME_DURATION: Duration = Duration::from_millis(500);

    pub fn new() -> Self {
        Self {
            timer: Timer::new(Self::NPC_FRAME_DURATION, TimerMode::Repeating),
            frame: 0,
            state_changed: true,
        }
    }

    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.just_finished() {
            return;
        }
        // println!("npc animation frame: {}", self.frame);
        self.frame = (self.frame + 1) % NPC_ATLAS_COLS;
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