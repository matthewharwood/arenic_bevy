mod auto_shot;
mod holy_nova;

pub use auto_shot::*;
use bevy::math::Vec3;
use bevy::prelude::Component;
pub use holy_nova::*;
use std::fmt::{Display, Formatter, Result as FmtResult};

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

/// Unified ability type enum for recording and playback
/// Replaces the duplicative AbilityId from timeline module
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AbilityType {
    AutoShot,
    HolyNova,
    PoisonShot,
    Heal,
}

impl AbilityType {
    /// Get ability type from numeric ID for backwards compatibility
    #[must_use]
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            1 => Some(Self::AutoShot),
            2 => Some(Self::HolyNova),
            3 => Some(Self::PoisonShot),
            4 => Some(Self::Heal),
            _ => None,
        }
    }

    /// Convert to numeric ID for storage/serialization
    #[must_use]
    pub fn to_id(self) -> u8 {
        match self {
            Self::AutoShot => 1,
            Self::HolyNova => 2,
            Self::PoisonShot => 3,
            Self::Heal => 4,
        }
    }
}

impl Display for AbilityType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let name = match *self {
            Self::AutoShot => "AutoShot",
            Self::HolyNova => "HolyNova",
            Self::PoisonShot => "PoisonShot",
            Self::Heal => "Heal",
        };
        write!(f, "{}", name)
    }
}
