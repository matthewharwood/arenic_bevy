use crate::trait_utils::ComponentDisplay;
use bevy::prelude::Component;
use super::ArenaTransform;

#[derive(Component, Debug)]
pub struct Labyrinth;

impl ComponentDisplay for Labyrinth {
    fn name(&self) -> &'static str {
        "Labyrinth"
    }
}

impl ArenaTransform for Labyrinth {
    const INDEX: usize = 0;
}