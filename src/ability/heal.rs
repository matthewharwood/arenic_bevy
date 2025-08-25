use bevy::prelude::Component;

/// Marker component for Heal ability
#[derive(Component, Debug)]
pub struct Heal;

impl Heal {
    pub fn new() -> Self {
        Self
    }
}
