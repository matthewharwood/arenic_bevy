use bevy::prelude::Component;

/// Marker component for Trap ability
#[derive(Component, Debug)]
pub struct Trap;

impl Trap {
    pub fn new() -> Self {
        Self
    }
}
