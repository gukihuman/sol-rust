use bevy::prelude::*;
pub mod movement;

pub struct MotionPlugin;
impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement::update);
    }
}
