//! Character action recording plugin.
//! 
//! This plugin handles recording character actions during gameplay sessions,
//! triggered by the R key press.

use bevy::prelude::*;
use crate::components::{
    Character, CharacterSelected, ArenaTimer, CharacterTimer, RecordedActions, 
    ActionEvent, ArenaName, SessionVersion
};
use crate::movement::CharacterMoveEvent;

/// Plugin that handles character action recording
pub struct RecordingPlugin;

impl Plugin for RecordingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_character_deselection,
            handle_arena_transitions,
            handle_recording_toggle,
            record_movement_events,
            update_character_timers,
        ).chain());
    }
}

/// System that stops recording when characters are deselected (Tab key effect)
fn handle_character_deselection(
    character_query: Query<(Entity, &Character, &ArenaName), With<Character>>,
    selected_query: Query<Entity, With<CharacterSelected>>,
    mut character_timer_query: Query<&mut CharacterTimer>,
    mut recorded_actions_query: Query<&mut RecordedActions>,
    time: Res<Time>,
) {
    // Handle characters that are no longer selected (stop their active recordings)
    for (entity, character, arena_name) in character_query.iter() {
        // If this character was recording but is no longer selected, stop their recording
        if let Ok(character_timer) = character_timer_query.get(entity) {
            if character_timer.is_recording {
                // Check if this character is still selected
                if selected_query.get(entity).is_err() {
                    // Character is no longer selected, implement revert logic
                    if let Ok(mut recorded_actions) = recorded_actions_query.get_mut(entity) {
                        let _timestamp = time.elapsed_secs_f64();
                        let arena_index = arena_name.to_index();
                        
                        // Check if there was a saved session
                        if recorded_actions.has_saved_session(arena_index) {
                            // Revert to saved session
                            if recorded_actions.revert_to_saved_session(arena_index) {
                                println!("Tab pressed: Reverted to saved session for {} in arena {}", 
                                        character.name, arena_name.name());
                            }
                        } else {
                            // No saved session, revert to empty state
                            if recorded_actions.revert_to_saved_session(arena_index) {
                                println!("Tab pressed: Reverted to empty state (no saved session) for {} in arena {}", 
                                        character.name, arena_name.name());
                            }
                        }
                    }
                    
                    if let Ok(mut character_timer) = character_timer_query.get_mut(entity) {
                        character_timer.stop_recording();
                    }
                }
            }
        }
    }
}

/// System that handles arena transitions and cancels active recordings
fn handle_arena_transitions(
    character_query: Query<(Entity, &Character, &ArenaName), (With<Character>, Changed<ArenaName>)>,
    mut character_timer_query: Query<&mut CharacterTimer>,
    mut recorded_actions_query: Query<&mut RecordedActions>,
    time: Res<Time>,
) {
    for (entity, character, new_arena) in character_query.iter() {
        // Character transitioned to a new arena, cancel any active recording
        if let Ok(character_timer) = character_timer_query.get(entity) {
            if character_timer.is_recording {
                if let Ok(mut recorded_actions) = recorded_actions_query.get_mut(entity) {
                    let timestamp = time.elapsed_secs_f64();
                    
                    // Stop the current recording
                    if recorded_actions.stop_recording(timestamp) {
                        println!("Arena transition: Cancelled active recording for character {} entering {}", 
                                character.name, new_arena.name());
                    }
                }
                
                // Stop the character timer
                if let Ok(mut character_timer) = character_timer_query.get_mut(entity) {
                    character_timer.stop_recording();
                }
            }
        }
    }
}

