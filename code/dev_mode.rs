use bevy::{
    prelude::*,
    input::common_conditions::input_toggle_active,
    sprite::MaterialMesh2dBundle
};
use bevy_screen_diagnostics::{
    ScreenDiagnosticsPlugin,
    ScreenFrameDiagnosticsPlugin
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::{ env, path::Path };
use crate::*;
const COLLISION_GRID_Z_INDEX: f32 = 20.0;
const CROSSHAIR_Z_INDEX: f32 = 999.;
const CROSSHAIR_RADIUS: f32 = 10.5;
pub struct DevModePlugin;
impl Plugin for DevModePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                WorldInspectorPlugin::default()
                    .run_if(
                        input_toggle_active(false, KeyCode::N)
                    ),
                ScreenDiagnosticsPlugin::default(),
                ScreenFrameDiagnosticsPlugin
                ))
            .add_systems(Startup, convert_assets)
            .add_systems(
                OnEnter(start_bundle::AppState::InGame),
                (spawn_crosshair, spawn_collision_grid)
            )
        ;
    }
}
pub fn spawn_collision_grid(
    mut commands: Commands,
    collision_array: Res<core::collision::CollisionArray>
) {
    let square_size = 32.0;
    let gap = 2.0;
    let grid_size = 10;
    for y in 0..grid_size {
        for x in 0..grid_size {
            let position = Vec3::new(
                x as f32 * (square_size + gap), 
                y as f32 * (square_size + gap), 
                COLLISION_GRID_Z_INDEX
            );
            let color = match collision_array.get_tile(y, x) {
                Some(&0) => Color::rgb(1.0, 0.5, 0.5), // FreeToMove
                Some(&1) => Color::rgb(0.5, 1.0, 0.5), // EntityCollision
                Some(&2) => Color::rgb(0.5, 0.5, 1.0), // ProjectileCollision
                _ => Color::WHITE,
            };
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(square_size, square_size)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            });
        }
    }
}
pub fn spawn_crosshair(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut camera: ResMut<core::camera::Camera>,
    mut controlled_entity: ResMut<motion::destination::ControlledEntity>,
) {
    let crosshair = commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(
                shape::Circle::new(CROSSHAIR_RADIUS).into()
            ).into(),
            material: materials.add(ColorMaterial::from(Color::ALICE_BLUE)),
            transform: Transform::from_translation(Vec3::new(0., 0., CROSSHAIR_Z_INDEX)),
            ..default()

        },
        motion::movement::Movement::default(),
    )).id();
    camera.followed_entity = Some(crosshair);
    controlled_entity.0 = Some(crosshair);
}
pub fn convert_assets() {
    // 📜 adapt it to other files
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let input_path = Path::new(&manifest_dir).join("whales/icon.png");
    let output_path = Path::new(&manifest_dir).join("icon.ico");
    let cache_path = Path::new(&manifest_dir).join("whales/cache.json");

    if !static_convert::cache_check(&input_path, &cache_path) {
        if static_convert::convert_icon_png_to_ico(&input_path, &output_path) {
            static_convert::update_cache(&input_path, &cache_path);
        }
    }
}