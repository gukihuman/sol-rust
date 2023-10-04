use bevy::prelude::*;
pub mod dev_mode;
pub mod core;
pub mod motion;

pub struct BootPlugin;
impl Plugin for BootPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .insert_resource(core::collision::CollisionArray::default())
            .insert_resource(core::time::WorldTime::default())
            .insert_resource(core::gamepad::GamepadState::default())
            .insert_resource(core::camera::CameraFollowedEntity::default())
            .add_plugins(core::camera::CameraPlugin)
            .add_plugins(motion::MotionPlugin)
            .add_systems(Update, core::gamepad::update);
    }
}