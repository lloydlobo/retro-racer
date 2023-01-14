use crate::prelude::*;

/// Actions are divided in two enumerations:
/// * One for pure Player Ship actions, during effective gameplay, added on the
///   player entity itself.
/// * One for Menu actions, added as a global resource
#[derive(Actionlike, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerAction {
    Forward,
    Backward,
    // Left,
    // Right,
    RotateLeft,
    RotateRight,
    Fire,
    Boost,
}
