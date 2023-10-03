use bevy::prelude::*;
pub mod crosshair;
pub mod collision_grid;

pub struct DevModePlugin;
impl Plugin for DevModePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, crosshair::setup)
            .add_systems(Startup, collision_grid::setup)
            .add_systems(Update, collision_grid::spawn); // spawned once
    }
}
