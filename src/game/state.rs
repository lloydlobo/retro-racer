use crate::prelude::*;
use bevy::ecs::schedule::StateData;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum AppState {
    StartMenu,
    Game,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum AppGameState {
    Invalid,
    Game,
    Pause,
    GameOver,
}
