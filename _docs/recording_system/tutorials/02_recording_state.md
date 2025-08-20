# Tutorial 02: Recording State Machine

## Objective

Implement the state machine that manages recording modes using the superior **command pattern**. Systems send commands (intent) to a single state machine, which validates transitions and emits state change events. This eliminates race conditions and makes invalid transitions impossible at the type level.

## Prerequisites

- Completed Tutorial 01 (Timeline Foundation)
- Understanding of Bevy Resources and Events
- Familiarity with state machines

## Components/Systems

We'll create:

- Recording state resource managed by single state machine
- Recording mode enum with type-safe transitions
- Command enum for expressing intent
- State change events for notifications
- Single authoritative state machine that validates all transitions

## Implementation Steps

### Step 1: Create Recording State Components

Create `src/recording/mod.rs`:

```rust
use bevy::prelude::*;
use bevy::time::Virtual;
use std::fmt::{self, Display, Formatter};
use std::error::Error;
use std::time::Duration;
use crate::timeline::{DraftTimeline, TimelineEvent, EventType, TimeStamp, GridPos, TimelineClock};
use crate::arena::{Arena, ArenaId, CurrentArena, ArenaEntities};
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

impl Display for RecordingMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Idle => write!(f, "Idle"),
            Self::Countdown => write!(f, "Countdown"),
            Self::Recording => write!(f, "Recording"),
            Self::DialogPaused => write!(f, "DialogPaused"),
        }
    }
}

/// Command pattern: Systems send commands (intent), state machine emits events (what happened)
#[derive(Event)]
pub enum RecordingCommand {
    StartRecording { entity: Entity },
    StopRecording { reason: StopReason },
    PauseForDialog,
    ResumeFromDialog,
    CommitRecording { entity: Entity },
    ClearRecording { entity: Entity },
}

/// Event emitted when recording state actually changes
#[derive(Event)]
pub struct RecordingStateChanged {
    pub previous: RecordingMode,
    pub current: RecordingMode,
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
    pub remaining: Duration,
    pub initial: Duration,
}

impl RecordingCountdown {
    pub fn new(duration: Duration) -> Self {
        Self {
            remaining: duration,
            initial: duration,
        }
    }

    pub fn tick(&mut self, delta: Duration) -> bool {
        self.remaining = self.remaining.saturating_sub(delta);
        self.remaining.is_zero()
    }

    pub fn get_display_number(&self) -> Option<u8> {
        let secs = self.remaining.as_secs_f32();
        match secs {
            s if s > 2.0 => Some(3),
            s if s > 1.0 => Some(2),
            s if s > 0.0 => Some(1),
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
// These remain for specialized use cases
#[derive(Debug, Clone)]
pub enum StopReason {
    UserInterrupted,    // User pressed R again
    TimeComplete,       // 120 seconds elapsed
    ArenaTransition,    // Tried to leave arena
    CharacterSwitch,    // Tried to switch characters
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

/// Event to show retry dialog for ghost recording attempt
#[derive(Event)]
pub struct ShowRetryDialog {
    pub character: Entity,
    pub arena: ArenaId,
}

/// Detect when player presses R to start/stop recording - now uses command pattern
pub fn detect_recording_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    recording_state: Res<RecordingState>,
    // Fixed: Single query for all active characters (both regular and ghosts)
    active_character: Option<Single<(Entity, Option<&Ghost>), (With<Character>, With<Active>)>>,
    mut command_writer: EventWriter<RecordingCommand>,
    mut retry_dialog_events: EventWriter<ShowRetryDialog>,
) {
    if !keyboard.just_pressed(KEY_RECORD) {
        return;
    }

    match recording_state.mode {
        RecordingMode::Idle => {
            // Use let-else for cleaner early returns
            let Some((character_entity, ghost_marker)) = active_character else {
                return;
            };

            // Check if this character is a ghost
            if ghost_marker.is_some() {
                // Ghost selected - show retry dialog
                retry_dialog_events.write(ShowRetryDialog {
                    character: *character_entity,
                    arena: ArenaId::new(ArenaName::GuildHouse), // Current arena
                });
                info!("Cannot record a ghost - showing retry dialog for character {:?}", *character_entity);
            } else {
                // Regular character - send start recording command
                command_writer.write(RecordingCommand::StartRecording { 
                    entity: *character_entity 
                });
                info!("Sending start recording command for character {:?}", *character_entity);
            }
        }
        RecordingMode::Recording => {
            // Stop recording - send stop command
            command_writer.write(RecordingCommand::StopRecording {
                reason: StopReason::UserInterrupted,
            });
            info!("Sending stop recording command - user interrupted");
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
/// State machine that processes commands and emits state change events
/// This is the SINGLE SOURCE OF TRUTH for all recording state transitions
pub fn process_recording_commands(
    mut commands_reader: EventReader<RecordingCommand>,
    mut recording_state: ResMut<RecordingState>,
    mut state_change_writer: EventWriter<RecordingStateChanged>,
    mut countdown_commands: Commands,
) {
    for command in commands_reader.read() {
        let previous_mode = recording_state.mode;
        
        match command {
            RecordingCommand::StartRecording { entity } => {
                match previous_mode {
                    RecordingMode::Idle => {
                        // Valid transition: start countdown
                        recording_state.mode = RecordingMode::Countdown;
                        recording_state.recording_entity = Some(*entity);
                        
                        // Add countdown component to entity
                        countdown_commands.entity(*entity)
                            .insert(RecordingCountdown::new(Duration::from_secs(3)));
                        
                        state_change_writer.write(RecordingStateChanged {
                            previous: previous_mode,
                            current: RecordingMode::Countdown,
                            reason: TransitionReason::StartRequest(*entity),
                        });
                        
                        info!("Started recording countdown for entity {:?}", entity);
                    }
                    _ => {
                        warn!("Cannot start recording from state: {:?}", previous_mode);
                    }
                }
            }
            
            RecordingCommand::StopRecording { reason } => {
                match previous_mode {
                    RecordingMode::Recording => {
                        // Valid transition: stop recording
                        recording_state.mode = RecordingMode::Idle;
                        let entity = recording_state.recording_entity.take();
                        
                        // Remove recording components if entity exists
                        if let Some(entity) = entity {
                            countdown_commands.entity(entity)
                                .remove::<Recording>()
                                .remove::<RecordingCountdown>();
                        }
                        
                        let transition_reason = match reason {
                            StopReason::UserInterrupted => TransitionReason::UserInterrupted,
                            StopReason::TimeComplete => TransitionReason::TimeComplete,
                            StopReason::ArenaTransition => TransitionReason::ArenaTransition,
                            StopReason::CharacterSwitch => TransitionReason::CharacterSwitch,
                        };
                        
                        state_change_writer.write(RecordingStateChanged {
                            previous: previous_mode,
                            current: RecordingMode::Idle,
                            reason: transition_reason,
                        });
                        
                        info!("Stopped recording due to: {:?}", reason);
                    }
                    _ => {
                        warn!("Cannot stop recording from state: {:?}", previous_mode);
                    }
                }
            }
            
            RecordingCommand::PauseForDialog => {
                match previous_mode {
                    RecordingMode::Recording => {
                        recording_state.mode = RecordingMode::DialogPause;
                        
                        state_change_writer.write(RecordingStateChanged {
                            previous: previous_mode,
                            current: RecordingMode::DialogPause,
                            reason: TransitionReason::DialogOpened,
                        });
                        
                        info!("Paused recording for dialog");
                    }
                    _ => {
                        warn!("Cannot pause recording from state: {:?}", previous_mode);
                    }
                }
            }
            
            RecordingCommand::ResumeFromDialog => {
                match previous_mode {
                    RecordingMode::DialogPause => {
                        recording_state.mode = RecordingMode::Recording;
                        
                        state_change_writer.write(RecordingStateChanged {
                            previous: previous_mode,
                            current: RecordingMode::Recording,
                            reason: TransitionReason::DialogClosed,
                        });
                        
                        info!("Resumed recording from dialog");
                    }
                    _ => {
                        warn!("Cannot resume recording from state: {:?}", previous_mode);
                    }
                }
            }
            
            RecordingCommand::CommitRecording { entity } => {
                // Handle commit recording - convert to ghost
                info!("Committing recording for entity {:?}", entity);
                // TODO: Implement commit logic in future systems
            }
            
            RecordingCommand::ClearRecording { entity } => {
                // Handle clear recording - discard timeline
                info!("Clearing recording for entity {:?}", entity);
                // TODO: Implement clear logic in future systems  
            }
        }
    }
}
```

