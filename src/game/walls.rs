use crate::prelude::*;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_walls))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(respawn_wall));

        // with_system(movement));
    }
}

/// Draw left and right wall sprites.
fn draw_walls(parent: &mut ChildBuilder) {
    let sprite =
        Sprite { custom_size: Some(Vec2::splat(TILE_SIZE)), color: TILE_COLOR, ..default() };

    for y in 0..3 {
        let pos_y = (y as f32).mul_add(TILE_SIZE, -HALF_TILE);

        parent.spawn(SpriteBundle {
            sprite: sprite.clone(),
            transform: Transform {
                scale: TileScreen::tile_scale(),
                translation: Vec3::new(LEFT_WALL_X, pos_y, 0f32),
                ..default()
            },
            ..default()
        });

        parent.spawn(SpriteBundle {
            sprite: sprite.clone(),
            transform: Transform {
                scale: TileScreen::tile_scale(),
                translation: Vec3::new(RIGHT_WALL_X, pos_y, 0f32),
                ..default()
            },
            ..default()
        });
    }
}

fn spawn_walls(mut commands: Commands) {
    for y in 0..6 {
        let pos_y = SCREEN_Y + TILE_SIZE;
        let y_distance = y as f32 * TILE_SIZE * WALL_SPACING;

        commands
            .spawn((Wall, MoveY))
            .with_children(draw_walls)
            .insert(anchor_sprite(0f32, pos_y + y_distance));
    }

    // Rapier configuration without gravity.
    // rapier_cfg.gravity = Vec2::ZERO;
}

/// Respawn the walls each time the previous goes out of scope along y axis.
/// Gives the infinite road or walls illusion.
fn respawn_wall(mut query: Query<&mut Transform, With<Wall>>) {
    for mut wall_transform in query.iter_mut() {
        if wall_transform.translation.y < TILE_SIZE.mul_add(WALL_SPACING.neg() + 1f32, PLAYER_Y) {
            wall_transform.translation.y = TILE_SIZE
                .mul_add(WALL_SPACING, (SCREEN_HEIGHT as f32).mul_add(TILE_SIZE, SCREEN_Y));
        }
    }
}
