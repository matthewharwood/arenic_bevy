use bevy::prelude::Component;

/// Marker component for Barrier ability
#[derive(Component, Debug)]
pub struct Barrier;

impl Barrier {
    pub fn new() -> Self {
        Self
    }
}
