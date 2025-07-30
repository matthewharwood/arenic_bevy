use crate::trait_utils::ComponentDisplay;
use bevy::prelude::Component;
use super::ArenaTransform;

#[derive(Component, Debug)]
pub struct Labyrinth;

impl ComponentDisplay for Labyrinth {
    const NAME: &'static str = "Labyrinth";
}

impl ArenaTransform for Labyrinth {
    const INDEX: usize = 0;
}