use bevy::{ prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_screen_diagnostics::{
    Aggregate,
    ScreenDiagnostics,
    ScreenDiagnosticsPlugin
};
use crate::*;
const DIAGNOSTICS_COLOR: Color = Color::rgb(0.69, 0.91, 0.882);
fn fps_format(fps: f64) -> String { format!("{:.0}", fps) }
fn ms_format(ms: f64) -> String { format!("{:.1} /", ms) }
#[derive(Resource)] struct ShowFpsLastState(bool);
pub struct DiagnosticsPlugin;
impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ShowFpsLastState(true))
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
            .add_systems(Update, update_diagnostics_visibility)
        ;
    }
}
fn spawn_diagnostics(mut diagnostics: ResMut<ScreenDiagnostics>) {
    diagnostics
        .add("fps".to_string(), FrameTimeDiagnosticsPlugin::FPS)
        .aggregate(Aggregate::MovingAverage(20))
        .format(fps_format)
        .diagnostic_color(DIAGNOSTICS_COLOR)
        .toggle_name();
    diagnostics
        .add("ms/frame".to_string(), FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .aggregate(Aggregate::MovingAverage(20))
        .format(ms_format)
        .diagnostic_color(DIAGNOSTICS_COLOR)
        .toggle_name();
}
fn update_diagnostics_visibility(
    mut diagnostics: ResMut<ScreenDiagnostics>,
    settings: Res<settings::Settings>
) {
    if settings.show_fps {
        diagnostics.modify("fps").diagnostic_color(DIAGNOSTICS_COLOR);
        diagnostics.modify("ms/frame").diagnostic_color(DIAGNOSTICS_COLOR);
    } else {
        diagnostics.modify("fps")
            .diagnostic_color(Color::rgba(1.0, 1.0, 1.0, 0.0));
        diagnostics.modify("ms/frame")
            .diagnostic_color(Color::rgba(1.0, 1.0, 1.0, 0.0));
    }
}