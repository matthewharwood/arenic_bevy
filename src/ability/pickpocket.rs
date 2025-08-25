use bevy::prelude::Component;

/// Marker component for Pickpocket ability
#[derive(Component, Debug)]
pub struct Pickpocket;

impl Pickpocket {
    pub fn new() -> Self {
        Self
    }
}
