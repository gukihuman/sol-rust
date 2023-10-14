use bevy::{ prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_screen_diagnostics::{
    Aggregate,
    ScreenDiagnostics,
    ScreenDiagnosticsPlugin
};
const DIAGNOSTIC_COLOR: Color = Color::rgb(0.69, 0.91, 0.882);
fn fps_format(fps: f64) -> String { format!("{:.0}", fps) }
fn ms_format(ms: f64) -> String { format!("{:.1} /", ms) }
pub struct DiagnosticPlugin;
impl Plugin for DiagnosticPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                ScreenDiagnosticsPlugin{
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        top: Val::Px(5.0),
                        right: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                },
                FrameTimeDiagnosticsPlugin,
            ))
            .add_systems(Startup, spawn_diagnostics)
        ;
    }
}
fn spawn_diagnostics(mut diagnostics: ResMut<ScreenDiagnostics>) {
    diagnostics
        .add("fps".to_string(), FrameTimeDiagnosticsPlugin::FPS)
        .aggregate(Aggregate::MovingAverage(20))
        .format(fps_format)
        .diagnostic_color(DIAGNOSTIC_COLOR)
        .toggle_name();
    diagnostics
        .add("ms/frame".to_string(), FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .aggregate(Aggregate::MovingAverage(20))
        .format(ms_format)
        .diagnostic_color(DIAGNOSTIC_COLOR)
        .toggle_name();
}
// fn toggle_diagnostics(mut diagnostics: ResMut<ScreenDiagnostics>) {
//     diagnostics.modify("fps").toggle();
//     diagnostics.modify("ms/frame").toggle();
// }