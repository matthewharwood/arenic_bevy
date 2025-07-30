use crate::trait_utils::ComponentDisplay;
use bevy::prelude::Component;
use super::ArenaTransform;

#[derive(Component, Debug)]
pub struct Sanctum;

impl ComponentDisplay for Sanctum {
    const NAME: &'static str = "Sanctum";
}

impl ArenaTransform for Sanctum {
    const INDEX: usize = 2;
}