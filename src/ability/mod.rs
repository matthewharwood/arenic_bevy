mod auto_shot;
mod holy_nova;

pub use auto_shot::*;
use bevy::math::Vec3;
use bevy::prelude::Component;
pub use holy_nova::*;
// Note: Display imports removed with AbilityType

/// Marker component for projectile entities
#[derive(Component)]
pub struct Projectile;

/// Component storing the target position for the projectile
#[derive(Component)]
pub struct Target(Vec3);

/// Component storing the initial spawn position for lerp calculations
#[derive(Component)]
pub struct Origin(Vec3);

/// Component for tracking elapsed time in animations/effects
#[derive(Component)]
pub struct ElapsedTime(pub f32);

/// Component for storing duration of effects
#[derive(Component)]
pub struct Duration(pub f32);

/// Component for starting radius of expanding effects
#[derive(Component)]
pub struct StartRadius(pub f32);

/// Component for ending radius of expanding effects
#[derive(Component)]
pub struct EndRadius(pub f32);

/// Minimal ability type enum for timeline/testing compatibility
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AbilityType {
    AutoShot,
    HolyNova,
    PoisonShot,
    Heal,
}

// Note: Display implementation removed with AbilityType
