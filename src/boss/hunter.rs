use super::Boss;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct BossHunter;

impl Boss for BossHunter {
    const NAME: &'static str = "The Hunter";
    const TEXTURE_PATH: &'static str = "bosses/hunter.png";
    const ANIMATION_FPS: f32 = 10.0;
}