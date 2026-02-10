//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

pub mod animation;
mod action;
pub mod player;
pub mod player_ghost;
mod asleep;
mod awake;
mod camera;


pub(super) fn plugin(app: &mut App) {
    app.init_state::<PlayerState>();
    
    app.add_plugins((
        animation::plugin,
        action::plugin,
        player::plugin,
        player_ghost::plugin,
        asleep::plugin,
        awake::plugin,
        camera::plugin,
    ));
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum PlayerState {
    #[default]
    Awake,
    Asleep,
}
