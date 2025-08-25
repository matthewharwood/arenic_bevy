use bevy::prelude::Component;

/// Marker component for Mushroom ability
#[derive(Component, Debug)]
pub struct Mushroom;

impl Mushroom {
    pub fn new() -> Self {
        Self
    }
}
