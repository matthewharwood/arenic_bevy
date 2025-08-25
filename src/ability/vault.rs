use bevy::prelude::Component;

/// Marker component for Vault ability
#[derive(Component, Debug)]
pub struct Vault;

impl Vault {
    pub fn new() -> Self {
        Self
    }
}
