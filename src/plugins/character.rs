//! Character management plugin.
//! 
//! This plugin handles character spawning, movement, selection, and sprite updates.

use bevy::prelude::*;
use crate::{
    bundles::{CharacterBundle, SelectedCharacterBundle},
    components::{Character, CharacterSelected},
    config::{assets::*, display::*},
    plugins::input::CharacterActionEvent,
    utils::{calculate_character_position, clamp_to_grid_boundaries},
};

/// Plugin responsible for character-related functionality
pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_characters)
            .add_systems(
                Update,
                (
                    handle_character_movement_events,
                    handle_character_selection_events,
                    update_character_sprites,
                ),
            );
    }
}

/// System that spawns initial characters in the game
fn spawn_initial_characters(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn first character at tile position (33, 15) in arena 1 (center of the arena)
    let (char1_x, char1_y) = calculate_character_position(1, 33, 15);
    commands.spawn(SelectedCharacterBundle::new(
        &asset_server,
        char1_x,
        char1_y,
    ));

    // Spawn second character at tile position (30, 15) in arena 1 (3 tiles to the left)
    let (char2_x, char2_y) = calculate_character_position(1, 30, 15);
    commands.spawn(CharacterBundle::new(&asset_server, char2_x, char2_y, false));
}

/// System that handles character movement events
fn handle_character_movement_events(
    mut character_events: EventReader<CharacterActionEvent>,
    mut player_query: Query<&mut Transform, With<CharacterSelected>>,
) {
    for event in character_events.read() {
        if let Ok(mut transform) = player_query.single_mut() {
            let mut new_x = transform.translation.x;
            let mut new_y = transform.translation.y;

            match event {
                CharacterActionEvent::MoveLeft => new_x -= TILE_SIZE,
                CharacterActionEvent::MoveRight => new_x += TILE_SIZE,
                CharacterActionEvent::MoveUp => new_y += TILE_SIZE,
                CharacterActionEvent::MoveDown => new_y -= TILE_SIZE,
                CharacterActionEvent::CycleCharacter => {
                    // Handled by separate system
                }
            }

            // Clamp position to stay within the 3x3 grid boundaries
            let (clamped_x, clamped_y) = clamp_to_grid_boundaries(new_x, new_y);

            // Apply the clamped position
            transform.translation.x = clamped_x;
            transform.translation.y = clamped_y;
        }
    }
}

/// System that handles character selection cycling events
fn handle_character_selection_events(
    mut character_events: EventReader<CharacterActionEvent>,
    mut commands: Commands,
    characters_query: Query<Entity, With<Character>>,
    selected_query: Query<Entity, With<CharacterSelected>>,
) {
    for event in character_events.read() {
        if matches!(event, CharacterActionEvent::CycleCharacter) {
            // Get all character entities as a Vec
            let characters: Vec<Entity> = characters_query.iter().collect();

            if characters.len() <= 1 {
                continue; // No cycling needed with 0 or 1 characters
            }

            // Find current selected character index using cleaner Rust 2024 patterns
            let next_index = selected_query
                .single()
                .ok()
                .and_then(|selected_entity| characters.iter().position(|&e| e == selected_entity))
                .map(|current_index| (current_index + 1) % characters.len())
                .unwrap_or(0);

            // Remove CharacterSelected from all characters
            for entity in &characters {
                commands.entity(*entity).remove::<CharacterSelected>();
            }

            // Add CharacterSelected to next character
            commands
                .entity(characters[next_index])
                .insert(CharacterSelected);
        }
    }
}

/// System that updates character sprites based on selection state
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