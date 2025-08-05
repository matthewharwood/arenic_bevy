
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

pub const TOTAL_ARENAS: u32 = 9;

/// Window dimensions
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
pub const HALF_WINDOW_WIDTH: f32 = WINDOW_WIDTH / 2.0;
pub const HALF_WINDOW_HEIGHT: f32 = WINDOW_HEIGHT / 2.0;

