use sol_rust::*;
use bevy::prelude::*;
use bevy::window::*;

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
        .add_plugins(BootPlugin)
        .add_plugins(dev_mode::DevModePlugin)
        .run();
}