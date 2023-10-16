use bevy::prelude::*; use crate::*;
pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin { fn build(&self, app: &mut App) { app
    .add_systems(Update, start_game)
;}}
fn start_game(
    keys: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<core::GameState>>,
){
    if keys.just_pressed(KeyCode::M) {
        next_state.set(core::GameState::InGame);
    }
}