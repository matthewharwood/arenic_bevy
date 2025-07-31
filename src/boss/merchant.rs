use super::Boss;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct BossMerchant;

impl Boss for BossMerchant {
    const NAME: &'static str = "The Merchant";
    const TEXTURE_PATH: &'static str = "characters/merchant.png";
    const ANIMATION_FPS: f32 = 10.0;
}