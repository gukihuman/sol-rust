use bevy::prelude::*;
use crate::collision::CollisionArray;

#[derive(Resource)]
pub struct GridCreated {
    ran: bool,
}


pub fn setup(mut commands: Commands) {
    commands.insert_resource(GridCreated{ ran: false });
}

pub fn spawn(
    mut commands: Commands,
    mut state: ResMut<GridCreated>,
    collision_array: Res<CollisionArray>
) {
    if state.ran { return; }
    state.ran = true;
    
    let square_size = 32.0;
    let gap = 2.0;
    let grid_size = 10;
    for y in 0..grid_size {
        for x in 0..grid_size {
            let position = Vec3::new(
                x as f32 * (square_size + gap), 
                y as f32 * (square_size + gap), 
                0.0
            );
            let color = match collision_array.get(y, x) {
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