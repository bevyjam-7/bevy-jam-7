use bevy::{ecs::system::command, prelude::*};
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;
pub(super) fn plugin(app: &mut App) {

}

// Dialogue when the game first starts. The game is in pause state when dialogue is in action
pub fn start_dialogue(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);

    dialogue_runner.start_node("WakeUp");
    commands.spawn(dialogue_runner);
}