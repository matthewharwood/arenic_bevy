use crate::arena::{ARENA_HEIGHT, ARENA_WIDTH, CameraUpdate, CurrentArena, TILE_SIZE};
use crate::arena_camera::ZoomOut;
use bevy::prelude::*;

pub const ZOOM: (f32, f32) = (24.0, 72.0);

/// Shared function to position camera based on arena index
pub fn position_camera_for_arena(transform: &mut Transform, arena_index: u8, zoom: f32) {
    let (x, y) = (8.125, 3.5);
    let (offset_x, offset_y) = calculate_camera_position(arena_index);
    let camera_translation = Vec3::new(x + offset_x, y - offset_y, zoom);
    let camera_center = Vec3::new(x + offset_x, y - offset_y, 0.0);

    transform.translation = camera_translation;
    transform.look_at(camera_center, Vec3::Y);
}

/// Setup camera to center on a specific arena
pub fn setup_camera(mut commands: Commands, current_arena: Res<CurrentArena>) {
    let arena = &*current_arena;
    let mut transform = Transform::default();
    position_camera_for_arena(&mut transform, arena.0.as_u8(), ZOOM.0);

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
    mut commands: Commands,
    keycode: Res<ButtonInput<KeyCode>>,
    current_arena: Res<CurrentArena>,
    camera_query: Single<(Entity, &mut Transform, Option<&ZoomOut>), With<Camera>>,
    mut arena_refresh_event: EventWriter<CameraUpdate>,
) {
    if keycode.just_pressed(KeyCode::KeyP) {
        let (camera_entity, mut camera_transform, zoom_out) = camera_query.into_inner();
        let current_arena = &*current_arena;

        if zoom_out.is_some() {
            // Camera is zoomed out, zoom back in to current arena
            // Reset camera position based on current arena
            position_camera_for_arena(&mut camera_transform, current_arena.0.as_u8(), ZOOM.0);
            commands.entity(camera_entity).remove::<ZoomOut>();

            // Send event
            arena_refresh_event.write(CameraUpdate);
        } else {
            // Center camera to see all 9 arenas (middle of the 3x3 grid)
            commands.entity(camera_entity).insert(ZoomOut);
            position_camera_for_arena(&mut camera_transform, 4, ZOOM.1);

            // Send event
            arena_refresh_event.write(CameraUpdate);
        }
    }
}

pub fn calculate_camera_position(arena_index: u8) -> (f32, f32) {
    let arena_col = arena_index % 3;
    let arena_row = arena_index / 3;
    (
        arena_col as f32 * ARENA_WIDTH,
        arena_row as f32 * ARENA_HEIGHT,
    )
}

/// Draw a black border around the current arena when zoomed out
pub fn draw_arena_border(
    mut gizmos: Gizmos,
    current_arena: Res<CurrentArena>,
    camera: Query<&ZoomOut, With<Camera3d>>,
) {
    // Only draw if camera is zoomed out - following Rule #22: Error-Safe ECS First
    let Ok(_zoom_out) = camera.single() else {
        return;
    };

    let arena = &*current_arena;

    // Use the same calculation as position_camera_for_arena to get the exact center
    let (x, y) = (8.125, 3.5); // Base position (center of a single arena)
    let (offset_x, offset_y) = calculate_camera_position(arena.0.as_u8());
    let center = Vec3::new(x + offset_x, y - offset_y + (TILE_SIZE / 2.0), 1.0); // Same as camera looks at, with z=1 for visibility

    // Draw 5 rectangles for thickness (using 3D rect in world space)
    for i in 0..5 {
        let thickness_offset = i as f32 * 0.05; // Larger offset for visibility

        // Draw rectangle using rect (3D version) - positioned in world space
        gizmos.rect(
            Isometry3d::from_translation(center),
            Vec2::new(
                ARENA_WIDTH + thickness_offset * 2.0,
                ARENA_HEIGHT + thickness_offset * 2.0,
            ),
            Color::BLACK,
        );
    }
}
