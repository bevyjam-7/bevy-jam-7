use bevy::prelude::*;

pub mod dialogue;

pub (super) fn plugin (app: &mut App){
    app.add_plugins((
        dialogue::plugin,
    ));
}