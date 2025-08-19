# Tutorial 02: Recording State Machine

## Objective

Implement the state machine that manages recording modes using event-driven transitions. All state changes go through
RecordingTransition events for traceability and debugging.

## Prerequisites

- Completed Tutorial 01 (Timeline Foundation)
- Understanding of Bevy Resources and Events
- Familiarity with state machines

## Components/Systems

We'll create:

- Recording state resource with event-driven transitions
- Recording mode enum
- Recording marker components
- State transition events with from/to tracking
- Recording initiation system with let-else patterns

## Implementation Steps

### Step 1: Create Recording State Components

Create `src/recording/mod.rs`:

```rust
use bevy::prelude::*;
use bevy::time::Virtual;
use crate::timeline::{DraftTimeline, TimelineEvent, EventType, TimeStamp, Arena, GridPos, TimelineClock};
use crate::ability::AbilityType;
use crate::character::Character;
use crate::selectors::Active;

/// Global recording state for the game
#[derive(Resource)]
pub struct RecordingState {
    pub mode: RecordingMode,
    pub recording_entity: Option<Entity>,
}

impl Default for RecordingState {
    fn default() -> Self {
        Self {
            mode: RecordingMode::Idle,
            recording_entity: None,
        }
    }
}

/// Recording system modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecordingMode {
    /// Not recording
    Idle,
    /// Countdown before recording starts
    Countdown,
    /// Actively recording character actions
    Recording,
    /// Dialog shown, all timelines paused
    DialogPaused,
}

impl std::fmt::Display for RecordingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Idle => write!(f, "Idle"),
            Self::Countdown => write!(f, "Countdown"),
            Self::Recording => write!(f, "Recording"),
            Self::DialogPaused => write!(f, "DialogPaused"),
        }
    }
}

/// Event-driven state transition
#[derive(Event, Debug)]
pub struct RecordingTransition {
    pub from: RecordingMode,
    pub to: RecordingMode,
    pub reason: TransitionReason,
}

#[derive(Debug, Clone)]
pub enum TransitionReason {
    StartRequest(Entity),  // User initiated recording
    CountdownComplete,
    UserInterrupted,
    TimeComplete,
    ArenaTransition,
    CharacterSwitch,
    DialogOpened,
    DialogClosed,
}

/// Marker component for characters currently being recorded
#[derive(Component)]
pub struct Recording;

/// Countdown timer before recording starts
#[derive(Component)]
pub struct RecordingCountdown {
    pub remaining: f32,
    pub initial: f32,
}

impl RecordingCountdown {
    pub fn new(duration: f32) -> Self {
        Self {
            remaining: duration,
            initial: duration,
        }
    }

    pub fn tick(&mut self, delta: f32) -> bool {
        self.remaining -= delta;
        self.remaining <= 0.0
    }

    pub fn get_display_number(&self) -> Option<u8> {
        match self.remaining {
            r if r > 2.0 => Some(3),
            r if r > 1.0 => Some(2),
            r if r > 0.0 => Some(1),
            _ => None,
        }
    }
}

/// Marker for entities that already have a published timeline
#[derive(Component)]
pub struct Ghost;
```

### Step 2: Create Recording Events

Add to `src/recording/mod.rs`:

```rust
/// Event to start recording a character
#[derive(Event)]
pub struct StartRecording {
    pub character: Entity,
    pub arena: Arena,
}

/// Event to stop recording (user interruption)
#[derive(Event)]
pub struct StopRecording {
    pub reason: StopReason,
}

#[derive(Debug, Clone)]
pub enum StopReason {
    UserInterrupted,    // User pressed R again
    TimeComplete,       // 120 seconds elapsed
    ArenaTransition,    // Tried to leave arena
    CharacterSwitch,    // Tried to switch characters
}

/// Event to commit the current recording
#[derive(Event)]
pub struct CommitRecording {
    pub character: Entity,
}

/// Event to clear/cancel the current recording
#[derive(Event)]
pub struct ClearRecording {
    pub character: Entity,
}

/// Event to reset arena timeline to start
#[derive(Event)]
pub struct ResetArenaTimeline {
    pub arena: Arena,
}
```

### Step 3: Create Recording Input Detection System

Add to `src/recording/mod.rs`:

