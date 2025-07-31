use super::Boss;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct BossBard;

impl Boss for BossBard {
    const NAME: &'static str = "The Bard";
    const TEXTURE_PATH: &'static str = "bosses/bard.png";
    const ANIMATION_FPS: f32 = 10.0; // Fast animation for the bard
}
