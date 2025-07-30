use super::Boss;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Trapper;

impl Boss for Trapper {
    const NAME: &'static str = "The Trapper";
    const TEXTURE_PATH: &'static str = "bosses/trapper.png";
    const ANIMATION_FPS: f32 = 10.0;
}
