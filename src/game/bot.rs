use crate::prelude::*;

pub struct BotPlugin;

impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app // Add event with arena.
            // .add_event::<BotSpawnEvent>()
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(spawn_bot));
        // .add_system_set(SystemSet::on_update(AppState::Game).
        // with_system(movement));
    }
}

fn spawn_bot(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = thread_rng();

    for y in 0..4 {
        let column = rng.gen_range(0..3);
        let pos_x: f32 = TileScreen::column_to_coord(column);
        let pos_y: f32 =
            TILE_SIZE.mul_add(CAR_SPACING, (SCREEN_HEIGHT as f32).mul_add(TILE_SIZE, SCREEN_Y));
        let y_distance = y as f32 * TILE_SIZE * CAR_SPACING;

        let mut invincible_timer = InvincibleTimer::new(INVINCIBLE_TIME, TimerMode::Once);
        let invincible_timer: Timer = invincible_timer.tick();
        commands
            .spawn((Car { column, invincible_timer, invincible_time_secs: 0f32 }, MoveY, Bot))
            .with_children(draw_car)
            .insert(anchor_sprite(pos_x, pos_y + y_distance));

        debug_spawn_bot(
            &mut commands,
            &asset_server,
            DebugBotPosition { y, y_distance, pos_x, pos_y },
        );
    }
}

struct DebugBotPosition {
    y: i32,
    y_distance: f32,
    pos_x: f32,
    pos_y: f32,
}

/// Debug to show car numbers.
fn debug_spawn_bot(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    bot_pos: DebugBotPosition,
) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/Calculator.ttf"),
        font_size: 40f32,
        color: Color::WHITE,
    };
    commands
        .spawn(Text2dBundle {
            text: Text::from_section(format!("{}", bot_pos.y), text_style),
            transform: Transform {
                translation: Vec3::new(bot_pos.pos_x, bot_pos.pos_y + bot_pos.y_distance, 1f32),
                ..default()
            },
            ..default()
        })
        .insert(MoveY);
}
