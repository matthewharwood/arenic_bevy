use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct Battleground {
    name: String,
    max_size: usize,
}
impl Battleground {
    pub fn new(name: String, max_size: usize) -> Self {
        Self { name, max_size }
    }
}
