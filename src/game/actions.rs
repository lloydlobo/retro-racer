use crate::prelude::*;

/// Actions are divided in two enumerations:
/// * One for pure Player Ship actions, during effective gameplay, added on the
///   player entity itself.
/// * One for Menu actions, added as a global resource
#[derive(Actionlike, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerAction {
    Forward,
    Backward,
    Left,
    Right,
    Fire,
    Boost,
}

pub fn get_input_map<T>() -> InputMap<PlayerAction> {
    let mut input_map = InputMap::new([
        (KeyCode::W, PlayerAction::Forward),
        (KeyCode::Up, PlayerAction::Forward),
        (KeyCode::A, PlayerAction::Left),
        (KeyCode::Left, PlayerAction::Left),
        (KeyCode::D, PlayerAction::Right),
        (KeyCode::Right, PlayerAction::Right),
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
        .insert(SingleAxis::positive_only(GamepadAxisType::LeftStickX, 0.4f32), PlayerAction::Right)
        .insert(
            SingleAxis::negative_only(GamepadAxisType::LeftStickX, 0.4f32.neg()),
            PlayerAction::Left,
        );

    input_map
}
