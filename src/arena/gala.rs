use crate::trait_utils::ComponentDisplay;
use bevy::prelude::Component;
use super::ArenaTransform;

#[derive(Component, Debug)]
pub struct Gala;

impl ComponentDisplay for Gala {
    fn name(&self) -> &'static str {
        "Gala"
    }
}

impl ArenaTransform for Gala {
    const INDEX: usize = 8;
}