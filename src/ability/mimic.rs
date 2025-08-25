use bevy::prelude::Component;

/// Marker component for Mimic ability
#[derive(Component, Debug)]
pub struct Mimic;

impl Mimic {
    pub fn new() -> Self {
        Self
    }
}
