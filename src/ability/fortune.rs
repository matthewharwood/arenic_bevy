use bevy::prelude::Component;

/// Marker component for Fortune ability
#[derive(Component, Debug)]
pub struct Fortune;

impl Fortune {
    pub fn new() -> Self {
        Self
    }
}
