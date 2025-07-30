use crate::trait_utils::ComponentDisplay;
use bevy::prelude::Component;
use super::ArenaTransform;

#[derive(Component, Debug)]
pub struct Bastion;

impl ComponentDisplay for Bastion {
    const NAME: &'static str = "Bastion";
}

impl ArenaTransform for Bastion {
    const INDEX: usize = 4;
}