use crate::arena::{CurrentArena, ARENA_HEIGHT, ARENA_WIDTH};
use bevy::prelude::*;

const ZOOM: (f32, f32) = (24.0, 72.0);

/// Shared function to position camera based on arena index
pub fn position_camera_for_arena(transform: &mut Transform, arena_index: u8) {
    let (x, y) = (8.125, 3.5);
    let (offset_x, offset_y) = calculate_camera_position(arena_index);
    let camera_translation = Vec3::new(x + offset_x, y - offset_y, ZOOM.0);
    let camera_center = Vec3::new(x + offset_x, y - offset_y, 0.0);

    transform.translation = camera_translation;
    transform.look_at(camera_center, Vec3::Y);
}

/// Setup camera to center on a specific arena
pub fn setup_camera(mut commands: Commands, current_arena: Single<&CurrentArena>) {
    let arena = current_arena.into_inner();
    let mut transform = Transform::default();
    position_camera_for_arena(&mut transform, arena.0);

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
            far: 150.0, // Increased far plane to accommodate further camera distance
        }),
        transform,
    ));
}

pub fn toggle_camera_zoom(
    keycode: Res<ButtonInput<KeyCode>>,
    camera_query: Single<&mut Transform, With<Camera>>,
) {
    if keycode.just_pressed(KeyCode::KeyP) {
        let mut camera = camera_query.into_inner();
        if camera.translation.z == ZOOM.0 {
            camera.translation.z = ZOOM.1;
        } else {
            camera.translation.z = ZOOM.0;
        }
    }
    if keycode.just_pressed(KeyCode::BracketRight) {}
}
pub fn calculate_camera_position(arena_index: u8) -> (f32, f32) {
    let arena_col = arena_index % 3;
    let arena_row = arena_index / 3;
    (
        arena_col as f32 * ARENA_WIDTH,
        arena_row as f32 * ARENA_HEIGHT,
    )
}

pub fn move_camera(
    current_arena: Single<&CurrentArena, Changed<CurrentArena>>,
    camera: Single<&mut Transform, With<Camera3d>>,
) {
    let arena = current_arena.into_inner();
    let mut camera_transform = camera.into_inner();
    position_camera_for_arena(&mut camera_transform, arena.0);
}
