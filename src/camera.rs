//! Camera system plugin.
//! 
//! This plugin handles all camera functionality including camera setup,
//! arena navigation, and zoom controls.

use bevy::prelude::*;
use crate::components::CurrentArena;
use crate::config::display::TILE_SIZE;
use crate::utils::calculate_camera_position;

/// Plugin that handles camera systems
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, (
                handle_arena_navigation_keys,
                update_camera_on_arena_change,
                handle_zoom_toggle,
                draw_arena_gizmo,
            ));
    }
}

/// Setup the main camera at startup
fn setup_camera(mut commands: Commands) {
    let (camera_x, camera_y) = calculate_camera_position(1);
    commands
        .spawn(Camera2d)
        .insert(Transform::from_xyz(camera_x, camera_y, 0.0))
        .insert(Projection::Orthographic(OrthographicProjection {
            near: -1000.0,
            scale: 1.0,
            far: 1000.0,
            viewport_origin: Vec2::new(0.5, 0.5),
            area: Rect::new(-1.0, -1.0, 1.0, 1.0),
            scaling_mode: Default::default(),
        }));
}

/// Handle arena navigation keys (left/right bracket)
fn handle_arena_navigation_keys(
    mut arena_query: Query<&mut CurrentArena>,
    camera_query: Query<&Projection, With<Camera>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    // Check if camera is at scale 3.0
    let _is_zoomed_out = camera_query.iter().any(|projection| {
        if let Projection::Orthographic(ortho) = projection {
            ortho.scale == 3.0
        } else {
            false
        }
    });

    if input.just_pressed(KeyCode::BracketRight) {
        for mut arena in &mut arena_query {
            arena.0 = CurrentArena::increment(arena.0);
        }
    }
    if input.just_pressed(KeyCode::BracketLeft) {
        for mut arena in &mut arena_query {
            arena.0 = CurrentArena::decrement(arena.0);
        }
    }
}

/// Update camera position when arena changes
fn update_camera_on_arena_change(
    arena_query: Query<&CurrentArena, Changed<CurrentArena>>,
    mut camera_query: Query<(&mut Transform, &Projection), With<Camera>>,
) {
    if let Ok(current_arena) = arena_query.single() {
        let (camera_x, camera_y) = calculate_camera_position(current_arena.0);

        for (mut transform, projection) in &mut camera_query {
            // Only move camera if not zoomed out (scale 1.0)
            if let Projection::Orthographic(ortho) = projection {
                if ortho.scale == 1.0 {
                    transform.translation.x = camera_x;
                    transform.translation.y = camera_y;
                }
            }
        }
    }
}

/// Handle zoom toggle with P key
fn handle_zoom_toggle(
    arena_query: Query<&CurrentArena>,
    mut camera_query: Query<(&mut Transform, &mut Projection), With<Camera>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyP) {
        for (mut transform, mut projection) in &mut camera_query {
            if let Projection::Orthographic(ortho) = &mut *projection {
                if ortho.scale == 1.0 {
                    ortho.scale = 3.0;
                    // Center on arena index 4 for zoom out and move down by TILE_SIZE * 3
                    let (camera_x, camera_y) = calculate_camera_position(4);
                    transform.translation.x = camera_x;
                    transform.translation.y = camera_y - (TILE_SIZE * 3.0);
                } else {
                    ortho.scale = 1.0;
                    // Return to current arena position (without Y offset)
                    for arena in &arena_query {
                        let (camera_x, camera_y) = calculate_camera_position(arena.0);
                        transform.translation.x = camera_x;
                        transform.translation.y = camera_y;
                    }
                }
            }
        }
    }
}

/// Draw arena border gizmo when zoomed out
fn draw_arena_gizmo(
    mut gizmos: Gizmos,
    arena_query: Query<&CurrentArena>,
    camera_query: Query<&Projection, With<Camera>>,
) {
    use crate::config::arena::{ARENA_WIDTH, ARENA_HEIGHT};
    use crate::config::display::{HALF_WINDOW_WIDTH, HALF_WINDOW_HEIGHT};
    
    for projection in &camera_query {
        if let Projection::Orthographic(ortho) = projection {
            if ortho.scale == 3.0 {
                // Only draw gizmo when zoomed out
                for arena in &arena_query {
                    let arena_col = arena.0 % 3;
                    let arena_row = arena.0 / 3;

                    // Calculate the center of the current arena in world coordinates
                    let arena_center_x =
                        -HALF_WINDOW_WIDTH + (arena_col as f32 * ARENA_WIDTH) + ARENA_WIDTH / 2.0;
                    let arena_center_y =
                        HALF_WINDOW_HEIGHT - (arena_row as f32 * ARENA_HEIGHT) - ARENA_HEIGHT / 2.0;
                    let arena_center = Vec2::new(arena_center_x, arena_center_y);

                    let border_thickness = 10.0; // Desired total border thickness
                    let border_color = Color::BLACK; // Your desired border color

                    // Draw the border using a loop, building inwardly
                    for i in 0..border_thickness as u32 {
                        let current_thickness_offset = i as f32;
                        gizmos.rect_2d(
                            arena_center,
                            Vec2::new(
                                ARENA_WIDTH - current_thickness_offset * 2.0,
                                ARENA_HEIGHT - current_thickness_offset * 2.0,
                            ),
                            border_color,
                        );
                    }
                }
            }
        }
    }
}