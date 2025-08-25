use bevy::prelude::Component;

/// Marker component for Sniper ability
#[derive(Component, Debug)]
pub struct Sniper;

impl Sniper {
    pub fn new() -> Self {
        Self
    }
}
