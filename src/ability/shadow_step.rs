use bevy::prelude::Component;

/// Marker component for Shadow Step ability
#[derive(Component, Debug)]
pub struct ShadowStep;

impl ShadowStep {
    pub fn new() -> Self {
        Self
    }
}
