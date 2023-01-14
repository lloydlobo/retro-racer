mod actions;
mod bot;
mod car;
mod helpers;
mod player;
mod state;
mod walls_arena;

pub use self::{
    actions::*,
    bot::*,
    car::*,
    helpers::*,
    player::*,
    state::*,
    walls_arena::*,
};
use crate::prelude::*;

pub const CAR: [&str; 4] = ["_O_", "OOO", "_O_", "O_O"];

pub const LEFT_WALL_X: f32 = SCREEN_X + HALF_TILE;

pub const RIGHT_WALL_X: f32 = SCREEN_X + SCREEN_WIDTH as f32 * TILE_SIZE;

pub fn accelerate(
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
