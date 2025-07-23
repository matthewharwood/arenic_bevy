//! Arena management plugin.
//! 
//! This plugin handles arena navigation, gizmo rendering, and arena-related
//! game logic including character tracking across arenas.

use bevy::prelude::*;
use crate::{
    components::{ArenaName, Character, CharacterSelected, CurrentArena, CyclicNavigation},
    config::{arena::*, camera::*, display::*},
    plugins::input::ArenaActionEvent,
};

/// Plugin responsible for arena-related functionality
pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_arena_navigation_events,
                draw_arena_gizmo,
                update_character_arena_markers,
                sync_current_arena_with_selected_character,
                ensure_character_selected_in_current_arena,
                debug_character_arena_changes,
            ),
        );
    }
}

/// System that handles arena navigation events
fn handle_arena_navigation_events(
    mut arena_events: EventReader<ArenaActionEvent>,
    mut arena_query: Query<&mut CurrentArena>,
) {
    for event in arena_events.read() {
        match event {
            ArenaActionEvent::NextArena => {
                for mut arena in &mut arena_query {
                    *arena = arena.increment();
                }
            }
            ArenaActionEvent::PreviousArena => {
                for mut arena in &mut arena_query {
                    *arena = arena.decrement();
                }
            }
            ArenaActionEvent::ToggleZoom => {
                // Zoom is handled by the camera plugin
            }
        }
    }
}

/// System that draws arena gizmos when zoomed out
fn draw_arena_gizmo(
    mut gizmos: Gizmos,
    arena_query: Query<&CurrentArena>,
    camera_query: Query<&Projection, With<Camera>>,
) {
    for projection in &camera_query {
        if matches!(projection, Projection::Orthographic(ortho) if ortho.scale == SCALE_ZOOMED_OUT) {
            for arena in &arena_query {
                let arena_col = arena.0 % 3;
                let arena_row = arena.0 / 3;

                let arena_center_x =
                    -HALF_WINDOW_WIDTH + (arena_col as f32 * ARENA_WIDTH) + ARENA_WIDTH / 2.0;
                let arena_center_y =
                    HALF_WINDOW_HEIGHT - (arena_row as f32 * ARENA_HEIGHT) - ARENA_HEIGHT / 2.0;
                let arena_center = Vec2::new(arena_center_x, arena_center_y);

                let border_thickness = 10.0;
                let border_color = Color::BLACK;

                // Draw border using concentric rectangles
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

/// System that updates character arena markers based on their position
fn update_character_arena_markers(
    mut commands: Commands,
    mut character_query: Query<(Entity, &Transform, Option<&ArenaName>), With<Character>>,
) {
    for (entity, transform, current_arena_name) in &mut character_query {
        let x = transform.translation.x;
        let y = transform.translation.y;

        // Calculate which arena this character is in based on position
        let arena_col = ((x + HALF_WINDOW_WIDTH) / ARENA_WIDTH).floor() as i32;
        let arena_row = ((HALF_WINDOW_HEIGHT - y) / ARENA_HEIGHT).floor() as i32;

        // Clamp to valid arena bounds (0-2 for both col and row)
        let arena_col = arena_col.clamp(0, 2) as u8;
        let arena_row = arena_row.clamp(0, 2) as u8;

        let arena_index = arena_row * 3 + arena_col;
        let new_arena_name = ArenaName::from_index(arena_index);

        // Only update if the arena has changed
        if current_arena_name != Some(&new_arena_name) {
            commands.entity(entity).insert(new_arena_name);
        }
    }
}

/// System that syncs CurrentArena with the selected character's arena
fn sync_current_arena_with_selected_character(
    mut arena_query: Query<&mut CurrentArena>,
    selected_character_query: Query<&ArenaName, (With<CharacterSelected>, Changed<ArenaName>)>,
) {
    if let Ok(arena_name) = selected_character_query.single() {
        for mut current_arena in &mut arena_query {
            current_arena.0 = arena_name.to_index();
            println!(
                "CurrentArena updated to: {} (index: {})",
                arena_name.name(),
                arena_name.to_index()
            );
        }
    }
}

/// System that ensures a character is selected in the current arena
fn ensure_character_selected_in_current_arena(
    mut commands: Commands,
    current_arena_query: Query<&CurrentArena, Changed<CurrentArena>>,
    selected_character_query: Query<&ArenaName, With<CharacterSelected>>,
    all_characters_query: Query<(Entity, &ArenaName), With<Character>>,
) {
    if let Ok(current_arena) = current_arena_query.single() {
        let target_arena = ArenaName::from_index(current_arena.0);

        // Check if there's already a selected character in the current arena
        let has_selected_in_arena = selected_character_query
            .single()
            .map(|arena_name| *arena_name == target_arena)
            .unwrap_or(false);

        if !has_selected_in_arena {
            // Find the first character in the target arena
            if let Some((character_entity, _)) = all_characters_query
                .iter()
                .find(|(_, arena_name)| **arena_name == target_arena)
            {
                // Remove CharacterSelected from all characters first
                for (entity, _) in all_characters_query.iter() {
                    commands.entity(entity).remove::<CharacterSelected>();
                }

                // Add CharacterSelected to the found character
                commands.entity(character_entity).insert(CharacterSelected);

                println!("Auto-selected character in arena: {}", target_arena.name());
            }
        }
    }
}

/// Debug system for character arena changes
fn debug_character_arena_changes(
    query: Query<&ArenaName, (With<CharacterSelected>, Changed<ArenaName>)>,
) {
    if let Ok(arena_name) = query.single() {
        println!("CharacterSelected entered arena: {}", arena_name.name());
    }
}