use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::motion::movement::Movement;
use crate::core::camera::CameraFollowedEntity;

const Z_INDEX: f32 = 30.;

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut followed_entity: ResMut<CameraFollowedEntity>
) {
    let crosshair = commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(2.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::ALICE_BLUE)),
            transform: Transform::from_translation(Vec3::new(0., 0., Z_INDEX)),
            ..default()
        },
        Movement::default()
    )).id();
    followed_entity.0 = Some(crosshair);
}