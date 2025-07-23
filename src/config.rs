//! Game configuration constants and types.
//! 
//! This module contains all the global constants used throughout the game,
//! organized by their purpose and usage.

/// Rendering and display constants
pub mod display {
    /// Size of each tile in pixels
    pub const TILE_SIZE: f32 = 19.0;
    
    /// Game window width in pixels
    pub const WINDOW_WIDTH: f32 = 1280.0;
    
    /// Game window height in pixels
    pub const WINDOW_HEIGHT: f32 = 720.0;
    
    /// Half of the window width for centering calculations
    pub const HALF_WINDOW_WIDTH: f32 = WINDOW_WIDTH / 2.0;
    
    /// Half of the window height for centering calculations
    pub const HALF_WINDOW_HEIGHT: f32 = WINDOW_HEIGHT / 2.0;
    
    /// Half of the tile size for positioning calculations
    pub const HALF_TILE_SIZE: f32 = TILE_SIZE / 2.0;
}

/// Arena grid configuration
pub mod arena {
    use super::display::TILE_SIZE;
    
    /// Number of tiles per arena width
    pub const GRID_WIDTH: usize = 66;
    
    /// Number of tiles per arena height
    pub const GRID_HEIGHT: usize = 31;
    
    /// Total width of a single arena in pixels
    pub const ARENA_WIDTH: f32 = GRID_WIDTH as f32 * TILE_SIZE;
    
    /// Total height of a single arena in pixels
    pub const ARENA_HEIGHT: f32 = GRID_HEIGHT as f32 * TILE_SIZE;
    
}

/// Camera and UI positioning constants
pub mod ui {
    
    /// Camera vertical padding
    pub const CAMERA_PADDING_Y: f32 = 36.0;
    
    /// Sidebar width in pixels
    pub const SIDEBAR_WIDTH: f32 = 13.5;
}

/// Asset paths used throughout the game
pub mod assets {
    /// Selected player sprite asset path
    pub const PLAYER_SELECTED: &str = "player_selected.png";
    
    /// Unselected player sprite asset path
    pub const PLAYER_UNSELECTED: &str = "player_unselected.png";
    
}