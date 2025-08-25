use bevy::prelude::Component;

/// Marker component for Dig ability
#[derive(Component, Debug)]
pub struct Dig;

impl Dig {
    pub fn new() -> Self {
        Self
    }
}