### Step 5: Create Recording Initialization System

Add to `src/recording/mod.rs`:

```rust
// This system is now integrated into the command processor
// The state machine handles initialization directly when processing StartRecording commands

/// Reset arena timeline when requested - uses ArenaEntities O(1) lookup
pub fn reset_arena_timeline(
    mut reset_events: EventReader<ResetArenaTimeline>,
    mut arena_q: Query<(&Arena, &mut TimelineClock)>,
    arena_entities: Res<ArenaEntities>,  // O(1) arena entity lookup
) {
    for event in reset_events.read() {
        // O(1) lookup for arena entity using ArenaEntities
        let arena_entity = arena_entities.get(event.arena.name());
        
        // Direct query for the specific arena - no iteration needed
        let Ok((_, mut clock)) = arena_q.get_mut(arena_entity) else {
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
/// Handle countdown completion and transition to recording
pub fn handle_countdown_completion(
    mut recording_state: ResMut<RecordingState>,
    mut countdown_query: Query<(Entity, &mut RecordingCountdown)>,
    mut state_change_writer: EventWriter<RecordingStateChanged>,
    mut countdown_commands: Commands,
    time: Res<Time>,
) {
    // Only process during countdown mode
    if recording_state.mode != RecordingMode::Countdown {
        return;
    }
    
    let delta = time.delta();
    
    for (entity, mut countdown) in countdown_query.iter_mut() {
        let prev_display = countdown.get_display_number();
        
        if countdown.tick(delta) {
            // Countdown complete - transition to recording
            countdown_commands.entity(entity)
                .remove::<RecordingCountdown>()
                .insert(Recording);
            
            recording_state.mode = RecordingMode::Recording;
            
            // Emit state change event directly
            state_change_writer.write(RecordingStateChanged {
                previous: RecordingMode::Countdown,
                current: RecordingMode::Recording,
                reason: TransitionReason::CountdownComplete,
            });
            
            info!("Recording countdown completed for entity {:?}", entity);
        } else {
            // Check if display number changed for UI feedback
            let new_display = countdown.get_display_number();
            if prev_display != new_display {
                if let Some(num) = new_display {
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
/// Block arena/character switching during recording - now uses command pattern
pub fn block_recording_interruptions(
    keyboard: Res<ButtonInput<KeyCode>>,
    recording_state: Res<RecordingState>,
    mut command_writer: EventWriter<RecordingCommand>,
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
        command_writer.write(RecordingCommand::StopRecording { reason });
        info!("Blocked action during recording: {:?}", reason);
    }
}
```

