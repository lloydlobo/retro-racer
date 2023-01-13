use crate::prelude::*;

pub fn anchor_sprite(x: f32, y: f32) -> SpriteBundle {
    let position = Vec2::new(x, y);

    SpriteBundle {
        sprite: Sprite { color: Color::NONE, ..default() },
        transform: Transform { translation: position.extend(0f32), ..default() },
        ..default()
    }
}
