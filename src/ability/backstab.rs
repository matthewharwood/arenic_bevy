use bevy::prelude::Component;

/// Marker component for Backstab ability
#[derive(Component, Debug)]
pub struct Backstab;

impl Backstab {
    pub fn new() -> Self {
        Self
    }
}
