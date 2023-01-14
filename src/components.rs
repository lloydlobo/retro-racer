pub use crate::prelude::*;

#[derive(Component, Debug)]
pub struct ForState<T> {
    pub states: Vec<T>,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Bot;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct MoveY;

#[derive(Component)]
pub struct Car {
    pub column: usize,
    pub invincible_timer: Timer,
    pub invincible_time_secs: f32,
}

#[derive(Default)]
pub struct CollisionEvent;
