mod bot;
mod car;
mod helpers;
mod player;
mod state;
mod walls;

pub use self::{
    bot::*,
    car::*,
    helpers::*,
    player::*,
    state::*,
    walls::*,
};
use crate::prelude::*;

pub const CAR: [&str; 4] = ["_O_", "OOO", "_O_", "O_O"];

pub const LEFT_WALL_X: f32 = SCREEN_X + HALF_TILE;

pub const RIGHT_WALL_X: f32 = SCREEN_X + SCREEN_WIDTH as f32 * TILE_SIZE;
