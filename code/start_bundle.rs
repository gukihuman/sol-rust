use crate::*;
use bevy::prelude::*;
#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState { #[default] MainMenu, InGame, Scene }
pub struct StartBundlePlugins;
impl Plugin for StartBundlePlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_state::<AppState>()
            .insert_resource(core::time::WorldTime::default())
            .insert_resource(core::gamepad::GamepadState::default())
            .insert_resource(
                motion::destination::ControlledEntity::default()
            )
            .insert_resource(motion::indicator::IndicatorEntity::default())
            .add_plugins((
                core::CorePlugin,
                motion::MotionPlugin,
            ))
            .add_systems(Update, core::gamepad::update_gamepad_state)
            .add_systems(Update, start_game)
        ;
    }
}
fn start_game(
    keys: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
){
    if keys.just_pressed(KeyCode::M) {
        next_state.set(AppState::InGame);
    }
}