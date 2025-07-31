use super::Boss;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct BossWarrior;

impl Boss for BossWarrior {
    const NAME: &'static str = "The Warrior";
    const TEXTURE_PATH: &'static str = "bosses/warrior.png";
    const ANIMATION_FPS: f32 = 10.0; // Slower animation for the heavy warrior
}