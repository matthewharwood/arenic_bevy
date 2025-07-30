use super::Boss;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Tank;

impl Boss for Tank {
    const NAME: &'static str = "The Tank";
    const TEXTURE_PATH: &'static str = "bosses/tank.png";
    const ANIMATION_FPS: f32 = 10.0; // Slower animation for the heavy tank
}
