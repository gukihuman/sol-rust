use crate::*;
use bevy::prelude::*;

pub struct StartBundlePlugins;
impl Plugin for StartBundlePlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .insert_resource( core::collision::CollisionArray::default() )
            .insert_resource( core::time::WorldTime::default() )
            .insert_resource( core::gamepad::GamepadState::default() )
            .insert_resource( motion::destination::ControlledEntity::default() )
            .insert_resource( motion::indicator::IndicatorEntity::default() )
            .add_plugins( core::camera::CameraPlugin )
            .add_plugins( motion::MotionPlugin )
            .add_systems( Update, core::gamepad::update )
            ;
    }
}