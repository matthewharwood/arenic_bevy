use bevy::prelude::Component;

/// Marker component for Border ability
#[derive(Component, Debug)]
pub struct Border;

impl Border {
    pub fn new() -> Self {
        Self
    }
}
