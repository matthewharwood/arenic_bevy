use bevy::prelude::Component;

/// Marker component for Siphon ability
#[derive(Component, Debug)]
pub struct Siphon;

impl Siphon {
    pub fn new() -> Self {
        Self
    }
}
