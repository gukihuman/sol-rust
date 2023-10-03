use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(2.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::ALICE_BLUE)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });
}