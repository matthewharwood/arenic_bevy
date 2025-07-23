//! Game components definitions.
//! 
//! This module contains all the component types used in the ECS architecture.
//! Components represent data that can be attached to entities.

use bevy::prelude::*;
use std::fmt::{self, Display};
use crate::const_grid::StandardArenaIndex;


/// Tracks the currently active arena with const generic type safety
#[derive(Component, Debug, Copy, Clone)]
pub struct CurrentArena<const TOTAL_ARENAS: usize = 9> {
    arena: StandardArenaIndex,
}

impl<const TOTAL_ARENAS: usize> CurrentArena<TOTAL_ARENAS> {
    /// Create CurrentArena without bounds checking (when index is guaranteed valid)
    pub const fn new_unchecked(index: usize) -> Self {
        Self { 
            arena: StandardArenaIndex::new_unchecked(index) 
        }
    }
    
    /// Get the arena index as u8 for legacy compatibility
    pub const fn get_index_u8(self) -> u8 {
        self.arena.get() as u8
    }
    
    /// Navigate to next arena with compile-time bounds checking
    pub const fn next(self) -> Self {
        Self { arena: self.arena.next() }
    }
    
    /// Navigate to previous arena with compile-time bounds checking
    pub const fn prev(self) -> Self {
        Self { arena: self.arena.prev() }
    }
}

impl CurrentArena {
    /// Mutable increment using const generic navigation
    pub fn increment_mut(&mut self) {
        *self = self.next();
    }
    
    /// Mutable decrement using const generic navigation
    pub fn decrement_mut(&mut self) {
        *self = self.prev();
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