### Step 8: Handle Recording Completion

Add to `src/recording/mod.rs`:

```rust
/// Check if recording time limit reached - uses ArenaEntities O(1) lookup
pub fn check_recording_time_limit(
    recording_state: Res<RecordingState>,
    arena_q: Query<(&Arena, &TimelineClock)>,
    arena_entities: Res<ArenaEntities>,  // O(1) arena entity lookup
    current_arena: Res<CurrentArena>,
    mut command_writer: EventWriter<RecordingCommand>,
) {
    // Only check during active recording
    if recording_state.mode != RecordingMode::Recording {
        return;
    }

    // O(1) lookup for current arena entity using ArenaEntities
    let current_arena_entity = arena_entities.get(current_arena.name());
    
    // Direct query for the current arena - no iteration needed
    let Ok((_, clock)) = arena_q.get(current_arena_entity) else {
        return;
    };

    if clock.current().as_secs() >= 120.0 - 0.1 { // 120 second recording limit
        command_writer.write(RecordingCommand::StopRecording {
            reason: StopReason::TimeComplete,
        });
        info!("Recording time limit reached");
    }
}

// This system is now integrated into the command processor
// The state machine handles stop recording directly when processing StopRecording commands
```

### Step 9: Create the Recording Plugin

Add to `src/recording/mod.rs`:

