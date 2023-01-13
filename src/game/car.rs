use crate::prelude::*;

/// Helper function to draw car shape for player and bot.
pub fn draw_car(parent: &mut ChildBuilder) {
    let sprite =
        Sprite { custom_size: Some(Vec2::splat(TILE_SIZE)), color: TILE_COLOR, ..default() };

    for (y, line) in CAR.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Vec2::new(
                (x as f32).mul_add(TILE_SIZE, -TILE_SIZE),
                (y as f32).mul_add(TILE_SIZE.neg(), TILE_SIZE) + HALF_TILE,
            );

            if c == 'O' {
                parent.spawn(SpriteBundle {
                    sprite: sprite.clone(),
                    transform: Transform {
                        scale: TileScreen::tile_scale(),
                        // Creates a 3D vector from `self` and the given `z` value.
                        translation: Vec2::extend(pos, 0f32),
                        ..default()
                    },
                    ..default()
                });
            }
        }
    }
}
