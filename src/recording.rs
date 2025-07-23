//! Character action recording and playback plugin.
//! 
//! This plugin handles recording character actions during gameplay sessions,
//! triggered by the R key press, and playback of saved sessions.

use bevy::prelude::*;
use crate::components::{
    Character, CharacterSelected, ArenaTimer, CharacterTimer, RecordedActions, 
    ActionEvent, ArenaName, ArenaStatus, PlaybackState, CurrentArena
};
use crate::movement::CharacterMoveEvent;

/// Plugin that handles character action recording and playback
pub struct RecordingPlugin;

impl Plugin for RecordingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_playback_toggle,
            start_playback_for_arena,
            process_playback_actions,
        ).chain().before(crate::movement::handle_movement_input))
        .add_systems(Update, (
            handle_character_deselection,
            handle_arena_transitions,
            handle_recording_toggle,
            record_movement_events,
            update_character_timers,
            stop_playback_when_complete,
        ).chain());
    }
}

/// System that stops recording when characters are deselected (Tab key effect)
fn handle_character_deselection(
    character_query: Query<(Entity, &Character, &ArenaName), With<Character>>,
    selected_query: Query<Entity, With<CharacterSelected>>,
    mut character_timer_query: Query<&mut CharacterTimer>,
    mut recorded_actions_query: Query<&mut RecordedActions>,
    _time: Res<Time>,
) {
    // Handle characters that are no longer selected (stop their active recordings)
    for (entity, character, arena_name) in character_query.iter() {
        // If this character was recording but is no longer selected, stop their recording
        if let Ok(character_timer) = character_timer_query.get(entity) {
            if character_timer.is_recording {
                // Check if this character is still selected
                if selected_query.get(entity).is_err() {
                    // Character is no longer selected, clear draft recording
                    if let Ok(mut recorded_actions) = recorded_actions_query.get_mut(entity) {
                        let arena_index = arena_name.to_index();
                        
                        // Clear the draft recording
                        recorded_actions.clear_draft();
                        
                        // Show appropriate message based on whether there's a saved session
                        if recorded_actions.has_saved_session(arena_index) {
                            println!("Tab pressed: Reverted to saved session for {} in arena {}", 
                                    character.name, arena_name.name());
                        } else {
                            println!("Tab pressed: Reverted to empty state (no saved session) for {} in arena {}", 
                                    character.name, arena_name.name());
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
    _time: Res<Time>,
) {
    for (entity, character, new_arena) in character_query.iter() {
        // Character transitioned to a new arena, cancel any active recording
        if let Ok(character_timer) = character_timer_query.get(entity) {
            if character_timer.is_recording {
                if let Ok(mut recorded_actions) = recorded_actions_query.get_mut(entity) {
                    // Clear the draft recording
                    recorded_actions.clear_draft();
                    println!("Arena transition: Cancelled active recording for character {} entering {}", 
                            character.name, new_arena.name());
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
                        
                        // Set arena status to Recording and reset timer
                        if let Some(mut arena_timer) = arena_timer_query.iter_mut()
                            .find(|at| at.arena == *arena_name) {
                            arena_timer.set_status(ArenaStatus::Recording);
                            arena_timer.timer.reset();
                            println!("Arena {} status changed to Recording, timer reset", arena_name.name());
                        }
                        
                        // Start new recording for this arena and record initial position
                        let arena_index = arena_name.to_index();
                        let had_previous = recorded_actions.start_recording(arena_index, timestamp);
                        
                        if had_previous {
                            println!("R pressed: Started new recording for arena {} (replaced previous)", 
                                    arena_name.name());
                        } else {
                            println!("R pressed: Started new recording for arena {}", 
                                    arena_name.name());
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
                        
                        // Set arena status back to Paused
                        if let Some(mut arena_timer) = arena_timer_query.iter_mut()
                            .find(|at| at.arena == *arena_name) {
                            arena_timer.set_status(ArenaStatus::Paused);
                            println!("Arena {} status changed to Paused", arena_name.name());
                        }
                        
                        let arena_index = arena_name.to_index();
                        
                        // Save the recording session BEFORE stopping (while still active)
                        let saved_successfully = recorded_actions.save_current_session(arena_index);
                        let stopped_successfully = recorded_actions.stop_recording(timestamp);
                        
                        if stopped_successfully {
                            
                            let total_recorded_arenas = recorded_actions.count_recorded_arenas();
                            if saved_successfully {
                                println!("Stopped and saved recording in arena {}. Total recorded arenas: {}", 
                                        arena_name.name(), total_recorded_arenas);
                            } else {
                                println!("Stopped recording in arena {} (save failed). Total recorded arenas: {}", 
                                        arena_name.name(), total_recorded_arenas);
                            }
                            
                            // Print summary of the just-completed recording (now saved)
                            if let Some(recording) = recorded_actions.get_saved_recording(arena_index) {
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
                    
                    // Set arena status to Recording and reset timer
                    if let Some(mut arena_timer) = arena_timer_query.iter_mut()
                        .find(|at| at.arena == *arena_name) {
                        arena_timer.set_status(ArenaStatus::Recording);
                        arena_timer.timer.reset();
                        println!("Arena {} status changed to Recording, timer reset", arena_name.name());
                    }
                    
                    // Start new recording for this arena and record initial position
                    let arena_index = arena_name.to_index();
                    let _had_previous = new_actions.start_recording(arena_index, timestamp);
                    
                    println!("R pressed: Started first recording for arena {}", 
                            arena_name.name());
                    
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
                    let added = recorded_actions.add_action(ActionEvent::Move {
                        direction: event.direction,
                        timestamp,
                    });
                    if added {
                        println!("Recorded movement: {:?} at t={:.2}s in arena {} (absolute timestamp)", 
                                 event.direction, timestamp, arena_name.name());
                    } else {
                        println!("Failed to record movement - no active recording");
                    }
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

/// System that handles the '=' key press to toggle arena playback status
fn handle_playback_toggle(
    current_arena_query: Query<&CurrentArena>,
    mut arena_timer_query: Query<&mut ArenaTimer>,
    input: Res<ButtonInput<KeyCode>>,
) {
    
    if input.just_pressed(KeyCode::Equal) {
        println!("= key pressed!");
        if let Ok(current_arena) = current_arena_query.single() {
            println!("Current arena index: {}", current_arena.0);
            let target_arena = ArenaName::from_index(current_arena.0);
            
            // Find the arena timer for the current arena
            if let Some(mut arena_timer) = arena_timer_query.iter_mut()
                .find(|at| at.arena == target_arena) {
                
                // Toggle to Playback status
                match arena_timer.get_status() {
                    ArenaStatus::Playback => {
                        // If already in playback, toggle to Paused
                        arena_timer.set_status(ArenaStatus::Paused);
                        println!("Arena {} status changed from Playback to Paused", target_arena.name());
                    }
                    _ => {
                        // Toggle to Playback from any other state
                        arena_timer.set_status(ArenaStatus::Playback);
                        arena_timer.timer.reset(); // Reset timer to start from beginning
                        println!("Arena {} status changed to Playback", target_arena.name());
                    }
                }
            }
        }
    }
}

/// System that starts playback for all characters in an arena when status changes to Playback
fn start_playback_for_arena(
    mut commands: Commands,
    arena_timer_query: Query<&ArenaTimer, Changed<ArenaTimer>>,
    mut character_query: Query<(Entity, &Character, &ArenaName, Option<&RecordedActions>, &mut Transform)>,
    playback_query: Query<Entity, With<PlaybackState>>,
) {
    for arena_timer in arena_timer_query.iter() {
        if arena_timer.is_playback() {
            println!("Starting playback for arena: {}", arena_timer.arena.name());
            
            // Remove existing playback states
            for entity in playback_query.iter() {
                commands.entity(entity).remove::<PlaybackState>();
            }
            
            // Start playback for all characters with saved sessions in this arena
            for (entity, character, arena_name, recorded_actions, mut transform) in character_query.iter_mut() {
                if *arena_name == arena_timer.arena {
                    println!("Checking {} in arena {} for saved recordings", character.name, arena_name.name());
                    if let Some(recorded_actions) = recorded_actions {
                            if let Some(saved_recording) = recorded_actions.get_saved_recording(arena_timer.arena.to_index()) {
                                println!("Found saved recording for {} with {} actions", character.name, saved_recording.actions.len());
                                if let Some(playback_state) = PlaybackState::new(saved_recording.clone()) {
                                    commands.entity(entity).insert(playback_state);
                                    
                                    // Reset character to starting position
                                    if let Some(first_action) = saved_recording.actions.first() {
                                        if let ActionEvent::Position { x, y, .. } = first_action {
                                            transform.translation.x = *x;
                                            transform.translation.y = *y;
                                            println!("Reset {} to starting position ({}, {})", character.name, x, y);
                                        }
                                    }
                                    
                                    println!("Started playback for {} in arena {} with {} actions", 
                                            character.name, arena_name.name(), saved_recording.actions.len());
                                    println!("  Recording start time: {:.2}s, end time: {:.2}s", 
                                            saved_recording.session_start_time, 
                                            saved_recording.session_end_time.unwrap_or(0.0));
                                    
                                    // Debug: show first few actions
                                    for (i, action) in saved_recording.actions.iter().take(3).enumerate() {
                                        match action {
                                            ActionEvent::Position { x, y, timestamp } => {
                                                println!("  Action {}: Position ({}, {}) at t={:.2}s (relative: {:.2}s)", 
                                                        i, x, y, timestamp, timestamp - saved_recording.session_start_time);
                                            }
                                            ActionEvent::Move { direction, timestamp } => {
                                                println!("  Action {}: Move {:?} at t={:.2}s (relative: {:.2}s)", 
                                                        i, direction, timestamp, timestamp - saved_recording.session_start_time);
                                            }
                                        }
                                    }
                                }
                            } else {
                                println!("No saved recording found for {} in arena {}", character.name, arena_name.name());
                            }
                        } else {
                            println!("No RecordedActions component for {} in arena {}", character.name, arena_name.name());
                        }
                }
            }
        } else if matches!(arena_timer.get_status(), ArenaStatus::Paused | ArenaStatus::Recording) {
            // Remove playback states when arena is no longer in playback mode
            for (entity, _, arena_name, _, _) in character_query.iter_mut() {
                if *arena_name == arena_timer.arena {
                    commands.entity(entity).remove::<PlaybackState>();
                }
            }
        }
    }
}

/// System that processes playback actions based on arena timer
fn process_playback_actions(
    mut playback_query: Query<(Entity, &mut PlaybackState, &mut Transform, &Character)>,
    arena_timer_query: Query<&ArenaTimer>,
    mut move_events: EventWriter<CharacterMoveEvent>,
) {
    for (entity, mut playback_state, mut transform, character) in playback_query.iter_mut() {
        // Find the arena timer for this recording's arena
        let arena_name = ArenaName::from_index(playback_state.recording.arena_index);
        if let Some(arena_timer) = arena_timer_query.iter()
            .find(|at| at.arena == arena_name && at.is_playback()) {
            
            let elapsed = arena_timer.timer.elapsed_secs_f64();
            
            // Process all actions that should have occurred by this time
            while let Some(action) = playback_state.get_current_action(elapsed) {
                match action {
                    ActionEvent::Position { x, y, timestamp } => {
                        // For position events, directly set the transform
                        transform.translation.x = *x;
                        transform.translation.y = *y;
                        println!("Playback: {} position set to ({}, {}) at relative time {:.2}s", 
                                character.name, x, y, timestamp - playback_state.recording.session_start_time);
                    }
                    ActionEvent::Move { direction, timestamp } => {
                        // For movement events, emit a movement event
                        move_events.write(CharacterMoveEvent::new(entity, *direction));
                        println!("Playback: {} move {:?} at relative time {:.2}s (elapsed: {:.2}s)", 
                                character.name, direction, timestamp - playback_state.recording.session_start_time, elapsed);
                    }
                }
                
                playback_state.advance_action();
            }
        }
    }
}

/// System that stops playback when all actions are complete or arena status changes
fn stop_playback_when_complete(
    mut commands: Commands,
    playback_query: Query<(Entity, &PlaybackState)>,
    arena_timer_query: Query<&ArenaTimer>,
) {
    for (entity, playback_state) in playback_query.iter() {
        let arena_name = ArenaName::from_index(playback_state.recording.arena_index);
        
        // Check if the arena is still in playback mode
        let still_in_playback = arena_timer_query.iter()
            .any(|at| at.arena == arena_name && at.is_playback());
        
        // Remove playback state if complete or arena is no longer in playback
        if playback_state.is_complete() || !still_in_playback {
            commands.entity(entity).remove::<PlaybackState>();
            if playback_state.is_complete() {
                println!("Playback complete for entity in arena {}", arena_name.name());
            }
        }
    }
}