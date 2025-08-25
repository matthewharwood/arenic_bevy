use bevy::prelude::Component;

/// Marker component for Bash ability
#[derive(Component, Debug)]
pub struct Bash;

impl Bash {
    pub fn new() -> Self {
        Self
    }
}
