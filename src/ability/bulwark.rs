use bevy::prelude::Component;

/// Marker component for Bulwark ability
#[derive(Component, Debug)]
pub struct Bulwark;

impl Bulwark {
    pub fn new() -> Self {
        Self
    }
}
