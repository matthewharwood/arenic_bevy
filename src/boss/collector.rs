use super::Boss;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Collector;

impl Boss for Collector {
    const NAME: &'static str = "The Collector";
    const TEXTURE_PATH: &'static str = "bosses/collector.png";
    const ANIMATION_FPS: f32 = 10.0;
}
