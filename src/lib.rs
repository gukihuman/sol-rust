pub mod camera;
pub mod collision;
pub mod dev_mode;

pub mod prelude {
    pub use crate::camera::CameraPlugin;
    pub use crate::collision::CollisionPlugin;
    pub use crate::dev_mode::DevModePlugin;

    use bevy::prelude::Plugin;

    pub struct BootPlugins;

    impl Plugin for BootPlugins {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_plugins(CameraPlugin)
               .add_plugins(CollisionPlugin);
        }
    }
}