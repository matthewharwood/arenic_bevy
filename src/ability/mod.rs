mod auto_shot;
mod holy_nova;

pub use auto_shot::*;
use bevy::math::Vec3;
use bevy::prelude::Component;
pub use holy_nova::*;

/// Marker component for projectile entities
#[derive(Component)]
pub struct Projectile;

/// Component tracking time-to-live (elapsed, total) in seconds
pub type TtlElapsed = f32;
pub type TtlTotal = f32;
#[derive(Component)]
pub struct TimeToLive(TtlElapsed, TtlTotal);

/// Component storing the target position for the projectile
#[derive(Component)]
pub struct Target(Vec3);

/// Component storing the initial spawn position for lerp calculations
#[derive(Component)]
pub struct Origin(Vec3);
