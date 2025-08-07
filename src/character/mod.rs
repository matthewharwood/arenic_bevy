use bevy::prelude::Component;
use crate::arena::TILE_SIZE;

/// Marker component for character entities.
#[derive(Component, Debug)]
pub struct Character;

#[derive(Component, Debug)]
pub struct Boss;

#[derive(Component, Debug)]
pub struct AutoShot {
    pub(crate) distance: f32,
}


impl AutoShot {
    pub fn new(dist: f32) -> Self {
        Self {
            distance: (TILE_SIZE * dist).round(),
        }
    }
}
