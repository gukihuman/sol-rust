use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use crate::core::gamepad::GamepadState;
const ZOOM: f32 = 0.5;
const ZOOM_MIN: f32 = 0.5;
const ZOOM_MAX: f32 = 2.;
const ZOOM_SPEED: f32 = 1.;
#[derive(Resource)] pub struct Camera(Entity);
#[derive(Resource)] pub struct CameraZoom(f32);
#[derive(Resource)] pub struct CameraFollowedEntity(pub Option<Entity>);
impl Default for CameraFollowedEntity { fn default() -> Self { Self (None) } }
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
            .add_systems(Update, (zoom, follow));

    }
}
pub fn startup (mut commands: Commands) {
    // let camera = commands.spawn(Camera2dBundle::default()).id();
    let camera = commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: -1000.0,
            far: 0.0,
            ..Default::default()
        },
        ..Default::default()
    }).id();
    commands.insert_resource(Camera(camera));
    commands.insert_resource(CameraZoom(ZOOM));
}
pub fn zoom(
    mut transforms: Query<&mut Transform>,
    mut camera_zoom: ResMut<CameraZoom>,
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    gamepad_state: ResMut<GamepadState>,
    time: Res<Time>,
    camera: Res<Camera>,
) {
    // zoom
    for event in mouse_wheel_reader.iter() {
        camera_zoom.0 *= 1.0 - event.y * 0.1;
    }
    let fps_adjusted_zoom_speed = ZOOM_SPEED * time.delta_seconds();
    camera_zoom.0 *= 1.0 - gamepad_state.right_stick_y * fps_adjusted_zoom_speed;

    camera_zoom.0 = camera_zoom.0.clamp(ZOOM_MIN, ZOOM_MAX);
    if let Ok(mut transform) = transforms.get_mut(camera.0) {
        transform.scale.x = camera_zoom.0;
        transform.scale.y = camera_zoom.0;
    }
}
pub fn follow(
    camera: Res<Camera>,
    mut transforms: Query<&mut Transform>,
    followed_entity: Res<CameraFollowedEntity>
) {
    if let Some(entity_to_follow) = followed_entity.0 {
        if let Ok(entity_transform) = transforms.get_component::<Transform>(entity_to_follow) {
            let entity_position = entity_transform.translation;
            if let Ok(mut camera_transform) = transforms.get_mut(camera.0) {
                camera_transform.translation.x = entity_position.x;
                camera_transform.translation.y = entity_position.y;
            }
        }
    }
}