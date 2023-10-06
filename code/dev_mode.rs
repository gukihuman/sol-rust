use bevy::{
    prelude::*,
    input::common_conditions::input_toggle_active
};
use bevy_screen_diagnostics::{
    ScreenDiagnosticsPlugin,
    ScreenFrameDiagnosticsPlugin
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
pub mod crosshair;
pub mod collision_grid;
pub mod convert;

pub struct DevModePlugin;
impl Plugin for DevModePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ScreenDiagnosticsPlugin::default())
            .add_plugins(ScreenFrameDiagnosticsPlugin)
            .add_plugins(
                WorldInspectorPlugin::default()
                    .run_if(
                        input_toggle_active(true, KeyCode::N)
                    )
            )
            .add_systems(Startup, crosshair::startup)
            .add_systems(Startup, collision_grid::spawn)
            .add_systems(Startup, convert::startup)
            ;
    }
}
