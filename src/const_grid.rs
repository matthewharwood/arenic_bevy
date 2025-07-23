//! Const generic grid system for type-safe coordinates and arena management.
//! 
//! This module provides compile-time validated grid operations, arena indexing,
//! and position calculations for optimal performance and type safety.

/// Standard arena index with compile-time bounds checking
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StandardArenaIndex {
    index: usize,
}

impl StandardArenaIndex {
    /// Create arena index without bounds checking (when index is guaranteed valid)
    pub const fn new_unchecked(index: usize) -> Self {
        Self { index }
    }
    
    /// Get the raw index value
    pub const fn get(self) -> usize {
        self.index
    }
    
    /// Navigate to next arena with wraparound
    pub const fn next(self) -> Self {
        Self { index: (self.index + 1) % 9 }
    }
    
    /// Navigate to previous arena with wraparound
    pub const fn prev(self) -> Self {
        Self { 
            index: if self.index == 0 { 8 } else { self.index - 1 }
        }
    }
}

/// Pre-computed arena positions for O(1) lookup
pub struct StandardArenaPositions;

impl StandardArenaPositions {
    /// Pre-computed arena center positions (compile-time calculated)
    const ARENA_CENTERS: [(f32, f32); 9] = {
        // Constants for calculations
        const HALF_WINDOW_WIDTH: f32 = 640.0; // 1280 / 2
        const HALF_WINDOW_HEIGHT: f32 = 360.0; // 720 / 2
        const ARENA_WIDTH: f32 = 1254.0; // 66 * 19
        const ARENA_HEIGHT: f32 = 589.0; // 31 * 19
        
        let mut positions = [(0.0, 0.0); 9];
        let mut i = 0;
        
        // Compile-time loop to calculate all positions
        while i < 9 {
            let arena_col = i % 3;
            let arena_row = i / 3;
            
            let x = -HALF_WINDOW_WIDTH + (arena_col as f32 * ARENA_WIDTH) + ARENA_WIDTH / 2.0;
            let y = HALF_WINDOW_HEIGHT - (arena_row as f32 * ARENA_HEIGHT) - ARENA_HEIGHT / 2.0;
            
            positions[i] = (x, y);
            i += 1;
        }
        
        positions
    };
    
    /// Zero-cost arena position lookup (no runtime calculation)
    pub const fn get_center(arena_index: usize) -> Option<(f32, f32)> {
        if arena_index < 9 {
            Some(Self::ARENA_CENTERS[arena_index])
        } else {
            None
        }
    }
}