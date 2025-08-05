use bevy::prelude::*;

/// Grid dimensions for each arena
pub const GRID_WIDTH: u32 = 66;
pub const GRID_HEIGHT: u32 = 31;

/// Physical dimensions
pub const TILE_SIZE: f32 = 19.0; // Each tile is 19 world units
pub const HALF_TILE: f32 = TILE_SIZE / 2.0;

/// Calculated arena dimensions
pub const ARENA_WIDTH: f32 = GRID_WIDTH as f32 * TILE_SIZE;
pub const ARENA_HEIGHT: f32 = GRID_HEIGHT as f32 * TILE_SIZE;
pub const ARENA_WIDTH_HALF: f32 = ARENA_WIDTH / 2.0;
pub const ARENA_HEIGHT_HALF: f32 = ARENA_HEIGHT / 2.0;

pub const TOTAL_ARENAS: u8 = 9;

/// Window dimensions
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
pub const HALF_WINDOW_WIDTH: f32 = WINDOW_WIDTH / 2.0;
pub const HALF_WINDOW_HEIGHT: f32 = WINDOW_HEIGHT / 2.0;

/// Debug colors for arena visualization (one for each arena)
pub const DEBUG_COLORS: [Color; 9] = [
    Color::srgb(1.0, 0.329, 0.0),     // #ff5400
    Color::srgb(1.0, 0.557, 0.0),     // #ff8e00
    Color::srgb(1.0, 0.824, 0.0),     // #ffd200
    Color::srgb(0.506, 0.902, 0.314), // #81e650
    Color::srgb(0.0, 0.824, 0.404),   // #00d267
    Color::srgb(0.0, 0.753, 1.0),     // #00c0ff
    Color::srgb(0.545, 0.282, 0.996), // #8b48fe
    Color::srgb(0.792, 0.255, 0.988), // #ca41fc
    Color::srgb(1.0, 0.275, 0.984),   // #ff46fb
];
