use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Movement {
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


pub fn update(time: Res<Time>, mut query: Query<(&mut Transform, &mut Movement)>) {

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