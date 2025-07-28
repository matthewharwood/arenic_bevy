use super::Boss;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Sprinter;

impl Boss for Sprinter {
    const NAME: &'static str = "The Sprinter";
    const TEXTURE_PATH: &'static str = "bosses/sprinter.png";
    const ANIMATION_FPS: f32 = 10.0; // Fast animation for the sprinter
}
