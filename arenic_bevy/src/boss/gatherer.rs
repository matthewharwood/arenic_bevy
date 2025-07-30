use super::Boss;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Gatherer;

impl Boss for Gatherer {
    const NAME: &'static str = "The Gatherer";
    const TEXTURE_PATH: &'static str = "bosses/gatherer.png";
    const ANIMATION_FPS: f32 = 10.0;
}
