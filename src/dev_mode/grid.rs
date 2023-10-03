use bevy::prelude::*;
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    FreeToMove,
    EntityCollision,
    ProjectileCollision,
}
pub fn spawn(mut commands: Commands) {
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
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1., 0.5, 0.5),
                    custom_size: Some(Vec2::new(square_size, square_size)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            });
        }
    }
}