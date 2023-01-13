use crate::prelude::*;

const CAR: [&str; 4] = ["_O_", "OOO", "_O_", "O_O"];

const LEFT_WALL_X: f32 = SCREEN_X + HALF_TILE;
const RIGHT_WALL_X: f32 = SCREEN_X + SCREEN_WIDTH as f32 * TILE_SIZE;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_walls));
        // .add_system_set(SystemSet::on_update(AppState::Game).with_system(movement));
    }
}

/// Draw left and right wall sprites.
fn draw_walls(parent: &mut ChildBuilder) {
    let sprite = Sprite {
        custom_size: Some(Vec2::splat(TILE_SIZE)),
        color: TILE_COLOR,
        ..default()
    };

    for y in 0..3 {
        let pos_y = y as f32 * TILE_SIZE - HALF_TILE;

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
