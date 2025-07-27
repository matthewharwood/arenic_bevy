use crate::trait_utils::ComponentDisplay;
use bevy::prelude::Component;
use super::ArenaTransform;

#[derive(Component, Debug)]
pub struct GuildHouse;

impl ComponentDisplay for GuildHouse {
    fn name(&self) -> &'static str {
        "GuildHouse"
    }
}

impl ArenaTransform for GuildHouse {
    const INDEX: usize = 1;
}