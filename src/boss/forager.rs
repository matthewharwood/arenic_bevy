use super::Boss;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct BossForager;

impl Boss for BossForager {
    const NAME: &'static str = "The Forager";
    const TEXTURE_PATH: &'static str = "bosses/forager.png";
    const ANIMATION_FPS: f32 = 10.0;
}