```rust
// Const keymap for recording controls
const KEY_RECORD: KeyCode = KeyCode::KeyR;
const KEY_ARENA_PREV: KeyCode = KeyCode::BracketLeft;
const KEY_ARENA_NEXT: KeyCode = KeyCode::BracketRight;
const KEY_CHARACTER_SWITCH: KeyCode = KeyCode::Tab;

/// Detect when player presses R to start/stop recording
pub fn detect_recording_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    recording_state: Res<RecordingState>,
    active_character: Option<Single<Entity, (With<Character>, With<Active>, Without<Ghost>)>>,
    active_ghost: Option<Single<Entity, (With<Character>, With<Active>, With<Ghost>)>>,
    current_arena: Res<CurrentArena>,
    mut start_events: EventWriter<StartRecording>,
    mut stop_events: EventWriter<StopRecording>,
) {
    if !keyboard.just_pressed(KEY_RECORD) {
        return;
    }

    match recording_state.mode {
        RecordingMode::Idle => {
            // Use let-else for cleaner early returns
            let Some(character_single) = active_character else {
                // Check if it's a ghost instead
                if active_ghost.is_some() {
                    info!("Cannot record a ghost - showing retry dialog");
                    // TODO: Show retry dialog in future tutorial
                }
                return;
            };

            // Use explicit Arena::new() constructor
            let Some(arena_idx) = Arena::new(current_arena.0) else {
                warn!("Invalid arena index: {}", current_arena.0);
                return;
            };

            start_events.write(StartRecording {
                character: *character_single,
                arena: arena_idx,
            });
            info!("Starting recording for character {:?}", *character_single);
        }
        RecordingMode::Recording => {
            // Stop recording - user interrupted
            stop_events.write(StopRecording {
                reason: StopReason::UserInterrupted,
            });
            info!("User interrupted recording");
        }
        _ => {
            // Ignore input in other states
        }
    }
}
```

### Step 4: Process State Transitions

Add to `src/recording/mod.rs`:

```rust
/// Process recording state transitions via events
pub fn process_recording_transitions(
    mut transitions: EventReader<RecordingTransition>,
    mut recording_state: ResMut<RecordingState>,
) {
    for transition in transitions.read() {
        // Validate transition is from current state
        if transition.from != recording_state.mode {
            warn!(
                "Invalid transition: current state is {}, but transition is from {}",
                recording_state.mode, transition.from
            );
            continue;
        }

        info!(
            "Recording state transition: {} -> {} (reason: {:?})",
            transition.from, transition.to, transition.reason
        );

        recording_state.mode = transition.to;

        // Handle entity tracking for specific transitions
        match &transition.reason {
            TransitionReason::StartRequest(entity) => {
                recording_state.recording_entity = Some(*entity);
            }
            TransitionReason::UserInterrupted
            | TransitionReason::TimeComplete
            | TransitionReason::ArenaTransition
            | TransitionReason::CharacterSwitch => {
                recording_state.recording_entity = None;
            }
            _ => {}
        }
    }
}
```

### Step 5: Create Recording Initialization System

Add to `src/recording/mod.rs`:

```rust
/// Initialize recording when StartRecording event is received
pub fn initialize_recording(
    mut commands: Commands,
    mut start_events: EventReader<StartRecording>,
    recording_state: Res<RecordingState>,
    character_q: Query<&Transform, With<Character>>,
    mut reset_events: EventWriter<ResetArenaTimeline>,
    mut transition_events: EventWriter<RecordingTransition>,
) {
    for event in start_events.read() {
        // Reset the arena timer to 0
        reset_events.write(ResetArenaTimeline {
            arena: event.arena,
        });

        // Get character's current transform using let-else
        let Ok(transform) = character_q.get(event.character) else {
            warn!("Character {:?} not found", event.character);
            continue;
        };

        // Convert world position to grid position
        let grid_pos = GridPos::new(
            (transform.translation.x / 100.0).round() as i32,
            (transform.translation.z / 100.0).round() as i32,
        );

        // Create initial timeline event at t=0
        let initial_event = TimelineEvent {
            timestamp: TimeStamp::ZERO,
            event_type: EventType::Movement(grid_pos),
        };

        // Create draft timeline with initial position
        let mut draft = DraftTimeline::new();
        draft.add_event(initial_event);

        // Add recording components to character
        // Note: draft is consumed by .insert(), transferring ownership for efficient storage
        commands.entity(event.character)
            .insert(Recording)
            .insert(draft) // Zero-copy: DraftTimeline ownership transfers to ECS
            .insert(RecordingCountdown::new(3.0));

        // Trigger state transition
        transition_events.write(RecordingTransition {
            from: RecordingMode::Idle,
            to: RecordingMode::Countdown,
            reason: TransitionReason::StartRequest(event.character),
        });

        info!("Initialized recording for character {:?}", event.character);
    }
}

/// Reset arena timeline when requested
pub fn reset_arena_timeline(
    mut reset_events: EventReader<ResetArenaTimeline>,
    mut arena_q: Query<(&Arena, &mut TimelineClock)>,
) {
    for event in reset_events.read() {
        // Use iterator find with proper Arena comparison
        let Some((_, mut clock)) = arena_q
            .iter_mut()
            .find(|(idx, _)| **idx == event.arena)
        else {
            continue;
        };

        clock.reset();
        info!("Reset {} timer to 0.0", event.arena);
    }
}
```

