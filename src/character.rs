//! Character management plugin.
//!
//! This plugin handles all character functionality, including spawning, selection,
//! arena tracking, and character-arena interactions.

use crate::bundles::{CharacterBundle, SelectedCharacterBundle};
use crate::components::{ArenaName, ArenaTimer, Character, CharacterSelected, CurrentArena};
use crate::config::arena::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::config::assets::{PLAYER_SELECTED, PLAYER_UNSELECTED};
use crate::config::display::{HALF_WINDOW_HEIGHT, HALF_WINDOW_WIDTH};
use crate::utils::calculate_character_position;
use bevy::prelude::*;

/// Plugin that handles character systems
pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_selected).add_systems(
            Update,
            (
                cycle_selected_character,
                update_character_sprites,
                update_character_arena_markers,
                sync_current_arena_with_selected_character,
                ensure_character_selected_in_current_arena,
                debug_character_arena_changes,
                activate_arena_timers_on_character_entry,
            ),
        );
    }
}

/// Spawn initial player characters
fn spawn_player_selected(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the first character at tile position (33, 15) in arena 1 (center of the arena)
    let (char1_x, char1_y) = calculate_character_position(1, 33, 15);
    commands.spawn(SelectedCharacterBundle::new(
        &asset_server,
        char1_x,
        char1_y,
        "Dean",
    ));

    // Spawn the second character at tile position (30, 15) in arena 1 (3 tiles to the left)
    let (char2_x, char2_y) = calculate_character_position(1, 30, 15);
    commands.spawn(CharacterBundle::new(
        &asset_server,
        char2_x,
        char2_y,
        false,
        "Matt",
    ));
}

/// Handle a Tab key to cycle through characters
fn cycle_selected_character(
    mut commands: Commands,
    characters_query: Query<Entity, With<Character>>,
    selected_query: Query<Entity, With<CharacterSelected>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Tab) {
        // Get all character entities as a Vec
        let characters: Vec<Entity> = characters_query.iter().collect();

        if characters.len() <= 1 {
            return; // No cycling needed with 0 or 1 characters
        }

        // Find the current selected character index
        let current_selected = selected_query.single();

        let next_index = match current_selected {
            Ok(selected_entity) => {
                // Find the current index and get the next index cyclically
                if let Some(current_index) = characters.iter().position(|&e| e == selected_entity) {
                    (current_index + 1) % characters.len()
                } else {
                    0 // Default to first if not found
                }
            }
            Err(_) => 0, // No current selection, start with first
        };

        // Remove CharacterSelected from all characters
        for entity in &characters {
            commands.entity(*entity).remove::<CharacterSelected>();
        }

        // Add CharacterSelected to the next character
        commands
            .entity(characters[next_index])
            .insert(CharacterSelected);
    }
}

/// Update character sprites based on selection status
fn update_character_sprites(
    mut character_query: Query<(Entity, &mut Sprite), With<Character>>,
    selected_query: Query<Entity, With<CharacterSelected>>,
    asset_server: Res<AssetServer>,
) {
    let selected_entity = selected_query.single().ok();

    for (entity, mut sprite) in &mut character_query {
        if Some(entity) == selected_entity {
            sprite.image = asset_server.load(PLAYER_SELECTED);
        } else {
            sprite.image = asset_server.load(PLAYER_UNSELECTED);
        }
    }
}

/// Update character arena markers based on position
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

/// Sync the current arena with the selected character's arena
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

/// Debug output for character arena changes
fn debug_character_arena_changes(
    query: Query<&ArenaName, (With<CharacterSelected>, Changed<ArenaName>)>,
) {
    if let Ok(arena_name) = query.single() {
        println!("CharacterSelected entered arena: {}", arena_name.name());
    }
}

/// Ensure there's always a selected character in the current arena
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
            let first_character_in_arena = all_characters_query
                .iter()
                .find(|(_, arena_name)| **arena_name == target_arena)
                .map(|(entity, _)| entity);

            if let Some(character_entity) = first_character_in_arena {
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

/// Activate arena timers when a selected character enters an arena
fn activate_arena_timers_on_character_entry(
    mut timer_query: Query<&mut ArenaTimer>,
    selected_character_query: Query<&ArenaName, (With<CharacterSelected>, Changed<ArenaName>)>,
) {
    // Only update timer status when a selected character enters an arena
    if let Ok(arena_name) = selected_character_query.single() {
        if let Some(arena_timer) = timer_query.iter_mut().find(|at| at.arena == *arena_name) {
            // Only change status if currently paused
            if arena_timer.is_paused() {
                // Keep the timer paused but log entry
                println!(
                    "Selected character entered arena: {} (status: {:?})",
                    arena_name.name(),
                    arena_timer.get_status()
                );
            } else {
                // Timer is already in Playback mode
                println!(
                    "Selected character entered arena: {} (status: {:?} - continuing)",
                    arena_name.name(),
                    arena_timer.get_status()
                );
            }
        }
    }
}
