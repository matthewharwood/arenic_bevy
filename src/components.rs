//! Game components definitions.
//! 
//! This module contains all the component types used in the ECS architecture.
//! Components represent data that can be attached to entities.

use bevy::prelude::*;
use std::fmt::{self, Display};

/// Trait for types that support cyclic navigation (wrapping around at boundaries)
pub trait CyclicNavigation: Sized + Copy {
    /// The underlying value type (must support arithmetic and conversion)
    type Value: Into<usize> + TryFrom<usize> + Copy + PartialOrd;
    
    /// Get the current value
    fn current_value(&self) -> Self::Value;
    /// Create an instance with the given value
    fn with_value(value: Self::Value) -> Self;
    /// Get the maximum value for the cycle
    fn max_value() -> Self::Value;
    
    /// Move to the next item cyclically
    fn increment(&self) -> Self {
        let current: usize = self.current_value().into();
        let max: usize = Self::max_value().into();
        let next = (current + 1) % max;
        
        Self::with_value(
            Self::Value::try_from(next)
                .unwrap_or_else(|_| panic!("Failed to convert {} to value type", next))
        )
    }
    
    /// Move to the previous item cyclically  
    fn decrement(&self) -> Self {
        let current: usize = self.current_value().into();
        let max: usize = Self::max_value().into();
        let prev = if current == 0 { max - 1 } else { current - 1 };
        
        Self::with_value(
            Self::Value::try_from(prev)
                .unwrap_or_else(|_| panic!("Failed to convert {} to value type", prev))
        )
    }
}

/// Tracks the currently active arena (0-8)
#[derive(Component, Debug, Copy, Clone)]
pub struct CurrentArena(pub u8);

impl CyclicNavigation for CurrentArena {
    type Value = u8;
    
    fn current_value(&self) -> Self::Value {
        self.0
    }
    
    fn with_value(value: Self::Value) -> Self {
        CurrentArena(value)
    }
    
    fn max_value() -> Self::Value {
        9
    }
}

impl CurrentArena {
    /// Increment arena index cyclically (0-8) - legacy method for compatibility
    pub fn increment(value: u8) -> u8 {
        (value + 1) % 9
    }

    /// Decrement arena index cyclically (0-8) - legacy method for compatibility
    pub fn decrement(value: u8) -> u8 {
        if value == 0 { 8 } else { value - 1 }
    }
}

/// Marker component for entities that represent game characters
#[derive(Component, Debug)]
pub struct Character;

/// Marker component for the currently selected character
#[derive(Component, Debug)]
pub struct CharacterSelected;

/// Arena identification by name
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArenaName {
    Labyrinth = 0,
    GuildHouse = 1,
    Sanctum = 2,
    Mountain = 3,
    Bastion = 4,
    Pawnshop = 5,
    Crucible = 6,
    Casino = 7,
    Gala = 8,
}

impl ArenaName {
    /// Create an ArenaName from an index (0-8)
    /// 
    /// # Panics
    /// Panics if the index is not in the range 0-8
    pub fn from_index(index: u8) -> ArenaName {
        match index {
            0 => ArenaName::Labyrinth,
            1 => ArenaName::GuildHouse,
            2 => ArenaName::Sanctum,
            3 => ArenaName::Mountain,
            4 => ArenaName::Bastion,
            5 => ArenaName::Pawnshop,
            6 => ArenaName::Crucible,
            7 => ArenaName::Casino,
            8 => ArenaName::Gala,
            _ => panic!("Invalid arena index: {}", index),
        }
    }
    
    /// Convert ArenaName to its numeric index
    pub fn to_index(&self) -> u8 {
        *self as u8
    }
    
    /// Get the human-readable name of the arena
    pub fn name(&self) -> &'static str {
        match self {
            ArenaName::Labyrinth => "Labyrinth",
            ArenaName::GuildHouse => "Guild House",
            ArenaName::Sanctum => "Sanctum",
            ArenaName::Mountain => "Mountain",
            ArenaName::Bastion => "Bastion",
            ArenaName::Pawnshop => "Pawnshop",
            ArenaName::Crucible => "Crucible",
            ArenaName::Casino => "Casino",
            ArenaName::Gala => "Gala",
        }
    }
}

impl Display for ArenaName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl From<u8> for ArenaName {
    fn from(index: u8) -> Self {
        Self::from_index(index)
    }
}

impl From<ArenaName> for u8 {
    fn from(arena: ArenaName) -> Self {
        arena.to_index()
    }
}


