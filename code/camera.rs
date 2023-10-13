use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use crate::*;
const ZOOM: f32 = 0.5;
const ZOOM_MIN: f32 = 0.5;
const ZOOM_MAX: f32 = 2.0;
const ZOOM_SPEED: f32 = 1.0;
#[derive(Resource)] pub struct Camera {
    entity: Entity, // camera itself
    zoom: f32,
    pub followed_entity: Option<Entity>,
}
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (zoom, follow))
        ;
    }
}
pub fn spawn_camera (mut commands: Commands) {
    let camera = commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: -1000.0,
            far: 0.0,
            ..Default::default()
        },
        ..Default::default()
    }).id();
    commands.insert_resource(
        Camera {
            entity: camera,
            zoom: ZOOM,
            followed_entity: None,
        }
    );
}
fn zoom(
    mut camera: ResMut<Camera>,
    mut transforms: Query<&mut Transform>,
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    gamepad_state: ResMut<gamepad::GamepadState>,
    time: Res<Time>,
) {
    let fps_adjusted_zoom_speed = ZOOM_SPEED * time.delta_seconds();
    for event in mouse_wheel_reader.iter() {
        // 📜 ajdust value (expected super slow now)
        camera.zoom *= 1.0 - event.y * 0.1 * fps_adjusted_zoom_speed;
    }
    camera.zoom *= 1.0 - gamepad_state.right_stick_y * fps_adjusted_zoom_speed;
    camera.zoom = camera.zoom.clamp(ZOOM_MIN, ZOOM_MAX);
    if let Ok(mut transform) = transforms.get_mut(camera.entity) {
        transform.scale.x = camera.zoom;
        transform.scale.y = camera.zoom;
    }
}
fn follow(camera: ResMut<Camera>, mut transforms: Query<&mut Transform>) {
    let mut followed_transform: Option<Transform> = None;
    if let Some(followed_entity) = camera.followed_entity {
        if let Ok(transform) = transforms.get_mut(followed_entity) {
            followed_transform = Some(transform.clone());
        }
    }
    if let Some(transform) = followed_transform {
        if let Ok(mut camera_transform) = transforms.get_mut(camera.entity) {
            camera_transform.translation.x = transform.translation.x;
            camera_transform.translation.y = transform.translation.y;
        }
    }
}