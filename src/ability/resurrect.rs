use bevy::prelude::Component;

/// Marker component for Resurrect ability
#[derive(Component, Debug)]
pub struct Resurrect;

impl Resurrect {
    pub fn new() -> Self {
        Self
    }
}
