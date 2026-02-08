use bevy::prelude::*;

use crate::{PausableSystems, PlayerCamera, player::player::Player};

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(Update, 
    //     camera_follow_player.in_set(PausableSystems));
}


// A camera that follows the player around, TODO: make 
// fn camera_follow_player(
//     mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
//     mut player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
// ) {
//     let Ok(mut camera_transform) = camera_query.single_mut() else {
//         return;
//     };
//     let Ok(player_transform) = player_query.single_mut() else {
//         return;
//     };

//     // Move the camera to the player's position, but keep the camera's z position.
//     let new_camera_position = Vec3::new(
//         player_transform.translation.x,
//         player_transform.translation.y,
//         camera_transform.translation.z,
//     );

//     // Update the camera's transform to follow the player.
//     // You can also add some smoothing here if you want!
//     // For example, you could use `Vec3::lerp` to smoothly interpolate between the current camera position and the new camera position.
//     camera_transform.translation = new_camera_position;
// }