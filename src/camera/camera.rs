use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use crate::{
    ARENA_HEIGHT, ARENA_HEIGHT_HALF, ARENA_WIDTH, ARENA_WIDTH_HALF, ARENAS_PER_ROW,
    HALF_TILE, HALF_WINDOW_HEIGHT, HALF_WINDOW_WIDTH,
};

/// Calculate the world position of an arena by its index (0-8)
pub fn get_arena_position(arena_index: u32) -> Vec3 {
    let col = arena_index % ARENAS_PER_ROW;
    let row = arena_index / ARENAS_PER_ROW;
    
    // Start from window top-left corner and offset by arena size
    let x = -HALF_WINDOW_WIDTH + (col as f32 * ARENA_WIDTH) + HALF_TILE;
    let y = HALF_WINDOW_HEIGHT - (row as f32 * ARENA_HEIGHT) - HALF_TILE;
    
    Vec3::new(x, y, 0.0)
}

/// Calculate camera position to center on a specific arena
pub fn calculate_camera_position(arena_index: u8) -> Vec3 {
    let position = get_arena_position(arena_index as u32);
    // Add half arena dimensions to get center
    position + Vec3::new(ARENA_WIDTH_HALF - HALF_TILE, -ARENA_HEIGHT_HALF + HALF_TILE, 0.0)
}

/// Setup camera to center on a specific arena (0-8)
pub fn setup_camera(commands: &mut Commands, arena_index: u8) {
    let center = calculate_camera_position(arena_index);
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