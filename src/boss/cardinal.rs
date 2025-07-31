use super::Boss;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct BossCardinal;

impl Boss for BossCardinal {
    const NAME: &'static str = "The Cardinal";
    const TEXTURE_PATH: &'static str = "bosses/cardinal.png";
    const ANIMATION_FPS: f32 = 10.0;
}
