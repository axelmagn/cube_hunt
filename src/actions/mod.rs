use bevy::prelude::*;

use crate::actions::game_control::{get_movement, GameControl};
use crate::GameState;

mod game_control;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_systems(
            Update,
            set_movement_actions.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
    pub camera_orbit: Option<Vec2>,
}

pub fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    let camera_orbit = get_movement(GameControl::OrbitCameraUp, &keyboard_input) * Vec2::Y
        - get_movement(GameControl::OrbitCameraDown, &keyboard_input) * Vec2::Y
        + get_movement(GameControl::OrbitCameraLeft, &keyboard_input) * Vec2::X
        - get_movement(GameControl::OrbitCameraRight, &keyboard_input) * Vec2::X;
    actions.camera_orbit = if camera_orbit != Vec2::ZERO {
        Some(camera_orbit)
    } else {
        None
    };
}
