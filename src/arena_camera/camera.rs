use crate::arena::{get_arena_position, ArenaId, ARENA_HEIGHT_HALF, ARENA_WIDTH_HALF, HALF_TILE};
use crate::arena_camera::CAMERA_CENTER;
use bevy::prelude::*;

/// Calculate camera position to center on a specific arena
pub fn calculate_camera_position(arena_id: ArenaId) -> Vec3 {
    let position = get_arena_position(arena_id);
    // Add half arena dimensions to get center
    position
        + Vec3::new(
            ARENA_WIDTH_HALF - HALF_TILE,
            -ARENA_HEIGHT_HALF + HALF_TILE,
            0.0,
        )
}

/// Setup camera to center on a specific arena
pub fn setup_camera(commands: &mut Commands) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            fov: std::f32::consts::FRAC_PI_8,
            aspect_ratio: 16.0 / 9.0,
            near: 0.05,
            far: 50.0,
        }),
        Transform::from_xyz(8.125, 3.5, 24.0).looking_at(CAMERA_CENTER, Vec3::Y),
    ));
}
