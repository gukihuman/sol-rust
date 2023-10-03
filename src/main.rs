use sol_rust::prelude::*;
use bevy::{prelude::*, input::common_conditions::input_toggle_active};
use bevy::window::*;
use bevy_screen_diagnostics::{
    ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    present_mode: PresentMode::Fifo,
                    title: "Spirit of Lira".into(),
                    resolution: (1280., 720.).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .build(),
        )
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(
            WorldInspectorPlugin::default()
                .run_if(
                    input_toggle_active(true, KeyCode::N)
                )
        )
        .add_plugins(BootPlugins)
        .add_plugins(DevModePlugin) // turn off for release
        .run();
}