use bevy::prelude::Component;

/// Marker component for Dance ability
#[derive(Component, Debug)]
pub struct Dance;

impl Dance {
    pub fn new() -> Self {
        Self
    }
}
