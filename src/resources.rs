use crate::prelude::*;

// ------------------------------------------------------------------------

#[derive(Default)]
pub struct ScoreEntities {
    pub score: Option<Entity>,
    pub highscore: Option<Entity>,
}

// ------------------------------------------------------------------------

#[derive(Default, Resource)]
pub struct Scoreboard {
    pub score: usize,
    pub highscore: usize,
    pub entities: ScoreEntities,
}

// ------------------------------------------------------------------------

#[derive(Resource)]
pub struct GameData {
    pub move_timer: Timer,
    pub is_boosting: bool,
    pub boost_factor: f32,
    pub speed_factor: f32,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            move_timer: Timer::from_seconds(0.08f32, TimerMode::Repeating),
            is_boosting: false,
            boost_factor: 2f32,
            speed_factor: 1f32,
        }
    }
}

impl Default for GameData {
    fn default() -> Self {
        Self::new()
    }
}

// ------------------------------------------------------------------------

#[derive(Resource)]
pub struct ExplosionSound(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct MotorSound(pub Handle<AudioSource>);

// ------------------------------------------------------------------------
