//! Game plugins organized by functionality.
//! 
//! This module contains all the specialized plugins that handle different aspects
//! of the game, following a plugin-centric architecture for better maintainability.

pub mod arena;
pub mod camera;
pub mod character;
pub mod initialization;
pub mod input;

// Re-exports for convenience
pub use arena::*;
pub use camera::*;
pub use character::*;
pub use initialization::*;
pub use input::*;