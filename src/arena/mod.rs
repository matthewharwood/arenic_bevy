mod arena;
mod constants;

pub use arena::*;
use bevy::math::Vec3;
pub use constants::*;

pub fn get_local_tile_space(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x * TILE_SIZE, y * TILE_SIZE, z)
}