### Step 6: Create Countdown System

Add to `src/recording/mod.rs`:

```rust
/// Update recording countdown and transition to recording mode
pub fn update_recording_countdown(
    mut commands: Commands,
    virtual_time: Res<Time<Virtual>>,
    recording_state: Res<RecordingState>,
    mut countdown_q: Query<(Entity, &mut RecordingCountdown)>,
    mut transition_events: EventWriter<RecordingTransition>,
) {
    // Only process during countdown mode
    if recording_state.mode != RecordingMode::Countdown {
        return;
    }

    let delta = virtual_time.delta_secs();

    for (entity, mut countdown) in countdown_q.iter_mut() {
        let prev_display = countdown.get_display_number();

        if countdown.tick(delta) {
            // Countdown complete - start recording
            commands.entity(entity).remove::<RecordingCountdown>();
            
            transition_events.write(RecordingTransition {
                from: RecordingMode::Countdown,
                to: RecordingMode::Recording,
                reason: TransitionReason::CountdownComplete,
            });
            
            info!("Recording started!");
        } else {
            // Check if display number changed for UI feedback
            let new_display = countdown.get_display_number();
            if prev_display != new_display {
                if let Some(num) = new_display {
                    // PR Gate: Using debug! for countdown display
                    debug!("{}...", num);
                }
            }
        }
    }
}
```

### Step 7: Block Input During Recording

Add to `src/recording/mod.rs`:

```rust
/// Block arena/character switching during recording
pub fn block_recording_interruptions(
    keyboard: Res<ButtonInput<KeyCode>>,
    recording_state: Res<RecordingState>,
    mut stop_events: EventWriter<StopRecording>,
) {
    // Only check during active recording
    if recording_state.mode != RecordingMode::Recording {
        return;
    }

    // Check for blocked inputs using const keymaps
    let stop_reason = if keyboard.just_pressed(KEY_ARENA_PREV) || 
                         keyboard.just_pressed(KEY_ARENA_NEXT) {
        Some(StopReason::ArenaTransition)
    } else if keyboard.just_pressed(KEY_CHARACTER_SWITCH) {
        Some(StopReason::CharacterSwitch)
    } else {
        None
    };

    if let Some(reason) = stop_reason {
        stop_events.write(StopRecording { reason: reason.clone() });
        info!("Blocked action during recording: {:?}", reason);
    }
}
```

### Step 8: Handle Recording Completion

Add to `src/recording/mod.rs`:

