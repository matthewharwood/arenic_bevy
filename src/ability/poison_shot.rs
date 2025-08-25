use bevy::prelude::Component;

/// Marker component for Poison Shot ability
#[derive(Component, Debug)]
pub struct PoisonShot;

impl PoisonShot {
    pub fn new() -> Self {
        Self
    }
}
