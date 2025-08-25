use bevy::prelude::Component;

/// Marker component for Helix ability
#[derive(Component, Debug)]
pub struct Helix;

impl Helix {
    pub fn new() -> Self {
        Self
    }
}