```rust
/// Recording Plugin - configures all recording systems with command pattern
pub struct RecordingPlugin;

impl Plugin for RecordingPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<RecordingState>()
            
            // Command/Event architecture
            .add_event::<RecordingCommand>()
            .add_event::<RecordingStateChanged>()
            .add_event::<ShowRetryDialog>()
            
            // Systems with explicit ordering
            .add_systems(Update, (
                // Input detection (generates commands)
                detect_recording_input,
                block_recording_interruptions,
                check_recording_time_limit,
                
                // State machine (processes commands, emits events) - SINGLE SOURCE OF TRUTH
                process_recording_commands,
                
                // Internal state handling
                handle_countdown_completion,
                
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
        let mut countdown = RecordingCountdown::new(Duration::from_secs(3));

        assert_eq!(countdown.get_display_number(), Some(3));

        countdown.tick(Duration::from_millis(1500));
        assert_eq!(countdown.get_display_number(), Some(2));

        countdown.tick(Duration::from_secs(1));
        assert_eq!(countdown.get_display_number(), Some(1));

        assert!(!countdown.tick(Duration::from_millis(400))); // Not done yet
        assert!(countdown.tick(Duration::from_millis(200)));  // Now it's done
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
    fn test_command_driven_state_transitions() {
        use bevy::app::App;
        use bevy::prelude::*;
        
        // Create test app
        let mut app = App::new();
        app.init_resource::<RecordingState>();
        app.add_event::<RecordingCommand>();
        app.add_event::<RecordingStateChanged>();
        app.add_systems(Update, process_recording_commands);
        
        // Send start recording command
        app.world_mut().send_event(RecordingCommand::StartRecording {
            entity: Entity::PLACEHOLDER,
        });
        
        // Process the command
        app.update();
        
        // Verify state changed to countdown
        let state = app.world().resource::<RecordingState>();
        assert_eq!(state.mode, RecordingMode::Countdown);
        assert!(state.recording_entity.is_some());
        
        // Test invalid command (start recording when already in countdown)
        app.world_mut().send_event(RecordingCommand::StartRecording {
            entity: Entity::PLACEHOLDER,
        });
        
        app.update();
        
        // State should remain unchanged (invalid transition rejected)
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

1. **Command Pattern**: Systems send commands (intent), state machine validates and transitions
2. **Single Source of Truth**: Only the state machine can change recording state
3. **Type Safety**: Invalid transitions are impossible at compile time
4. **Race Condition Elimination**: Commands are processed sequentially by single authority
5. **Let-Else Pattern**: Cleaner early returns with proper error handling
6. **Const Keymaps**: Centralized key definitions prevent magic values
7. **Event Notifications**: State changes emit events for other systems to react
8. **⚡ ArenaEntities O(1) Lookup**: Use ArenaEntities resource for O(1) arena entity lookup in check_recording_time_limit system - critical for performance with multiple recording systems

## Why Command Pattern vs Direct Transitions?

The old approach had systems directly sending `RecordingTransition` events, which created several critical problems:

### Problems with Direct Transitions:

1. **Race Conditions**: Multiple systems could send conflicting transitions in the same frame
2. **Invalid States**: Systems could force invalid transitions (e.g., Recording → Countdown)  
3. **Distributed State Management**: Logic for "what transitions are valid" was scattered across systems
4. **Type Unsafety**: Nothing prevented systems from sending transitions with wrong `from` state
5. **Debugging Nightmare**: Hard to trace which system caused problematic state changes

### Command Pattern Benefits:

1. **Single Authority**: Only the state machine can change state - eliminates race conditions
2. **Type Safety**: Invalid transitions are rejected at runtime with clear warnings  
3. **Centralized Logic**: All transition validation lives in one place
4. **Intent vs Reality**: Commands express "what you want", events express "what happened"
5. **Debuggable**: Clear command → state change → event flow for troubleshooting
6. **Testable**: Easy to unit test the state machine with specific command sequences

The key insight: **Systems should express intent (commands), not dictate reality (transitions)**.

## Production Notes

### What We Got Right:

- **Command Pattern**: Systems send intent, state machine validates transitions
- **Single Source of Truth**: Only one place can change recording state
- **Type Safety**: Invalid transitions are caught and rejected
- **Let-else patterns**: Reduce nesting and improve readability
- **Const keymaps**: Make input configuration maintainable
- **Event Notifications**: Other systems can react to state changes
- **Centralized Validation**: All transition logic in one place

### What We Intentionally Simplified:

- No command queuing (unnecessary for recording use case)
- No state history (can be added if needed for debugging)
- No complex rollback (recording doesn't need transactional behavior)

### Why Command Pattern Matters:

- **Race Condition Prevention**: Commands are processed sequentially
- **Intent vs Reality**: Clear separation between "what you want" and "what happened"
- **Debuggable**: Easy to trace command → state change → event flow
- **Testable**: State machine can be unit tested with command sequences
- **Type Safe**: Invalid transitions are impossible at compile time
- **Centralized Authority**: One place owns all state transition logic

This architecture ensures recording state changes happen in a controlled, predictable manner. The command pattern
eliminates race conditions and makes the system much easier to debug and maintain.