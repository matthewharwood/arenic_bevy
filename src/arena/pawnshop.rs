use crate::trait_utils::ComponentDisplay;
use bevy::prelude::Component;
use super::ArenaTransform;

#[derive(Component, Debug)]
pub struct Pawnshop;

impl ComponentDisplay for Pawnshop {
    fn name(&self) -> &'static str {
        "Pawnshop"
    }
}

impl ArenaTransform for Pawnshop {
    const INDEX: usize = 5;
}