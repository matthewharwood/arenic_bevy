use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use crate::arena::{
    get_arena_position, ArenaId, ARENA_HEIGHT_HALF, ARENA_WIDTH_HALF, HALF_TILE,
};


/// Calculate camera position to center on a specific arena
pub fn calculate_camera_position(arena_id: ArenaId) -> Vec3 {
    let position = get_arena_position(arena_id);
    // Add half arena dimensions to get center
    position + Vec3::new(ARENA_WIDTH_HALF - HALF_TILE, -ARENA_HEIGHT_HALF + HALF_TILE, 0.0)
}

/// Setup camera to center on a specific arena
pub fn setup_camera(commands: &mut Commands, arena_id: ArenaId) {
    let center = calculate_camera_position(arena_id);
    let camera_pos = center + Vec3::new(0.0, 0.0, 1000.0); // Add height
    
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(camera_pos).looking_at(center, Vec3::Y),
        Projection::from(OrthographicProjection {
            scale: 1.0,
            scaling_mode: ScalingMode::WindowSize,
            ..OrthographicProjection::default_3d()
        }),
    ));
}