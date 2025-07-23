//! Const generic camera system for type-safe scales and positioning.
//!
//! This module provides compile-time validated camera scales and pre-computed
//! positioning for optimal performance.

use crate::const_grid::StandardArenaPositions;
use bevy::prelude::*;
use std::marker::PhantomData;

/// Type-safe camera scales with const generics
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraScale<const NUMERATOR: u32, const DENOMINATOR: u32 = 1> {
    _phantom: PhantomData<()>,
}

impl<const NUMERATOR: u32, const DENOMINATOR: u32> CameraScale<NUMERATOR, DENOMINATOR> {
    /// Get the scale value as f32
    pub const fn value() -> f32 {
        NUMERATOR as f32 / DENOMINATOR as f32
    }

    /// Check if this scale represents a zoomed-out view
    pub const fn is_zoomed_out() -> bool {
        NUMERATOR > DENOMINATOR
    }
}

/// Predefined camera scales using const generics
pub type NormalScale = CameraScale<1, 1>; // 1.0x scale

/// Compile-time camera positioning calculations
pub struct CameraPositions;

impl CameraPositions {
    /// Calculate camera position for any arena (at runtime, but using const when possible)
    pub const fn calculate_arena_position(arena_index: usize) -> Option<(f32, f32)> {
        StandardArenaPositions::get_center(arena_index)
    }
}

/// Marker trait for camera scale types
pub trait CameraScaleMarker: Send + Sync + 'static {
    fn scale_value() -> f32;
    fn is_zoomed_out() -> bool;
}

impl<const NUMERATOR: u32, const DENOMINATOR: u32> CameraScaleMarker
    for CameraScale<NUMERATOR, DENOMINATOR>
{
    fn scale_value() -> f32 {
        Self::value()
    }

    fn is_zoomed_out() -> bool {
        Self::is_zoomed_out()
    }
}

/// Bundle for creating cameras with typed scales
#[derive(Bundle)]
pub struct TypedCameraBundle {
    pub camera: Camera2d,
    pub transform: Transform,
    pub projection: Projection,
}

impl TypedCameraBundle {
    /// Create a new typed camera bundle at a specific position with given scale
    pub fn new_at_position_with_scale(x: f32, y: f32, scale: f32) -> Self {
        Self {
            camera: Camera2d,
            transform: Transform::from_xyz(x, y, 0.0),
            projection: Projection::Orthographic(OrthographicProjection {
                near: -1000.0,
                scale,
                far: 1000.0,
                viewport_origin: Vec2::new(0.5, 0.5),
                area: Rect::new(-1.0, -1.0, 1.0, 1.0),
                scaling_mode: Default::default(),
            }),
        }
    }

    /// Create a new typed camera bundle for a specific arena with const generic scale
    pub fn new_for_arena_with_scale<S: CameraScaleMarker>(arena_index: usize) -> Self {
        let (x, y) = if S::is_zoomed_out() {
            (0.0, 0.0) // Fallback position
        } else {
            CameraPositions::calculate_arena_position(arena_index).unwrap_or((0.0, 0.0))
        };

        Self::new_at_position_with_scale(x, y, S::scale_value())
    }

    /// Create a normal scale camera bundle for an arena
    pub fn normal_for_arena(arena_index: usize) -> Self {
        Self::new_for_arena_with_scale::<NormalScale>(arena_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_scales() {
        assert_eq!(NormalScale::value(), 1.0);
        assert!(!NormalScale::is_zoomed_out());
    }
}
