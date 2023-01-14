use crate::prelude::*;

pub fn anchor_sprite(x: f32, y: f32) -> SpriteBundle {
    let position = Vec2::new(x, y);

    SpriteBundle {
        sprite: Sprite { color: Color::NONE, ..default() },
        transform: Transform { translation: position.extend(0f32), ..default() },
        ..default()
    }
}

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
