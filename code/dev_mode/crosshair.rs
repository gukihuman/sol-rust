use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::*;
const Z_INDEX: f32 = 999.;
const RADIUS: f32 = 10.5;
pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut followed_entity: ResMut<core::camera::CameraFollowedEntity>,
    mut controlled_entity: ResMut<motion::destination::ControlledEntity>,
) {
    let crosshair = commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(Color::ALICE_BLUE)),
            transform: Transform::from_translation(Vec3::new(0., 0., Z_INDEX)),
            ..default()

        },
        motion::movement::Movement::default(),
        core::camera::CameraFollow
    )).id();
    followed_entity.0 = Some(crosshair);
    controlled_entity.0 = Some(crosshair);
}