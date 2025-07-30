use super::Boss;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Alchemist;

impl Boss for Alchemist {
    const NAME: &'static str = "The Alchemist";
    const TEXTURE_PATH: &'static str = "bosses/alchemist.png";
    const ANIMATION_FPS: f32 = 10.0;
}
