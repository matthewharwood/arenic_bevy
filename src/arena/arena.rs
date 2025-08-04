use bevy::prelude::*;

/// Type-safe identifier for an arena.
/// Arenas are indexed 0-8 in a 3x3 grid, left-to-right, top-to-bottom.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArenaId(pub u8);

impl ArenaId {
    /// Create a new ArenaId, ensuring it's within valid range (0-8)
    pub fn new(id: u8) -> Option<Self> {
        if id < 9 {
            Some(ArenaId(id))
        } else {
            None
        }
    }

    /// Get the raw arena index
    pub fn index(self) -> u8 {
        self.0
    }

    /// Calculate the column (0-2) from arena index
    pub fn col(self) -> u8 {
        self.0 % 3
    }

    /// Calculate the row (0-2) from arena index
    pub fn row(self) -> u8 {
        self.0 / 3
    }

    /// Get position within the 3x3 grid as (col, row)
    pub fn grid_pos(self) -> (u8, u8) {
        (self.col(), self.row())
    }
}

/// Marker component identifying an arena entity in the world.
/// Each arena entity should have this component along with ArenaId.
#[derive(Component, Debug)]
pub struct Arena;

/// Marker component for the currently active arena.
/// Only one arena entity should have this component at any time.
#[derive(Component, Debug)]
pub struct ActiveArena;

/// Associates an entity (typically a character) with a specific arena.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct InArena {
    pub arena_id: ArenaId,
}

impl InArena {
    pub fn new(arena_id: ArenaId) -> Self {
        Self { arena_id }
    }
}

/// Marker component for character entities.
#[derive(Component, Debug)]
pub struct Character;

/// Marker component for arena tile entities.
#[derive(Component, Debug)]
pub struct ArenaTile;

/// Grid position component for tiles within an arena.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
}

impl GridPosition {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}