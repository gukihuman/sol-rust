use crate::*;
use bevy::prelude::*;
pub struct StartBundlePlugins;
impl Plugin for StartBundlePlugins { fn build(&self, app: &mut App) { app
    .add_state::<core::GameState>()
    .init_resource::<time::WorldTime>()
    .init_resource::<collision::CollisionArray>()
    .add_plugins((
        camera::CameraPlugin,
        gamepad::GamepadPlugin,
        settings_input::SettingsInputPlugin,
        settings::SettingsPlugin,
        diagnostics::DiagnosticsPlugin,
        main_menu::MainMenuPlugin,
        movement::MotionPlugin
    ))
;}}