use crate::*;
use bevy::prelude::*;
pub struct StartBundlePlugins;
impl Plugin for StartBundlePlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_state::<core::GameState>()
            .init_resource::<time::WorldTime>()
            .init_resource::<collision::CollisionArray>()
            .add_plugins((
                movement::MotionPlugin,
                camera::CameraPlugin,
                settings_input::SettingsInputPlugin,
                settings::SettingsPlugin,
                diagnostics::DiagnosticsPlugin,
                gamepad::GamepadPlugin
            ))
            .add_systems(Update, (
                start_game,
                gamepad::update_gamepad_state,
            ))
        ;
    }
}
fn start_game(
    keys: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<core::GameState>>,
){
    if keys.just_pressed(KeyCode::M) {
        next_state.set(core::GameState::InGame);
    }
}