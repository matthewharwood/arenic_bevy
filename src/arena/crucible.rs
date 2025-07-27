use crate::trait_utils::ComponentDisplay;
use bevy::prelude::Component;
use super::ArenaTransform;

#[derive(Component, Debug)]
pub struct Crucible;

impl ComponentDisplay for Crucible {
    const NAME: &'static str = "Crucible";
}

impl ArenaTransform for Crucible {
    const INDEX: usize = 6;
}