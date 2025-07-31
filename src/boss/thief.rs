use super::Boss;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct BossThief;

impl Boss for BossThief {
    const NAME: &'static str = "The Thief";
    const TEXTURE_PATH: &'static str = "bosses/thief.png";
    const ANIMATION_FPS: f32 = 10.0; // Quick, sneaky movements
}
