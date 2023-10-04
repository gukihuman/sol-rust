pub mod resources {
    pub mod collision;
    pub mod time;
}
pub mod dev_mode;
pub mod camera;
pub mod motion;

pub mod prelude {
    use bevy::prelude::Plugin;
    use crate::resources::{
        collision::*,
        time::*,
    };

    pub use crate::dev_mode::DevModePlugin;

    pub struct BootPlugin;
    use crate::camera::CameraPlugin;
    use crate::motion::MotionPlugin;
    impl Plugin for BootPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app
                .insert_resource(CollisionArray::default())
                .insert_resource(WorldTime::default())
                .add_plugins(CameraPlugin)
                .add_plugins(MotionPlugin);
        }
    }
}