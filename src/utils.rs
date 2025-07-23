//! Utility functions for position calculations and game logic.
//!
//! This module contains pure functions that perform common calculations
//! used throughout the game systems.

use crate::config::{
    arena::{ARENA_HEIGHT, ARENA_WIDTH},
    display::{HALF_TILE_SIZE, HALF_WINDOW_HEIGHT, HALF_WINDOW_WIDTH, TILE_SIZE},
    ui::{CAMERA_PADDING_Y, SIDEBAR_WIDTH},
};

/// Calculate the camera position to center on a specific arena
///
/// # Arguments
/// * `arena_index` - The arena index (0-8) to center the camera on
///
/// # Returns
/// A tuple containing the (x, y) world coordinates for the camera
pub fn calculate_camera_position(arena_index: u8) -> (f32, f32) {
    let arena_col = arena_index % 3;
    let arena_row = arena_index / 3;

    // Calculate arena top-left corner (matching setup positioning)
    let arena_x = -SIDEBAR_WIDTH + (arena_col as f32 * ARENA_WIDTH);
    let arena_y = CAMERA_PADDING_Y - (arena_row as f32 * ARENA_HEIGHT);

    // Calculate an arena center by adding half-arena dimensions
    let center_x = arena_x;
    let center_y = arena_y;

    (center_x, center_y)
}

/// Calculate the world position of a character within a specific arena
///
/// # Arguments
/// * `arena_index` - The arena index (0-8) where the character is located
/// * `tile_x` - The tile X coordinate within the arena (0 to GRID_WIDTH-1)
/// * `tile_y` - The tile Y coordinate within the arena (0 to GRID_HEIGHT-1)
///
/// # Returns
/// A tuple containing the (x, y) world coordinates for the character
pub fn calculate_character_position(arena_index: u8, tile_x: usize, tile_y: usize) -> (f32, f32) {
    let arena_col = arena_index % 3;
    let arena_row = arena_index / 3;

    // Calculate arena top-left corner (matching setup positioning)
    let arena_x = -HALF_WINDOW_WIDTH + HALF_TILE_SIZE + (arena_col as f32 * ARENA_WIDTH);
    let arena_y = HALF_WINDOW_HEIGHT - HALF_TILE_SIZE - (arena_row as f32 * ARENA_HEIGHT);

    // Calculate character position within the arena
    let char_x = arena_x + (tile_x as f32 * TILE_SIZE);
    let char_y = arena_y - (tile_y as f32 * TILE_SIZE);

    (char_x, char_y)
}

/// Calculate the boundaries of the entire 3x3 arena grid
///
/// # Returns
/// A tuple containing (left, right, top, bottom) boundaries in world coordinates
pub fn calculate_grid_boundaries() -> (f32, f32, f32, f32) {
    let grid_left = -HALF_WINDOW_WIDTH + HALF_TILE_SIZE;
    let grid_right = grid_left + (3.0 * ARENA_WIDTH) - TILE_SIZE;
    let grid_top = HALF_WINDOW_HEIGHT - HALF_TILE_SIZE;
    let grid_bottom = grid_top - (3.0 * ARENA_HEIGHT) + TILE_SIZE;

    (grid_left, grid_right, grid_top, grid_bottom)
}

/// Clamp a position to stay within the 3x3 arena grid boundaries
///
/// # Arguments
/// * `x` - The X coordinate to clamp
/// * `y` - The Y coordinate to clamp
///
/// # Returns
/// A tuple containing the clamped (x, y) coordinates
pub fn clamp_to_grid_boundaries(x: f32, y: f32) -> (f32, f32) {
    let (grid_left, grid_right, grid_top, grid_bottom) = calculate_grid_boundaries();

    let clamped_x = x.clamp(grid_left, grid_right);
    let clamped_y = y.clamp(grid_bottom, grid_top);

    (clamped_x, clamped_y)
}
