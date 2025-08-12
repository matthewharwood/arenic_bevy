mod camera;

use bevy::math::Vec3;
use bevy::prelude::Component;
pub use camera::*;

/// Marker component indicating the camera is zoomed out
#[derive(Component)]
pub struct ZoomOut;

pub const CAMERA_CENTER: Vec3 = Vec3::new(8.125, 3.5, 0.0);
