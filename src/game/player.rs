use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app // Add event with arena.
            .add_plugin(InputManagerPlugin::<PlayerAction>::default())
            // .add_event::<PlayerSpawnEvent>()
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    // .with_system(player_input_system_legacy)
                    .with_system(player_input_system_latest)
                    .with_system(boost_player)
                    .with_system(accelerate)
                    .with_system(player_invincible_color),
            );
        // .add_system_set(SystemSet::on_update(AppState::Game).
        // with_system(movement));
    }
}

/// Draw invincibility aura around player car to show the change.
/// Useful when player is hit and needs time to recover.
fn player_invincible_color(mut cars: Query<(&Car, &mut Sprite)>) {
    for (car, mut car_sprite) in cars.iter_mut() {
        if car.invincible_timer.finished() {
            car_sprite.color = Color::WHITE;
        } else {
            let alpha = (car.invincible_timer.elapsed_secs() * 2f32) % 1f32;
            car_sprite.color = Color::rgba(1f32, 0.4f32, 0.2f32, alpha);
        }
    }
}
// app.add_plugin(InputManagerPlugin::<PlayerAction>::default());
fn spawn_player(mut commands: Commands) {
    let column = 1usize;
    let pos_x: f32 = TileScreen::column_to_coord(column);

    let mut input_map = InputMap::new([
        (KeyCode::W, PlayerAction::Forward),
        (KeyCode::Up, PlayerAction::Forward),
        (KeyCode::A, PlayerAction::RotateLeft),
        (KeyCode::Left, PlayerAction::RotateLeft),
        (KeyCode::D, PlayerAction::RotateRight),
        (KeyCode::Right, PlayerAction::RotateRight),
        (KeyCode::Space, PlayerAction::Boost),
    ]);
    input_map
        .insert(GamepadButtonType::South, PlayerAction::Boost)
        .insert(
            SingleAxis::positive_only(GamepadAxisType::LeftStickY, 0.4f32),
            PlayerAction::Forward,
        )
        .insert(
            SingleAxis::negative_only(GamepadAxisType::LeftStickY, 0.4f32.neg()),
            PlayerAction::Forward,
        )
        .insert(
            SingleAxis::positive_only(GamepadAxisType::LeftStickX, 0.4f32),
            PlayerAction::RotateRight,
        )
        .insert(
            SingleAxis::negative_only(GamepadAxisType::LeftStickX, 0.4f32.neg()),
            PlayerAction::RotateLeft,
        );

    // let mut invincible_timer = Timer::from_seconds(INVINCIBLE_TIME,
    // TimerMode::Once); // Immediately consume the timer, we don't want
    // invincibility at creation. invincible_timer.
    // tick(Duration::from_secs_f32(INVINCIBLE_TIME));
    let mut invincible_timer = InvincibleTimer::new(INVINCIBLE_TIME, TimerMode::Once);
    let invincible_timer: Timer = invincible_timer.tick();

    commands
        .spawn((
            Car { column, invincible_timer, invincible_time_secs: 0f32 },
            Player,
            ForState { states: vec![AppState::Game] },
            InputManagerBundle::<PlayerAction> { action_state: ActionState::default(), input_map },
        ))
        .with_children(draw_car)
        .insert(anchor_sprite(pos_x, PLAYER_Y));
}

/// Moves the player.
fn player_input_system_legacy(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&ActionState<PlayerAction>, &mut Car, &mut Transform), With<Player>>,
    gamestate: Res<State<AppGameState>>,
) {
    if *gamestate.current() != AppGameState::Game {
        return;
    }

    let (_action_state, mut car, mut player_transform) = query.single_mut();
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

fn player_input_system_latest(
    gamestate: Res<State<AppGameState>>,
    mut query: Query<(&ActionState<PlayerAction>, &mut Transform, &mut Car), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if *gamestate.current() != AppGameState::Game {
        return;
    }

    for (action_state, mut player_transform, mut car) in query.iter_mut() {
        let mut direction = 0f32;

        if action_state.just_pressed(PlayerAction::RotateLeft) && car.column > 0usize {
            direction -= COLUMN_SIZE;
            car.column -= 1usize;
        }
        if action_state.just_pressed(PlayerAction::RotateRight) && car.column < 2usize {
            direction += COLUMN_SIZE;
            car.column += 1usize;
        }

        player_transform.translation.x += direction;
    }
}

fn accelerate(
    mut query: Query<&mut Transform, With<MoveY>>,
    mut game_data: ResMut<GameData>,
    timer: Res<Time>,
) {
    let delta: Duration = timer.delta();
    let boost_factor: f32 = game_data.boost_factor;
    let speed_factor: f32 = game_data.speed_factor;

    if game_data.is_boosting {
        game_data.move_timer.tick(delta.mul_f32(boost_factor * speed_factor));
    } else {
        game_data.move_timer.tick(delta.mul_f32(speed_factor));
    }

    if !game_data.move_timer.finished() {
        return;
    }

    for mut entity_transform in query.iter_mut() {
        entity_transform.translation.y -= TILE_SIZE;
    }
}

fn boost_player(
    _gamestate: Res<State<AppGameState>>,
    mut query: Query<(&ActionState<PlayerAction>, &mut Transform, &mut Car), With<Player>>,
    mut game_data: ResMut<GameData>,
) {
    for (action_state, _player_transform, _car) in query.iter_mut() {
        if action_state.just_pressed(PlayerAction::Boost) {
            game_data.is_boosting = true;
        }
        if action_state.just_released(PlayerAction::Boost) {
            game_data.is_boosting = false;
        }
    }
}
