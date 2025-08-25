use bevy::prelude::Component;

/// Marker component for Cleanse ability
#[derive(Component, Debug)]
pub struct Cleanse;

impl Cleanse {
    pub fn new() -> Self {
        Self
    }
}