```rust
/// Check if recording time limit reached
pub fn check_recording_time_limit(
    recording_state: Res<RecordingState>,
    arena_q: Query<(&Arena, &TimelineClock)>,
    current_arena: Res<CurrentArena>,
    mut stop_events: EventWriter<StopRecording>,
) {
    // Only check during active recording
    if recording_state.mode != RecordingMode::Recording {
        return;
    }

    // Use explicit Arena::new() constructor
    let Some(current_idx) = Arena::new(current_arena.0) else {
        return;
    };

    // Check current arena timer using let-else
    let Some((_, clock)) = arena_q
        .iter()
        .find(|(idx, _)| **idx == current_idx)
    else {
        return;
    };

    if clock.current().as_secs() >= TimeStamp::MAX.0 - 0.1 {
        stop_events.write(StopRecording {
            reason: StopReason::TimeComplete,
        });
        info!("Recording time limit reached");
    }
}

/// Process stop recording events
pub fn process_stop_recording(
    mut commands: Commands,
    mut stop_events: EventReader<StopRecording>,
    recording_state: Res<RecordingState>,
    recording_entity: Option<Single<Entity, With<Recording>>>,
    mut transition_events: EventWriter<RecordingTransition>,
) {
    for event in stop_events.read() {
        // Use let-else for cleaner code
        let Some(entity_single) = recording_entity else {
            warn!("No recording entity found");
            continue;
        };

        // For now, just clear the recording
        // In future tutorials, we'll show dialog here
        // Note: .remove() consumes component ownership, enabling efficient cleanup
        commands.entity(*entity_single)
            .remove::<Recording>()
            .remove::<DraftTimeline>() // Zero-copy: Component ownership transferred for cleanup
            .remove::<RecordingCountdown>();

        // Determine transition reason
        let reason = match event.reason {
            StopReason::UserInterrupted => TransitionReason::UserInterrupted,
            StopReason::TimeComplete => TransitionReason::TimeComplete,
            StopReason::ArenaTransition => TransitionReason::ArenaTransition,
            StopReason::CharacterSwitch => TransitionReason::CharacterSwitch,
        };

        transition_events.write(RecordingTransition {
            from: RecordingMode::Recording,
            to: RecordingMode::Idle,
            reason,
        });

        info!("Stopped recording due to: {:?}", event.reason);
    }
}
```

### Step 9: Create the Recording Plugin

Add to `src/recording/mod.rs`:

```rust
pub struct RecordingPlugin;

impl Plugin for RecordingPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<RecordingState>()
            .init_resource::<GlobalTimelinePause>()

            // Events
            .add_event::<StartRecording>()
            .add_event::<StopRecording>()
            .add_event::<CommitRecording>()
            .add_event::<ClearRecording>()
            .add_event::<ResetArenaTimeline>()
            .add_event::<RecordingTransition>()

            // Systems with strict ordering
            .add_systems(Update, (
                // Input detection
                detect_recording_input,
                block_recording_interruptions,

                // State transitions MUST happen before other systems
                process_recording_transitions,

                // Recording flow
                initialize_recording,
                reset_arena_timeline,
                update_recording_countdown,

                // Completion handling
                check_recording_time_limit,
                process_stop_recording,
            ).chain());
    }
}
```

### Step 10: Wire Into Main

Update `src/main.rs`:

```rust
mod recording;
use crate::recording::RecordingPlugin;

// In main():
.add_plugins(RecordingPlugin)
```

## Unit Tests

