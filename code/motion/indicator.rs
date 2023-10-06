use bevy::prelude::*;
use crate::motion::movement::Movement;
use crate::motion::destination::ControlledEntity;
use bevy::sprite::MaterialMesh2dBundle;

const Z_INDEX: f32 = 30.;

#[derive(Default, Resource)]
pub struct IndicatorEntity(pub Option<Entity>);

pub fn update(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    controlled_entity: Res<ControlledEntity>,
    query: Query<(Entity, &Movement)>,
    mut indicator: ResMut<IndicatorEntity>,
) {
    if let Some(entity_id) = controlled_entity.0 {
        if let Ok((_, movement)) = query.get(entity_id) {
            match &movement.destination {
                Some(destination) => {
                    if indicator.0.is_none() {
                        let indicator_entity = commands
                            .spawn(MaterialMesh2dBundle {
                                mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                                material: materials.add(ColorMaterial::from(Color::GREEN)),
                                transform: Transform::from_translation(Vec3::new(destination.x, destination.y, Z_INDEX)),
                                ..default()
                            })
                            
                            .id();

                        indicator.0 = Some(indicator_entity);
                    }
                },
                None => {
                    if let Some(indicator_entity) = indicator.0 {
                        commands.entity(indicator_entity).despawn();
                        indicator.0 = None;
                    }
                },
            }
        }
    }
}

pub fn position(
    controlled_entity: Res<ControlledEntity>,
    indicator: Res<IndicatorEntity>,
    query: Query<(Entity, &Movement)>,
    mut transform_query: Query<&mut Transform>,
) {
    if let Some(entity_id) = controlled_entity.0 {
        if let Ok((_, movement)) = query.get(entity_id) {
            if let Some(destination) = movement.destination {
                if let Some(indicator_entity) = indicator.0 {
                    if let Ok(mut transform) = transform_query.get_mut(indicator_entity) {
                        transform.translation.x = destination.x;
                        transform.translation.y = destination.y;
                    }
                }
            }
        }
    }
}