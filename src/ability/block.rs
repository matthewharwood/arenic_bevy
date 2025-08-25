use bevy::prelude::Component;

/// Marker component for Block ability
#[derive(Component, Debug)]
pub struct Block;

impl Block {
    pub fn new() -> Self {
        Self
    }
}