/// System that handles R key press to toggle recording
fn handle_recording_toggle(
    mut commands: Commands,
    selected_query: Query<(Entity, &Character, &Transform, &ArenaName), With<CharacterSelected>>,
    mut arena_timer_query: Query<&mut ArenaTimer>,
    mut character_timer_query: Query<&mut CharacterTimer>,
    mut recorded_actions_query: Query<&mut RecordedActions>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        if let Ok((selected_entity, character, transform, arena_name)) = selected_query.single() {
            let timestamp = time.elapsed_secs_f64();
            
            // Handle both cases: existing components or missing components
            match (character_timer_query.get_mut(selected_entity), recorded_actions_query.get_mut(selected_entity)) {
                (Ok(mut character_timer), Ok(mut recorded_actions)) => {
                    // Both components exist, handle normal toggle logic
                    if !character_timer.is_recording {
                        // Start recording
                        character_timer.start_recording();
                        
                        // Reset arena timer for this arena
                        if let Some(mut arena_timer) = arena_timer_query.iter_mut()
                            .find(|at| at.arena == *arena_name) {
                            arena_timer.timer.reset();
                            arena_timer.timer.unpause();
                            println!("Arena timer reset and started for arena: {}", arena_name.name());
                        }
                        
                        // Start new recording for this arena and record initial position
                        let arena_index = arena_name.to_index();
                        let had_previous = recorded_actions.start_recording(arena_index, timestamp);
                        let current_version = recorded_actions.get_arena_recording_version(arena_index)
                            .map(|v| v.name()).unwrap_or("Unknown");
                        
                        if had_previous {
                            println!("R pressed: Started new Recording Session '{}' for arena {} (replaced previous)", 
                                    current_version, arena_name.name());
                        } else {
                            println!("R pressed: Started new Recording Session '{}' for arena {}", 
                                    current_version, arena_name.name());
                        }
                        
                        recorded_actions.add_action(ActionEvent::Position {
                            x: transform.translation.x,
                            y: transform.translation.y,
                            timestamp,
                        });
                        
                        println!("Started recording for character at position ({}, {})", 
                                 transform.translation.x, transform.translation.y);
                    } else {
                        // Stop recording
                        character_timer.stop_recording();
                        
                        let arena_index = arena_name.to_index();
                        let stopped_successfully = recorded_actions.stop_recording(timestamp);
                        
                        if stopped_successfully {
                            // Save the completed recording session
                            let _saved_successfully = recorded_actions.save_current_session(arena_index);
                            
                            let has_recording = recorded_actions.has_recording_for_arena(arena_index);
                            let total_recorded_arenas = recorded_actions.count_recorded_arenas();
                            let session_version = recorded_actions.get_arena_recording_version(arena_index)
                                .map(|v| v.name()).unwrap_or("Unknown");
                            println!("Stopped and saved Recording Session '{}' in arena {}. Total recorded arenas: {}", 
                                    session_version, arena_name.name(), total_recorded_arenas);
                            
                            // Print summary of the recording for this arena
                            if let Some(recording) = recorded_actions.get_arena_recording(arena_index) {
                                println!("Recording for {} in arena {}: {} actions (start: {:.2}s, end: {:.2}s)", 
                                        character.name, arena_name.name(),
                                        recording.actions.len(),
                                        recording.session_start_time,
                                        recording.session_end_time.unwrap_or(0.0));
                                
                                // Print first few actions
                                for (i, action) in recording.actions.iter().take(5).enumerate() {
                                    match action {
                                        ActionEvent::Position { x, y, timestamp } => {
                                            println!("  {}: Position ({}, {}) at t={:.2}s", i, x, y, timestamp);
                                        }
                                        ActionEvent::Move { direction, timestamp } => {
                                            println!("  {}: Move {:?} at t={:.2}s", i, direction, timestamp);
                                        }
                                    }
                                }
                                if recording.actions.len() > 5 {
                                    println!("  ... {} more actions", recording.actions.len() - 5);
                                }
                            }
                            
                            // Print all arenas with recordings
                            let recorded_arenas = recorded_actions.get_recorded_arena_indices();
                            if !recorded_arenas.is_empty() {
                                println!("Character has recordings in arenas: {:?}", recorded_arenas);
                            }
                        }
                    }
                }
                _ => {
                    // Components don't exist, add them and start recording immediately
                    println!("Adding recording components and starting first recording for character");
                    
                    // Create new components
                    let mut new_timer = CharacterTimer::new();
                    let mut new_actions = RecordedActions::default();
                    
                    // Start recording immediately
                    new_timer.start_recording();
                    
                    // Reset arena timer for this arena
                    if let Some(mut arena_timer) = arena_timer_query.iter_mut()
                        .find(|at| at.arena == *arena_name) {
                        arena_timer.timer.reset();
                        arena_timer.timer.unpause();
                        println!("Arena timer reset and started for arena: {}", arena_name.name());
                    }
                    
                    // Start new recording for this arena and record initial position
                    let arena_index = arena_name.to_index();
                    let _had_previous = new_actions.start_recording(arena_index, timestamp);
                    let current_version = new_actions.get_arena_recording_version(arena_index)
                        .map(|v| v.name()).unwrap_or("Unknown");
                    
                    println!("R pressed: Started first Recording Session '{}' for arena {}", 
                            current_version, arena_name.name());
                    
                    new_actions.add_action(ActionEvent::Position {
                        x: transform.translation.x,
                        y: transform.translation.y,
                        timestamp,
                    });
                    
                    // Insert components into the entity
                    commands.entity(selected_entity).insert((new_timer, new_actions));
                    
                    println!("Started first recording for character at position ({}, {})", 
                             transform.translation.x, transform.translation.y);
                }
            }
        }
    }
}

/// System that records movement events when recording is active
fn record_movement_events(
    mut move_events: EventReader<CharacterMoveEvent>,
    character_query: Query<(&CharacterTimer, &ArenaName)>,
    mut recorded_actions_query: Query<&mut RecordedActions>,
    time: Res<Time>,
) {
    for event in move_events.read() {
        // Check if this character is recording and get their current arena
        if let Ok((character_timer, arena_name)) = character_query.get(event.entity) {
            if character_timer.is_recording {
                if let Ok(mut recorded_actions) = recorded_actions_query.get_mut(event.entity) {
                    let timestamp = time.elapsed_secs_f64();
                    recorded_actions.add_action(ActionEvent::Move {
                        direction: event.direction,
                        timestamp,
                    });
                    println!("Recorded movement: {:?} at t={:.2}s in arena {}", 
                             event.direction, timestamp, arena_name.name());
                }
            }
        }
    }
}

/// System that updates character timers
fn update_character_timers(
    mut character_timer_query: Query<&mut CharacterTimer>,
    time: Res<Time>,
) {
    for mut character_timer in &mut character_timer_query {
        if character_timer.is_recording {
            character_timer.timer.tick(time.delta());
        }
    }
}