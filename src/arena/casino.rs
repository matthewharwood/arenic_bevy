use crate::trait_utils::ComponentDisplay;
use bevy::prelude::Component;
use super::ArenaTransform;

#[derive(Component, Debug)]
pub struct Casino;

impl ComponentDisplay for Casino {
    const NAME: &'static str = "Casino";
}

impl ArenaTransform for Casino {
    const INDEX: usize = 7;
}