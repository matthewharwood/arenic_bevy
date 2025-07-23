//! Game plugins for organizing systems into logical groups.
//! 
//! This module contains plugin implementations that group related systems
//! together, following Bevy's recommended architecture patterns.

use bevy::prelude::*;
use crate::{
    setup, spawn_player_selected,
    handle_arena_navigation_keys, update_camera_on_arena_change, handle_zoom_toggle, draw_arena_gizmo,
    move_selected_player, cycle_selected_character, update_character_sprites,
    update_character_arena_markers, sync_current_arena_with_selected_character,
    ensure_character_selected_in_current_arena, debug_character_arena_changes,
};

/// Plugin responsible for arena-related functionality including navigation,
/// camera management, and gizmo rendering.
pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            setup,
        )
        .add_systems(
            Update,
            (
                handle_arena_navigation_keys,
                update_camera_on_arena_change,
                handle_zoom_toggle,
                draw_arena_gizmo,
            ),
        );
    }
}

/// Plugin responsible for character-related functionality including movement,
/// selection, and arena tracking.
pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            spawn_player_selected,
        )
        .add_systems(
            Update,
            (
                move_selected_player,
                cycle_selected_character,
                update_character_sprites,
                update_character_arena_markers,
                sync_current_arena_with_selected_character,
                ensure_character_selected_in_current_arena,
                debug_character_arena_changes,
            ),
        );
    }
}

