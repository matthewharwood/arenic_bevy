//! Game components definitions.
//!
//! This module contains all the component types used in the ECS architecture.
//! Components represent data that can be attached to entities.

use bevy::prelude::*;
use std::time::Duration;

/// Tracks the currently active arena (0-8)
#[derive(Component, Debug, Clone)]
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

/// Component for entities that represent game characters
#[derive(Component, Debug, Clone)]
pub struct Character {
    pub name: String,
}

impl Character {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// Marker component for the currently selected character
#[derive(Component, Debug, Clone)]
pub struct CharacterSelected;

/// Arena status for controlling timer and playback behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArenaStatus {
    /// Timer paused, no playback
    Paused,
    /// Playback state - timer ticking, all characters replaying saved sessions
    Playback,
}

impl Default for ArenaStatus {
    fn default() -> Self {
        ArenaStatus::Playback
    }
}

/// Timer component for each arena with status management
#[derive(Component, Debug, Clone)]
pub struct ArenaTimer {
    pub timer: Timer,
    pub arena: ArenaName,
    pub status: ArenaStatus,
}

impl ArenaTimer {
    /// Create a new arena timer with a default 2-minute duration (starts in Playback)
    pub fn new(arena: ArenaName) -> Self {
        let timer = Timer::new(Duration::from_secs(120), TimerMode::Repeating);
        // Timer starts unpaused for Playback
        Self {
            timer,
            arena,
            status: ArenaStatus::Playback,
        }
    }

    /// Get current arena status
    pub fn get_status(&self) -> ArenaStatus {
        self.status
    }

    /// Check if an arena is paused
    pub fn is_paused(&self) -> bool {
        self.status == ArenaStatus::Paused
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
    /// # Panics /// if the index is not in the range 0-8
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
#[derive(Component, Debug, Clone)]
pub struct TopNavBar;

/// Marker component for side navigation bars
#[derive(Component, Debug, Clone)]
pub struct SideNavBar;

/// Marker component for the bottom navigation bar
#[derive(Component, Debug, Clone)]
pub struct BottomNavBar;
