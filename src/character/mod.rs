use bevy::input::ButtonInput;
use bevy::prelude::{Component, KeyCode, Res};

/// Marker component for character entities.
#[derive(Component, Debug)]
pub struct Character;

#[derive(Component, Debug)]
pub struct Boss;

pub fn toggle_active_character(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if !keyboard_input.just_pressed(KeyCode::Tab) {
        return;
    }
}
