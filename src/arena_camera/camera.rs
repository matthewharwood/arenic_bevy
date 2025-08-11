use crate::arena::{Arena, CurrentArena, ARENA_HEIGHT, ARENA_WIDTH};
use crate::arena_camera::ZoomOut;
use crate::character::Character;
use crate::selectors::Active;
use bevy::prelude::*;

const ZOOM: (f32, f32) = (24.0, 72.0);

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
pub fn setup_camera(mut commands: Commands, current_arena: Single<&CurrentArena>) {
    let arena = current_arena.into_inner();
    let mut transform = Transform::default();
    position_camera_for_arena(&mut transform, arena.0, ZOOM.0);

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
    current_arena_q: Single<&CurrentArena>,
    camera_query: Single<(Entity, &mut Transform, Option<&ZoomOut>), With<Camera>>,
) {
    if keycode.just_pressed(KeyCode::KeyP) {
        let (camera_entity, mut camera_transform, zoom_out) = camera_query.into_inner();
        let current_arena = current_arena_q.into_inner();

        if zoom_out.is_some() {
            // Camera is zoomed out, zoom back in to current arena
            // Reset camera position based on current arena
            position_camera_for_arena(&mut camera_transform, current_arena.0, ZOOM.0);
            commands.entity(camera_entity).remove::<ZoomOut>();
        } else {
            // Center camera to see all 9 arenas (middle of the 3x3 grid)
            commands.entity(camera_entity).insert(ZoomOut);
            position_camera_for_arena(&mut camera_transform, 4, ZOOM.1);
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

pub fn move_camera(
    mut commands: Commands,
    current_arena: Single<&CurrentArena, Changed<CurrentArena>>,
    camera: Single<(Entity, &mut Transform, Option<&ZoomOut>), With<Camera3d>>,
) {
    let arena = current_arena.into_inner();
    let (camera_entity, mut camera_transform, zoom) = camera.into_inner();
    if zoom.is_some() {
        // When zoomed out, don't move camera (it stays centered on all arenas)
        // The gizmo drawing is handled by draw_arena_border system
    } else {
        position_camera_for_arena(&mut camera_transform, arena.0, ZOOM.0);
        commands.entity(camera_entity).remove::<ZoomOut>();
    }
}

/// Draw a black border around the current arena when zoomed out
pub fn draw_arena_border(
    mut gizmos: Gizmos,
    current_arena: Single<&CurrentArena>,
    camera: Query<&ZoomOut, With<Camera3d>>,
) {
    // Only draw if camera is zoomed out
    if camera.single().is_err() {
        return;
    }
    
    let arena = current_arena.into_inner();
    let (offset_x, offset_y) = calculate_camera_position(arena.0);
    
    // Calculate center position of the arena
    let center_x = offset_x + ARENA_WIDTH / 2.0;
    let center_y = -offset_y - ARENA_HEIGHT / 2.0;  // Y is negative for lower arenas
    
    // Draw 5 rectangles for thickness
    for i in 0..5 {
        let thickness_offset = i as f32 * 0.02; // Small offset for each layer
        
        // Draw rectangle using rect_2d
        gizmos.rect_2d(
            Isometry2d::from_translation(Vec2::new(center_x, center_y)),
            Vec2::new(ARENA_WIDTH + thickness_offset * 2.0, ARENA_HEIGHT + thickness_offset * 2.0),
            Color::BLACK,
        );
    }
}

pub fn move_camera_on_character_arena_change(
    mut commands: Commands,
    current_arena_q: Single<&mut CurrentArena>,
    character_q: Single<(Entity, &mut Transform), (With<Character>, With<Active>)>,
    arenas: Query<Entity, With<Arena>>,
    arena_query: Query<&Arena>,
) {
    let mut current_arena = current_arena_q.into_inner();
    let (character_entity, mut character_transform) = character_q.into_inner();

    // Get the character's current position (local to current arena)
    let pos = character_transform.translation;

    // Check if character has moved outside current arena bounds
    let mut new_arena_index = current_arena.0;
    let mut new_local_pos = pos;

    // Check horizontal transitions
    if pos.x >= ARENA_WIDTH {
        // Moving right to next arena
        new_arena_index = current_arena.0 + 1;
        new_local_pos.x = pos.x - ARENA_WIDTH;
    } else if pos.x < 0.0 {
        // Moving left to previous arena
        new_arena_index = current_arena.0 - 1;
        new_local_pos.x = pos.x + ARENA_WIDTH;
    }

    // Check vertical transitions
    if pos.y >= ARENA_HEIGHT {
        // Moving up (decreasing arena row)
        new_arena_index = current_arena.0 - 3;
        new_local_pos.y = pos.y - ARENA_HEIGHT;
    } else if pos.y < 0.0 {
        // Moving down (increasing arena row)
        new_arena_index = current_arena.0 + 3;
        new_local_pos.y = pos.y + ARENA_HEIGHT;
    }

    // Clamp to valid arena range (0-8)
    new_arena_index = CurrentArena::go_to(new_arena_index);

    // Only process if character moved to a different arena
    if new_arena_index != current_arena.0 {
        // Find the target arena entity
        let target_arena_entity = arenas.iter().find(|&entity| {
            arena_query
                .get(entity)
                .map_or(false, |arena| arena.0 == new_arena_index)
        });

        if let Some(target_arena) = target_arena_entity {
            // Update character's transform to new local position
            character_transform.translation = new_local_pos;

            // Reparent the Active character to the new arena
            commands
                .entity(character_entity)
                .insert(ChildOf(target_arena));

            // Update the current arena index
            current_arena.0 = new_arena_index;

            println!(
                "Active character transitioned from Arena({}) to Arena({}) at local position {:?}",
                current_arena.0, new_arena_index, new_local_pos
            );
        }
    }
}
