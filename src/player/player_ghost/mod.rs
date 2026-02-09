use bevy::prelude::*;
use crate::player::PlayerState;

pub mod player_ghost;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<PlayerState>();

    app.add_plugins((
        player_ghost::plugin,
    ));
}

