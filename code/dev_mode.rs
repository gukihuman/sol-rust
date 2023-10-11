use bevy::{
    prelude::*,
    input::common_conditions::input_toggle_active
};
use bevy_screen_diagnostics::{
    ScreenDiagnosticsPlugin,
    ScreenFrameDiagnosticsPlugin
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::*;
pub mod crosshair;
pub mod collision_grid;
pub mod convert;

pub struct DevModePlugin;
impl Plugin for DevModePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, convert::convert_assets)
            .add_plugins(ScreenDiagnosticsPlugin::default())
            .add_plugins(ScreenFrameDiagnosticsPlugin)
            .add_plugins(
                WorldInspectorPlugin::default()
                    .run_if(
                        input_toggle_active(false, KeyCode::N)
                    )
            )
            .add_systems(Update, crosshair::startup.run_if(game_started))
            .add_systems(Update, collision_grid::spawn.run_if(game_started))
        ;
    }
}
fn game_started(
    mut start_game_event: EventReader<start_bundle::StartGameEvent>
) -> bool {
    start_game_event.iter().next().is_some()
}