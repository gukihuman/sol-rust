use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::motion::movement::Movement;
use crate::core::camera::CameraFollowedEntity;
use crate::motion::destination::ControlledEntity;


const Z_INDEX: f32 = 999.;
const RADIUS: f32 = 10.5;

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut followed_entity: ResMut<CameraFollowedEntity>,
    mut controlled_entity: ResMut<ControlledEntity>,
) {
    let crosshair = commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(Color::ALICE_BLUE)),
            transform: Transform::from_translation(Vec3::new(0., 0., Z_INDEX)),
            ..default()
        },
        Movement::default()
    )).id();
    followed_entity.0 = Some(crosshair);
    controlled_entity.0 = Some(crosshair);
}