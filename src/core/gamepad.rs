use bevy::prelude::*;
use bevy::input::gamepad::{GamepadEvent, GamepadAxisType};


#[derive(Resource)]
pub struct GamepadState {
    pub deadzone: f32,
    pub left_stick_x: f32,
    pub left_stick_y: f32,
    pub right_stick_x: f32,
    pub right_stick_y: f32,
}
impl Default for GamepadState {
    fn default() -> Self {
        Self {
            deadzone: 0.15,
            left_stick_x: 0.,
            left_stick_y: 0.,
            right_stick_x: 0.,
            right_stick_y: 0.,
        }
    }
}
pub fn update(
    mut gamepad_state: ResMut<GamepadState>,
    mut gamepad_event_reader: EventReader<GamepadEvent>,
) {
    for event in gamepad_event_reader.iter() {
        if let GamepadEvent::Axis(axis_event) = event {
            let value =
                if axis_event.value.abs() < gamepad_state.deadzone { 0.0 }
                else { axis_event.value };
            match axis_event.axis_type {
                GamepadAxisType::LeftStickX => gamepad_state.left_stick_x = value,
                GamepadAxisType::LeftStickY => gamepad_state.left_stick_y = value,
                GamepadAxisType::RightStickX => gamepad_state.right_stick_x = value,
                GamepadAxisType::RightStickY => gamepad_state.right_stick_y = value,
                _ => {},
            }
        }
    }
}