mod actions;
mod bot;
mod car;
mod helpers;
mod player;
mod state;
mod walls;

pub use self::{
    actions::*,
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

pub struct InvincibleTimer {
    time: f32,
    mode: TimerMode,
}

impl InvincibleTimer {
    pub const fn new(time: f32, mode: TimerMode) -> Self {
        Self { time, mode }
    }

    pub fn timer(&mut self) -> Timer {
        Timer::from_seconds(self.time, self.mode)
    }

    pub fn tick(&mut self) -> Timer {
        let mut timer = Self::timer(self);
        timer.tick(Duration::from_secs_f32(self.time));
        timer
    }
}
