use bevy::prelude::Component;

/// Marker component for Ironskin ability
#[derive(Component, Debug)]
pub struct Ironskin;

impl Ironskin {
    pub fn new() -> Self {
        Self
    }
}
