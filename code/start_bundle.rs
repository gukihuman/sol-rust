use crate::*;
use bevy::prelude::*;

#[derive(Event)] pub struct StartGameEvent();

pub struct StartBundlePlugins;
impl Plugin for StartBundlePlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .insert_resource(core::collision::CollisionArray::default())
            .insert_resource(core::time::WorldTime::default())
            .insert_resource(core::gamepad::GamepadState::default())
            .insert_resource(motion::destination::ControlledEntity::default())
            .insert_resource(motion::indicator::IndicatorEntity::default())
            .add_plugins(core::camera::CameraPlugin)
            .add_plugins(motion::MotionPlugin)
            .add_systems(Update, core::gamepad::update)
            .add_systems(Update, handle_start_game_event)
            .add_event::<StartGameEvent>()
        ;
    }
}

fn handle_start_game_event(
    keys: Res<Input<KeyCode>>,
    mut start_game_event: EventWriter<StartGameEvent>
) {
    if keys.just_pressed(KeyCode::M) {
        start_game_event.send(StartGameEvent());
    }
}