use bevy::prelude::Component;

/// Marker component for Dice ability
#[derive(Component, Debug)]
pub struct Dice;

impl Dice {
    pub fn new() -> Self {
        Self
    }
}
