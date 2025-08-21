// EXAMPLE: Reactive patterns for RecordingState (NOT RECOMMENDED for your use case)
// This demonstrates the alternatives but shows why your current approach is better

use bevy::prelude::*;
use std::time::Duration;

// Your existing types
#[derive(Resource, Debug)]
pub struct RecordingState {
    pub mode: RecordingMode,
    pub pending_request: Option<RecordingRequest>,
    pub countdown_remaining: Option<Duration>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecordingMode {
    Idle,
    Countdown,
    Recording,
    DialogPaused,
}

#[derive(Debug, Clone)]
pub enum RecordingRequest {
    Start,
    Stop,
    Commit,
    Clear,
}

#[derive(Event)]
pub struct RecordingStateChanged {
    pub old_mode: RecordingMode,
    pub new_mode: RecordingMode,
}

#[derive(Event)]
pub struct PendingRequestChanged {
    pub request: Option<RecordingRequest>,
}

// === OPTION 1: Resource Change Detection ===
// Problems: Coarse-grained, runs every frame when ANY field changes

/// System that reacts to ANY change in RecordingState
/// Problem: This runs when countdown_remaining changes every frame during countdown
fn reactive_recording_change_detector(
    recording_state: Res<RecordingState>,
    mut state_events: EventWriter<RecordingStateChanged>,
    mut last_mode: Local<Option<RecordingMode>>,
) {
    // This will trigger constantly during countdown due to countdown_remaining updates
    if recording_state.is_changed() {
        if let Some(old_mode) = *last_mode {
            if old_mode != recording_state.mode {
                state_events.write(RecordingStateChanged {
                    old_mode,
                    new_mode: recording_state.mode,
                });
                info!("Mode changed from {:?} to {:?}", old_mode, recording_state.mode);
            }
        }
        *last_mode = Some(recording_state.mode);
    }
}

/// System that reacts to pending_request changes
/// Problem: Still requires polling and local state tracking
fn reactive_pending_request_detector(
    recording_state: Res<RecordingState>,
    mut request_events: EventWriter<PendingRequestChanged>,
    mut last_request: Local<Option<Option<RecordingRequest>>>,
) {
    if recording_state.is_changed() {
        if let Some(last) = *last_request {
            // Compare current vs last pending_request
            let current = recording_state.pending_request.clone();
            if current != last {
                request_events.write(PendingRequestChanged {
                    request: current.clone(),
                });
                info!("Pending request changed: {:?}", current);
            }
        }
        *last_request = Some(recording_state.pending_request.clone());
    }
}

// === OPTION 2: Observer Pattern ===
// Better for discrete events, but adds complexity for your state machine

#[derive(Event)]
pub struct ModeChangeRequested {
    pub new_mode: RecordingMode,
    pub reason: String,
}

/// Observer that reacts to mode change requests
fn mode_change_observer(
    trigger: Trigger<ModeChangeRequested>,
    mut recording_state: ResMut<RecordingState>,
    mut commands: Commands,
) {
    let event = trigger.event();
    let old_mode = recording_state.mode;
    
    // Validate state transition
    match (old_mode, event.new_mode) {
        (RecordingMode::Idle, RecordingMode::Countdown) => {
            recording_state.mode = event.new_mode;
            recording_state.countdown_remaining = Some(Duration::from_secs(3));
            info!("Mode change: {:?} -> {:?} ({})", old_mode, event.new_mode, event.reason);
        }
        (RecordingMode::Countdown, RecordingMode::Recording) => {
            recording_state.mode = event.new_mode;
            recording_state.countdown_remaining = None;
            info!("Mode change: {:?} -> {:?} ({})", old_mode, event.new_mode, event.reason);
        }
        _ => {
            warn!("Invalid mode transition: {:?} -> {:?}", old_mode, event.new_mode);
        }
    }
}

/// Input system using observer pattern
fn observer_based_input_detection(
    keyboard: Res<ButtonInput<KeyCode>>,
    recording_state: Res<RecordingState>,
    mut mode_events: EventWriter<ModeChangeRequested>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        match recording_state.mode {
            RecordingMode::Idle => {
                mode_events.write(ModeChangeRequested {
                    new_mode: RecordingMode::Countdown,
                    reason: "User pressed R to start recording".to_string(),
                });
            }
            RecordingMode::Recording => {
                mode_events.write(ModeChangeRequested {
                    new_mode: RecordingMode::Idle,
                    reason: "User pressed R to stop recording".to_string(),
                });
            }
            _ => {}
        }
    }
}

// === OPTION 3: Fine-grained Field Watching ===
// This is what a reactive framework would provide, but requires external crates

/// Hypothetical fine-grained reactive system (would need external crate like bevy_reactor)
fn hypothetical_fine_grained_reactive() {
    // This is what you COULD do with a reactive framework:
    
    // reactive_watch!(recording_state.pending_request, |old, new| {
    //     if new.is_some() && old.is_none() {
    //         // Automatically trigger processing when request becomes Some
    //         events.write(RecordingUpdate);
    //     }
    // });
    
    // reactive_watch!(recording_state.mode, |old, new| {
    //     info!("Recording mode changed: {:?} -> {:?}", old, new);
    //     // Could automatically trigger UI updates, sound effects, etc.
    // });
}

// === WHY YOUR CURRENT APPROACH IS BETTER ===

/// Your current approach (simplified)
fn your_current_approach_is_better(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut recording_state: ResMut<RecordingState>,
    mut recording_events: EventWriter<crate::recording::RecordingUpdate>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        // Explicit, predictable event triggering
        recording_state.pending_request = Some(crate::recording::RecordingRequest::Start);
        recording_events.write(crate::recording::RecordingUpdate);
        
        // ✅ Advantages:
        // - Clear causality: input -> state change -> explicit event
        // - No polling or frame-by-frame checking
        // - Easy to test: just send RecordingUpdate event
        // - Single orchestrator handles all logic
        // - Explicit system ordering
        // - No external dependencies
        // - Performance: only runs when needed, not every frame
    }
}

// === PLUGIN DEMONSTRATING REACTIVE ALTERNATIVES ===

pub struct ReactiveRecordingExamplePlugin;

impl Plugin for ReactiveRecordingExamplePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events for reactive patterns
            .add_event::<RecordingStateChanged>()
            .add_event::<PendingRequestChanged>()
            .add_event::<ModeChangeRequested>()
            
            // Option 1: Resource change detection (not recommended)
            .add_systems(Update, (
                reactive_recording_change_detector,
                reactive_pending_request_detector,
            ))
            
            // Option 2: Observer pattern (adds complexity)
            .add_observer(mode_change_observer)
            .add_systems(Update, observer_based_input_detection);
    }
}

// === CONCLUSION ===
// 
// After implementing these alternatives, your current unified event architecture
// with RecordingUpdate as orchestrator is SUPERIOR because:
//
// 1. **Performance**: No polling every frame
// 2. **Clarity**: Explicit event flow
// 3. **Testability**: Easy to unit test
// 4. **Maintainability**: Single point of coordination
// 5. **Simplicity**: No external dependencies
// 6. **Predictability**: Deterministic execution order
//
// The reactive patterns above would actually make your code MORE complex
// and potentially LESS performant for your use case.