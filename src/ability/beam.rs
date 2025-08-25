use bevy::prelude::Component;

/// Marker component for Beam ability
#[derive(Component, Debug)]
pub struct Beam;

impl Beam {
    pub fn new() -> Self {
        Self
    }
}
