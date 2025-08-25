use bevy::prelude::Component;

/// Marker component for Taunt ability
#[derive(Component, Debug)]
pub struct Taunt;

impl Taunt {
    pub fn new() -> Self {
        Self
    }
}
