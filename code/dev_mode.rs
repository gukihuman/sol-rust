use bevy::{
    prelude::*,
    input::common_conditions::{
        input_just_pressed,
        input_toggle_active
    }
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
                        input_toggle_active(false, KeyCode::N)
                    )
            )
            .add_systems(Update,
                (
                    convert::startup,
                    crosshair::startup,
                    collision_grid::spawn
                )
                    .run_if(
                        input_just_pressed(KeyCode::M)
                    )
            )
        ;
    }
}
