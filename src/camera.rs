use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy::input::gamepad::{GamepadEvent, GamepadAxisType};

const ZOOM_MIN: f32 = 0.5; // assuming 0.5 is your minimum zoom
const ZOOM_MAX: f32 = 2.0; // assuming 2.0 is your maximum zoom        
const DEADZONE: f32 = 0.15;

#[derive(Resource)]
pub struct MainCamera(Entity);

#[derive(Resource)]
pub struct CameraZoom(f32);

#[derive(Resource, Default)]
pub struct GamepadState {
    right_stick_y: f32,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, start)
            .add_systems(Update, camera_zoom_system);

    }
}
pub fn start (mut commands: Commands) {
    let camera = commands.spawn(Camera2dBundle::default()).id();
    commands.insert_resource(MainCamera(camera));
    commands.insert_resource(CameraZoom(1.0));
    commands.insert_resource(GamepadState::default());
}

pub fn camera_zoom_system(
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    mut gamepad_event_reader: EventReader<GamepadEvent>,
    main_camera: Res<MainCamera>,
    mut transforms: Query<&mut Transform>,
    mut camera_zoom: ResMut<CameraZoom>,
    mut gamepad_state: ResMut<GamepadState>,
) {
    for event in mouse_wheel_reader.iter() {
        camera_zoom.0 *= 1.0 - event.y * 0.1;
    }

    for event in gamepad_event_reader.iter() {
        if let GamepadEvent::Axis(axis_event) = event {
            if let GamepadAxisType::RightStickY = axis_event.axis_type {
                gamepad_state.right_stick_y = axis_event.value;
            }
        }
    }
    

    if gamepad_state.right_stick_y.abs() > DEADZONE {
        camera_zoom.0 *= 1.0 - gamepad_state.right_stick_y * 0.03;
    }
    
    camera_zoom.0 = camera_zoom.0.clamp(ZOOM_MIN, ZOOM_MAX);

    if let Ok(mut transform) = transforms.get_mut(main_camera.0) {
        transform.scale = Vec3::splat(1.0) * camera_zoom.0;
    }
}