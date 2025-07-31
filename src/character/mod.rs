use bevy::prelude::Component;
pub mod hunter_character;
#[derive(Component, Debug)]
pub struct Character {
    name: String,
    health: u32,
    level: u32,
}

impl Character {
    pub fn new(name: String, health: u32, level: u32) -> Self {
        Self {
            name,
            health,
            level,
        }
    }
}