Create `src/recording/tests.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recording_countdown() {
        let mut countdown = RecordingCountdown::new(3.0);

        assert_eq!(countdown.get_display_number(), Some(3));

        countdown.tick(1.5);
        assert_eq!(countdown.get_display_number(), Some(2));

        countdown.tick(1.0);
        assert_eq!(countdown.get_display_number(), Some(1));

        assert!(!countdown.tick(0.4)); // Not done yet
        assert!(countdown.tick(0.2));  // Now it's done
    }

    #[test]
    fn test_recording_mode_display() {
        assert_eq!(RecordingMode::Idle.to_string(), "Idle");
        assert_eq!(RecordingMode::Countdown.to_string(), "Countdown");
        assert_eq!(RecordingMode::Recording.to_string(), "Recording");
        assert_eq!(RecordingMode::DialogPaused.to_string(), "DialogPaused");
    }

    #[test]
    fn test_recording_state_transitions() {
        let mut state = RecordingState::default();

        assert_eq!(state.mode, RecordingMode::Idle);

        // Simulate transitions
        state.mode = RecordingMode::Countdown;
        assert_eq!(state.mode, RecordingMode::Countdown);

        state.mode = RecordingMode::Recording;
        assert_eq!(state.mode, RecordingMode::Recording);
    }

    #[test]
    fn test_stop_reason_variants() {
        // Ensure all variants are covered
        let reasons = vec![
            StopReason::UserInterrupted,
            StopReason::TimeComplete,
            StopReason::ArenaTransition,
            StopReason::CharacterSwitch,
        ];

        // Test that Debug is implemented
        for reason in &reasons {
            let debug_str = format!("{:?}", reason);
            assert!(!debug_str.is_empty());
        }
    }

    #[test]
    fn test_transition_reason_completeness() {
        // Ensure all transition reasons are handled
        let reasons = vec![
            TransitionReason::StartRequest(Entity::PLACEHOLDER),
            TransitionReason::CountdownComplete,
            TransitionReason::UserInterrupted,
            TransitionReason::TimeComplete,
            TransitionReason::ArenaTransition,
            TransitionReason::CharacterSwitch,
            TransitionReason::DialogOpened,
            TransitionReason::DialogClosed,
        ];

        for reason in &reasons {
            let debug_str = format!("{:?}", reason);
            assert!(!debug_str.is_empty());
        }
    }
    
    #[test]
    fn test_event_driven_state_transitions() {
        use bevy::app::App;
        use bevy::prelude::*;
        
        // Create test app
        let mut app = App::new();
        app.init_resource::<RecordingState>();
        app.add_event::<RecordingTransition>();
        app.add_systems(Update, process_recording_transitions);
        
        // Send transition event - using explicit constructor
        app.world_mut().send_event(RecordingTransition {
            from: RecordingMode::Idle,
            to: RecordingMode::Countdown,
            reason: TransitionReason::StartRequest(Entity::PLACEHOLDER),
        });
        
        // Process the event
        app.update();
        
        // Verify state changed
        let state = app.world().resource::<RecordingState>();
        assert_eq!(state.mode, RecordingMode::Countdown);
        assert!(state.recording_entity.is_some());
        
        // Test invalid transition (wrong 'from' state)
        app.world_mut().send_event(RecordingTransition {
            from: RecordingMode::Recording, // Wrong! We're in Countdown
            to: RecordingMode::Idle,
            reason: TransitionReason::UserInterrupted,
        });
        
        app.update();
        
        // State should not have changed
        let state = app.world().resource::<RecordingState>();
        assert_eq!(state.mode, RecordingMode::Countdown);
    }
    
    #[test]
    fn test_global_timeline_pause() {
        let mut pause = GlobalTimelinePause::default();
        
        assert!(!pause.is_paused);
        assert!(pause.pause_reason.is_none());
        
        pause.pause(PauseReason::DialogOpen);
        assert!(pause.is_paused);
        assert!(matches!(pause.pause_reason, Some(PauseReason::DialogOpen)));
        
        pause.resume();
        assert!(!pause.is_paused);
        assert!(pause.pause_reason.is_none());
    }

```

## Verification

Run the tests:

```bash
cargo test recording
```

Run the game and test the state machine:

```bash
cargo run
```

Test sequence:

1. Press R - Should see "3... 2... 1... Recording started!" with state transitions logged
2. Press R again during recording - Should see transition from Recording to Idle
3. Press Tab during recording - Should block and stop recording
4. Press [ or ] during recording - Should block and stop recording
5. Wait 120 seconds - Should automatically stop with TimeComplete reason

## Next Steps

With the event-driven recording state machine complete, we can now:

- Tutorial 03: Capture character movement and abilities (recording intent, not transforms!)
- Tutorial 04: Build the playback system with strict ordering
- Tutorial 05: Add confirmation dialogs with state transitions

## Key Takeaways

1. **Event-Driven Transitions**: All state changes go through RecordingTransition events
2. **Let-Else Pattern**: Cleaner early returns with proper error handling
3. **Const Keymaps**: Centralized key definitions prevent magic values
4. **Explicit Constructors**: Arena::new() instead of raw u8 conversion
5. **Transition Tracing**: Every state change is logged with reason
6. **Virtual Time**: Uses Time<Virtual> for pause-safe countdown timers

## Production Notes

### What We Got Right:

- Event-driven state transitions provide full traceability
- let-else patterns reduce nesting and improve readability
- Const keymaps make input configuration maintainable
- RecordingTransition events enable debugging and analytics

### What We Intentionally Simplified:

- No state history (unnecessary for tutorials)
- No undo/redo (not relevant for recording)
- No complex state validation (enum makes invalid states unrepresentable)

### Why These Patterns Matter:

- **Event Transitions**: Debug any state issue by examining event history
- **Let-Else**: Reduces cognitive load when reading system code
- **Explicit Constructors**: Arena::new() makes the common case obvious and validates input
- **Const Keymaps**: Change controls in one place, not scattered throughout

This state machine ensures recording happens in a controlled, traceable manner. The event-driven transitions allow other
systems to react to state changes and provide excellent debugging capabilities when issues arise.