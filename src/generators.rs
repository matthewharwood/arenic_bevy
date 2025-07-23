//! Generator functions using Rust 2024's `gen` keyword for improved iteration patterns.
//! 
//! This module provides clean, readable generator functions that yield sequences
//! of values for various game components.

use crate::config::{arena::*, display::*};

/// Arena position information for setup generation
#[derive(Debug, Clone, Copy)]
pub struct ArenaPosition {
    pub index: usize,
    pub x_offset: f32,
    pub y_offset: f32,
    pub world_x: f32,
    pub world_y: f32,
}

/// Generator that yields arena positions for the 3x3 grid setup
/// This replaces the manual loop in setup() with a cleaner, more readable pattern
pub gen fn arena_positions() -> ArenaPosition {
    const ARENAS_PER_ROW: usize = 3;
    const TOTAL_ARENAS: usize = 9;
    
    for arena_index in 0..TOTAL_ARENAS {
        let arena_col = arena_index % ARENAS_PER_ROW;
        let arena_row = arena_index / ARENAS_PER_ROW;

        let x_offset = arena_col as f32 * ARENA_WIDTH;
        let y_offset = arena_row as f32 * ARENA_HEIGHT;

        let world_x = -HALF_WINDOW_WIDTH + HALF_TILE_SIZE + x_offset;
        let world_y = HALF_WINDOW_HEIGHT - HALF_TILE_SIZE - y_offset;

        yield ArenaPosition {
            index: arena_index,
            x_offset,
            y_offset,
            world_x,
            world_y,
        };
    }
}

/// Grid tile position information
#[derive(Debug, Clone, Copy)]
pub struct TilePosition {
    pub row: usize,
    pub col: usize,
    pub world_x: f32,
    pub world_y: f32,
}

/// Generator that yields tile positions within an arena
pub gen fn arena_tile_positions() -> TilePosition {
    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            let world_x = col as f32 * TILE_SIZE;
            let world_y = -(row as f32 * TILE_SIZE);

            yield TilePosition {
                row,
                col,
                world_x,
                world_y,
            };
        }
    }
}