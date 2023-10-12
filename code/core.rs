use bevy::prelude::*;
use crate::*;
pub mod collision;
pub mod time;
pub mod camera;
pub mod gamepad;

pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(collision::CollisionArray::default())
            .add_plugins((
                camera::CameraPlugin,
                input_map::InputMapPlugin
            ))
            .add_systems(Update, (
                gamepad::update_gamepad_state,
            ));
    }
}
