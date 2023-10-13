use bevy::prelude::*;
use crate::*;

#[derive(Resource)] pub struct ControlledEntity(pub Option<Entity>);
impl Default for ControlledEntity { fn default() -> Self { Self (None) } }

pub fn update(
    gamepad_state: Res<gamepad::GamepadState>,
    controlled_entity: Res<ControlledEntity>,
    mut query: Query<(&Transform, &mut motion::movement::Movement)>,
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