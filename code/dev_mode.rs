use bevy::{ prelude::*, sprite::MaterialMesh2dBundle };
use crate::*;
use std::{ env, path::Path, fs::{self, File}, io::{self, BufWriter} };
use sha2::{Digest, Sha256};
use ico::{IconDir, IconDirEntry, IconImage, ResourceType};
use serde_json::{json, Value};
const COLLISION_GRID_Z_INDEX: f32 = 20.0;
const CROSSHAIR_Z_INDEX: f32 = 999.;
const CROSSHAIR_RADIUS: f32 = 2.5;
const CROSSHAIR_COLOR: Color = Color::rgb(0.69, 0.91, 0.882);
pub struct DevModePlugin;
impl Plugin for DevModePlugin { fn build(&self, app: &mut App) { app
    .add_systems(Startup, convert_assets)
    .add_systems(
        OnEnter(core::GameState::InGame),
        (spawn_crosshair, spawn_collision_grid)
    )
;}}
pub fn spawn_collision_grid(
    mut commands: Commands,
    collision_array: Res<collision::CollisionArray>
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
    mut camera: ResMut<camera::Camera>,
    mut controlled_entity: ResMut<movement::ControlledEntity>,
) {
    let crosshair = commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(
                shape::Circle::new(CROSSHAIR_RADIUS).into()
            ).into(),
            material: materials.add(ColorMaterial::from(CROSSHAIR_COLOR)),
            transform: Transform::from_translation(Vec3::new(0., 0., CROSSHAIR_Z_INDEX)),
            ..default()

        },
        movement::Movement::default(),
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

    if !convert_cache_check(&input_path, &cache_path) {
        if convert_icon_png_to_ico(&input_path, &output_path) {
            convert_cache_update(&input_path, &cache_path);
        }
    }
}
pub fn convert_icon_png_to_ico(src_path: &Path, dst_path: &Path) -> bool {
    let img = image::open(&src_path).expect("Failed to open icon.png");
    let img = img.into_rgba8();
    let icon_img = create_icon_image(img);
    let icon_dir_entry = IconDirEntry::encode(&icon_img)
        .expect("Failed to encode to IconDirEntry");

    let mut icon_dir = IconDir::new(ResourceType::Icon);
    icon_dir.add_entry(icon_dir_entry);

    let file = File::create(&dst_path)
        .expect("Failed to create output file");
    let mut writer = BufWriter::new(file);

    icon_dir.write(&mut writer).expect("Failed to write icon");

    true
}
fn create_icon_image(
    image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>
) -> IconImage {
    let width = image.width();
    let height = image.height();
    let data = image.into_raw();
    IconImage::from_rgba_data(width, height, data)
}
pub fn convert_cache_check(src_path: &Path, cache_path: &Path)-> bool {
    let mut hasher = Sha256::new();
    let mut src = File::open(&src_path).expect("Failed to open icon.png");
    let _ = io::copy(&mut src, &mut hasher);
    let current_hash = format!("{:x}", hasher.finalize());

    let cache: Value = serde_json::from_str(
        &fs::read_to_string(&cache_path).unwrap_or("{}".into())
    ).unwrap();
    if let Some(previous_hash) = cache["hash"].as_str() {
        if previous_hash == current_hash {
            return true;
        }
    }
    false
}
pub fn convert_cache_update(src_path: &Path, cache_path: &Path) {
    let mut hasher = Sha256::new();
    let mut src = File::open(&src_path).expect("Failed to open icon.png");
    let _ = io::copy(&mut src, &mut hasher);
    let current_hash = format!("{:x}", hasher.finalize());

    let new_cache = json!({ "hash": current_hash });
    fs::write(&cache_path, new_cache.to_string())
        .expect("Failed to write cache file");
}
