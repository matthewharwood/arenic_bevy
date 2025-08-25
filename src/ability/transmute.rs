use bevy::prelude::Component;

/// Marker component for Transmute ability
#[derive(Component, Debug)]
pub struct Transmute;

impl Transmute {
    pub fn new() -> Self {
        Self
    }
}
