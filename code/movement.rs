use bevy::{ prelude::*, sprite::MaterialMesh2dBundle };
use crate::*;
const INDICATOR_Z_INDEX: f32 = 30.0;
#[derive(Resource, Default)] pub struct ControlledEntity(pub Option<Entity>);
#[derive(Resource, Default)] pub struct IndicatorEntity(pub Option<Entity>);
pub struct MotionPlugin;
impl Plugin for MotionPlugin { fn build(&self, app: &mut App) { app
    .init_resource::<movement::ControlledEntity>()
    .init_resource::<movement::IndicatorEntity>()
    .add_systems(Update, (
        update_go,
        update_destination,
        update_indicator
    ))
;}}
#[derive(Component)] pub struct Movement {
    pub speed: f32,
    pub velocity_change_rate: f32,
    pub close_enough: f32,
    pub current_velocity: Vec2,
    pub destination: Option<Vec2>,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            speed: 100.,
            velocity_change_rate: 7.,
            close_enough: 100.,
            current_velocity: Vec2::default(),
            destination: None,
        }
    }
}


pub fn update_go(time: Res<Time>, mut query: Query<(&mut Transform, &mut Movement)>) {

    let delta_seconds = time.delta_seconds();
    for (mut transform, mut movement) in query.iter_mut() {
        if let Some(destination) = movement.destination {
            let dir_to_dest = destination - Vec2::new(transform.translation.x, transform.translation.y);
            let desired_direction = if dir_to_dest.length() == 0.0 { 
                Vec2::default()  
            } else { 
                dir_to_dest.normalize() 
            };
            
            let distance_to_destination = dir_to_dest.length();

            if distance_to_destination < 1. {
                movement.destination = None;
            }

            let dynamic_speed = if distance_to_destination < movement.close_enough {
                0.2 * movement.speed + 0.8 * movement.speed * (distance_to_destination / movement.close_enough)
            } else {
                movement.speed
            };
            
            let desired_velocity = desired_direction * dynamic_speed;
            
            let adjustment = (desired_velocity - movement.current_velocity) * movement.velocity_change_rate;
            movement.current_velocity = movement.current_velocity + adjustment * delta_seconds;

            transform.translation.x += movement.current_velocity.x * delta_seconds;
            transform.translation.y += movement.current_velocity.y * delta_seconds;
        } else if movement.current_velocity.length() > 1. {
            let adjustment = -movement.current_velocity * movement.velocity_change_rate;
            movement.current_velocity = movement.current_velocity + adjustment * delta_seconds;

            transform.translation.x += movement.current_velocity.x * delta_seconds;
            transform.translation.y += movement.current_velocity.y * delta_seconds;

            if movement.current_velocity.length() < 1. {
                movement.current_velocity = Vec2::default();
            }
        }
    }
}
pub fn update_destination(
    gamepad_state: Res<gamepad::GamepadState>,
    controlled_entity: Res<ControlledEntity>,
    mut query: Query<(&Transform, &mut Movement)>,
) {
    if let Some(entity) = controlled_entity.0 {
        if let Ok((transform, mut movement)) = query.get_mut(entity) {
            if gamepad_state.left_stick_deadzone_exceed {
                let change_in_direction = Vec2::new(gamepad_state.left_stick_x, gamepad_state.left_stick_y) * movement.speed;
                movement.destination = Some(Vec2::new(
                    transform.translation.x + change_in_direction.x,
                    transform.translation.y + change_in_direction.y
                ))
            } else {
                movement.destination = None
            }
        }
    }
}
pub fn update_indicator(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    controlled_entity: Res<ControlledEntity>,
    query: Query<(Entity, &Movement)>,
    mut indicator: ResMut<IndicatorEntity>,
    mut transform_query: Query<&mut Transform>,
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
                                transform: Transform::from_translation(Vec3::new(destination.x, destination.y, INDICATOR_Z_INDEX)),
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