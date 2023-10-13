use sol_rust::*;
use bevy::prelude::*;
use bevy::window::*;
#[cfg(target_os = "windows")] use dotenv::dotenv;
fn main() {
    #[cfg(target_os = "windows")] dotenv().ok(); // build.rs vulcan -> dx12
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen,
                    present_mode: PresentMode::AutoVsync,
                    title: "Spirit of Lira".into(),
                    resolution: (1920.0, 1080.0).into(),
                    resizable: false,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }
        ).build()
    );
    app.add_plugins(start_bundle::StartBundlePlugins);
    #[cfg(debug_assertions)] app.add_plugins(dev_mode::DevModePlugin);
    app.run();
}