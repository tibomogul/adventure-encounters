use crate::prelude::*;

pub mod illumination;
pub mod field_of_view;

#[derive(Component)]
pub struct Player;

pub fn player_render_system(
    mut player_transform: Query<&mut Transform, With<Player>>
) {
    // Temporary system to render the player. A value of 2.4 or lower (intended is 2.0, as the objects were intended to be 1.0.)
    // will not diplay the Sprite. Note that in Bevy 0.9 this wasnt required
    if let Ok(mut transform) = player_transform.get_single_mut() {
        transform.translation.z = 2.5;
    }
}