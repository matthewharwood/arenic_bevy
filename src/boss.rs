use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct Boss {
    name: String,
    health: u32,
}

impl Boss {
    pub fn new(name: String, health: u32) -> Self {
        Self { name, health }
    }
}
