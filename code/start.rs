use sol_rust::*;
use bevy::prelude::*;
use bevy::window::*;

// manages default plugins and toggles dev mode
fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    present_mode: PresentMode::Fifo,
                    title: "Spirit of Lira".into(),
                    resolution: (1280.0, 720.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .build(),
        )
        .add_plugins(start_bundle::StartBundlePlugins)
        .add_plugins(dev_mode::DevModePlugin)
        .run();
}