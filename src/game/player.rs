use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app // Add event with arena.
            // .add_event::<PlayerSpawnEvent>()
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_player))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(input_player));
        // .add_system_set(SystemSet::on_update(AppState::Game).
        // with_system(movement));
    }
}
fn spawn_player(mut commands: Commands) {
    let column = 1;
    let pos_x = TileScreen::column_to_coord(column);

    commands
        .spawn((Car { column }, Player))
        .with_children(draw_car)
        .insert(anchor_sprite(pos_x, PLAYER_Y));
}

/// Moves the player.
fn input_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Car, &mut Transform), With<Player>>,
    gamestate: Res<State<AppGameState>>,
) {
    if *gamestate.current() != AppGameState::Game {
        return;
    }

    let (mut car, mut player_transform) = query.single_mut();
    let mut direction = 0f32;

    if keyboard_input.any_just_pressed([KeyCode::Left, KeyCode::A]) && car.column > 0usize {
        direction -= COLUMN_SIZE;
        car.column -= 1;
    }
    if keyboard_input.any_just_pressed([KeyCode::Right, KeyCode::D]) && car.column < 2usize {
        direction += COLUMN_SIZE;
        car.column += 1;
    }

    player_transform.translation.x += direction;
}
