// Existing abilities
mod auto_shot;
mod holy_nova;

// Alchemist abilities
mod acid_flask;
mod ironskin;
mod siphon;
mod transmute;

// Bard abilities
mod cleanse;
mod dance;
mod helix;
mod mimic;

// Cardinal abilities
mod barrier;
mod beam;
mod heal;
mod resurrect;

// Forager abilities
mod border;
mod boulder;
mod dig;
mod mushroom;

// Hunter abilities (auto_shot already exists above)
mod poison_shot;
mod sniper;
mod trap;

// Merchant abilities
mod coin_toss;
mod dice;
mod fortune;
mod vault;

// Thief abilities
mod backstab;
mod pickpocket;
mod shadow_step;
mod smoke_screen;

// Warrior abilities
mod bash;
mod block;
mod bulwark;
mod taunt;

// Existing exports
pub use auto_shot::*;
pub use holy_nova::*;

// Alchemist exports
pub use acid_flask::*;
pub use ironskin::*;
pub use siphon::*;
pub use transmute::*;

// Bard exports
pub use cleanse::*;
pub use dance::*;
pub use helix::*;
pub use mimic::*;

// Cardinal exports
pub use barrier::*;
pub use beam::*;
pub use heal::*;
pub use resurrect::*;

// Forager exports
pub use border::*;
pub use boulder::*;
pub use dig::*;
pub use mushroom::*;

// Hunter exports
pub use poison_shot::*;
pub use sniper::*;
pub use trap::*;

// Merchant exports
pub use coin_toss::*;
pub use dice::*;
pub use fortune::*;
pub use vault::*;

// Thief exports
pub use backstab::*;
pub use pickpocket::*;
pub use shadow_step::*;
pub use smoke_screen::*;

// Warrior exports
pub use bash::*;
pub use block::*;
pub use bulwark::*;
pub use taunt::*;

use bevy::math::Vec3;
use bevy::prelude::Component;
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

/// Hunter abilities
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HunterAbility {
    AutoShot,
    PoisonShot,
    Sniper,
    Trap,
}

/// Cardinal abilities
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CardinalAbility {
    HolyNova,
    Heal,
    Barrier,
    Beam,
    Resurrect,
}

/// Alchemist abilities
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AlchemistAbility {
    AcidFlask,
    Ironskin,
    Siphon,
    Transmute,
}

/// Bard abilities
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BardAbility {
    Cleanse,
    Dance,
    Helix,
    Mimic,
}

/// Forager abilities
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ForagerAbility {
    Border,
    Boulder,
    Dig,
    Mushroom,
}

/// Merchant abilities
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MerchantAbility {
    CoinToss,
    Dice,
    Fortune,
    Vault,
}

/// Thief abilities
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ThiefAbility {
    Backstab,
    Pickpocket,
    ShadowStep,
    SmokeScreen,
}

/// Warrior abilities
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WarriorAbility {
    Bash,
    Block,
    Bulwark,
    Taunt,
}

/// Ability type enum with nested class-specific abilities
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AbilityType {
    Hunter(HunterAbility),
    Cardinal(CardinalAbility),
    Alchemist(AlchemistAbility),
    Bard(BardAbility),
    Forager(ForagerAbility),
    Merchant(MerchantAbility),
    Thief(ThiefAbility),
    Warrior(WarriorAbility),
}

// Note: Sub-enums are already publicly exported via their definitions above
