use bevy::prelude::*;
use crate::arena::TILE_SIZE;

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

    /// Calculate the column (0-2) from arena index
    pub fn col(self) -> u8 {
        self.0 % 3
    }

    /// Calculate the row (0-2) from arena index
    pub fn row(self) -> u8 {
        self.0 / 3
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



/// Marker component for arena tile entities.
#[derive(Component, Debug)]
pub struct ArenaTile;

pub fn get_local_tile_space(row: u32, col: u32) -> Vec3 {
    Vec3::new( row as f32 * TILE_SIZE, -(col as f32 * TILE_SIZE), 0.0)
}

/// Grid position component for tiles within an arena.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPosition {
    pub row: u32,
    pub col: u32,
}