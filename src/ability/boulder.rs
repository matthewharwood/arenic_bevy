use bevy::prelude::Component;

/// Marker component for Boulder ability
#[derive(Component, Debug)]
pub struct Boulder;

impl Boulder {
    pub fn new() -> Self {
        Self
    }
}
