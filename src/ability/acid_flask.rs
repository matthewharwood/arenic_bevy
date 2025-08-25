use bevy::prelude::Component;

/// Marker component for Acid Flask ability
#[derive(Component, Debug)]
pub struct AcidFlask;

impl AcidFlask {
    pub fn new() -> Self {
        Self
    }
}
