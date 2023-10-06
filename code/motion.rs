use bevy::prelude::*;
pub mod movement;
pub mod destination;
pub mod indicator;

pub struct MotionPlugin;
impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement::update);
        app.add_systems(Update, destination::update);
        app.add_systems(Update, (indicator::update, indicator::position));
    }
}
