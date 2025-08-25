use bevy::prelude::Component;

/// Marker component for Smoke Screen ability
#[derive(Component, Debug)]
pub struct SmokeScreen;

impl SmokeScreen {
    pub fn new() -> Self {
        Self
    }
}
