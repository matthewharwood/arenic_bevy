use crate::trait_utils::ComponentDisplay;
use bevy::prelude::Component;
use super::ArenaTransform;

#[derive(Component, Debug)]
pub struct Mountain;

impl ComponentDisplay for Mountain {
    fn name(&self) -> &'static str {
        "Mountain"
    }
}

impl ArenaTransform for Mountain {
    const INDEX: usize = 3;
}