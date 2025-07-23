//! Camera management plugin.
//! 
//! This plugin handles all camera-related functionality including movement,
//! zoom, and animations.

use bevy::prelude::*;
use crate::{
    animation::{update_camera_animations, animate_camera_on_arena_change, animate_zoom_transitions},
    components::CurrentArena,
    config::{arena::*, camera::*, display::*},
};

/// Plugin responsible for camera management and animations
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_camera_animations,
                animate_camera_on_arena_change,
                animate_zoom_transitions,
            ),
        );
    }
}