use sol_rust::*;
use bevy::prelude::*;
use bevy::window::*;
#[cfg(target_os = "windows")] use dotenv::dotenv;

// manages default plugins and toggles dev mode
fn main() {
    #[cfg(target_os = "windows")] dotenv().ok(); // vulcan -> dx12
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen,
                    present_mode: PresentMode::AutoVsync,
                    title: "Spirit of Lira".into(),
                    resolution: (1280.0, 720.0).into(),
                    resizable: false,
                    window_theme: Some(WindowTheme::Dark),
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