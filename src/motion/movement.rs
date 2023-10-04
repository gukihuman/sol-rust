use bevy::prelude::*;

#[derive(Component)]
pub struct Movement {
    speed: f32,
    current_velocity: Vec2,
    destination: Vec2,
}
impl Default for Movement {
    fn default() -> Self {
        Self {
            speed: 5.,
            current_velocity: Vec2::default(),
            destination: Vec2 {x: 50., y: 50.},
        }
    }
}

// 📜 smooth vectors
// const ADJUSTMENT_RATE: f32 = 1.;
const CLOSE_ENOUGH: f32 = 10.;

pub fn update(time: Res<Time>, mut query: Query<(&mut Transform, &mut Movement)>) {
    let delta_seconds = time.delta_seconds();
    for (mut transform, mut movement) in query.iter_mut() {
        let desired_direction = (movement.destination - Vec2::new(transform.translation.x, transform.translation.y)).normalize();

        let distance_to_destination = (movement.destination - Vec2::new(transform.translation.x, transform.translation.y)).length();

        let dynamic_speed = if distance_to_destination < CLOSE_ENOUGH {
            movement.speed * (distance_to_destination / CLOSE_ENOUGH)
        } else {
            movement.speed
        };

        movement.current_velocity = desired_direction * dynamic_speed;

        transform.translation.x += movement.current_velocity.x * delta_seconds;
        transform.translation.y += movement.current_velocity.y * delta_seconds;
    }
}