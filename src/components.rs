//! Game components definitions.
//! 
//! This module contains all the component types used in the ECS architecture.
//! Components represent data that can be attached to entities.

use bevy::prelude::*;
use std::time::Duration;

/// Tracks the currently active arena (0-8)
#[derive(Component, Debug)]
pub struct CurrentArena(pub u8);

impl CurrentArena {
    /// Increment arena index cyclically (0-8)
    pub fn increment(value: u8) -> u8 {
        (value + 1) % 9
    }

    /// Decrement arena index cyclically (0-8)  
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

/// Timer component for each arena
#[derive(Component, Debug)]
pub struct ArenaTimer {
    pub timer: Timer,
    pub arena: ArenaName,
}

impl ArenaTimer {
    /// Create a new arena timer with default 2-minute duration
    pub fn new(arena: ArenaName) -> Self {
        let mut timer = Timer::new(Duration::from_secs(120), TimerMode::Repeating);
        timer.pause(); // Start paused until a CharacterSelected enters
        Self {
            timer,
            arena,
        }
    }
    
    /// Create a new arena timer with custom duration
    pub fn new_with_duration(arena: ArenaName, duration: Duration) -> Self {
        let mut timer = Timer::new(duration, TimerMode::Repeating);
        timer.pause(); // Start paused until a CharacterSelected enters
        Self {
            timer,
            arena,
        }
    }
}

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

// UI component markers
/// Marker component for the top navigation bar
#[derive(Component, Debug)]
pub struct TopNavBar;

/// Marker component for side navigation bars
#[derive(Component, Debug)]
pub struct SideNavBar;

/// Marker component for the bottom navigation bar
#[derive(Component, Debug)]
pub struct BottomNavBar;

