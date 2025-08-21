# Tutorial 02: Recording State Machine

## Objective

Implement the unified recording event architecture using the **RecordingUpdate orchestrator pattern**. This mirrors the successful camera system pattern to prevent event explosion and race conditions. A single recording_update() system handles ALL recording coordination, just like arena_update() handles camera coordination.

## Prerequisites

- Completed Tutorial 01 (Timeline Foundation)
- Understanding of Bevy Resources and Events
- Familiarity with state machines

## Components/Systems

We'll create:

- Recording state resource as single source of truth (like CurrentArena)
- Simple RecordingRequest events for expressing intent (like arena navigation inputs)
- RecordingUpdate root event for orchestration (like CameraUpdate)
- Single recording_update() orchestrator system (like arena_update())
- Clean event flow that prevents race conditions and complexity explosion

## Implementation Steps

### Step 1: Create Recording Events and State

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

// === ROOT EVENT - Like CameraUpdate ===
/// Root orchestration event that triggers recording coordination
/// Multiple systems can trigger this, single orchestrator handles all logic
#[derive(Event, Debug, Clone)]
pub struct RecordingUpdate;

// === TRIGGER EVENTS - Simple requests ===
/// Simple request events that express intent (like arena navigation inputs)
/// All operations target the currently Active Character (query With<Active> when processing)
#[derive(Event, Debug)]
pub enum RecordingRequest {
    Start, // Query Active Character when processing
    Stop { reason: StopReason },
    Commit,     // Query Active Character and CurrentArena resource
    Clear,      // Query Active Character
    ShowDialog, // Query Active Character when showing dialog
}

// === STATE RESOURCE - Single source of truth ===
/// Global recording state for the game (like CurrentArena)
/// No entity storage needed - always query Active Character when needed
#[derive(Resource, Debug)]
pub struct RecordingState {
    pub mode: RecordingMode,
    pub pending_request: Option<RecordingRequest>,
    pub countdown_remaining: Option<Duration>,
}

impl Default for RecordingState {
    fn default() -> Self {
        Self {
            mode: RecordingMode::Idle,
            pending_request: None,
            countdown_remaining: None,
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
    /// Dialog shown, all timelines paused (targets Active Character)
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

#[derive(Debug, Clone, Copy)]
pub enum StopReason {
    UserInterrupted, // User pressed R again
    TimeComplete,    // 120 seconds elapsed
    ArenaTransition, // Tried to leave arena
    CharacterSwitch, // Tried to switch characters
}

/// Marker component for characters currently being recorded
#[derive(Component)]
pub struct Recording;

/// Marker for entities that already have a published timeline
#[derive(Component)]
pub struct Ghost;
```

### Step 2: Create Input Detection Systems

These systems detect user input and trigger RecordingUpdate events (like arena navigation systems trigger CameraUpdate).

Add to `src/recording/mod.rs`:

```rust
// Const keymap for recording controls
const KEY_RECORD: KeyCode = KeyCode::KeyR;
const KEY_ARENA_PREV: KeyCode = KeyCode::BracketLeft;
const KEY_ARENA_NEXT: KeyCode = KeyCode::BracketRight;
const KEY_CHARACTER_SWITCH: KeyCode = KeyCode::Tab;

/// Detect when player presses R to start/stop recording
/// Triggers RecordingUpdate event (like arena navigation triggers CameraUpdate)
pub fn detect_recording_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut recording_state: ResMut<RecordingState>,
    active_character: Option<Single<(Entity, Option<&Ghost>), (With<Character>, With<Active>)>>,
    mut recording_update_events: EventWriter<RecordingUpdate>,
) {
    if !keyboard.just_pressed(KEY_RECORD) {
        return;
    }

    match recording_state.mode {
        RecordingMode::Idle => {
            let Some(single) = active_character else {
                return;
            };
            let (character_entity, ghost_marker) = single.into_inner();

            if ghost_marker.is_some() {
                // Ghost selected - show retry dialog
                recording_state.pending_request = Some(RecordingRequest::ShowDialog);
                recording_update_events.write(RecordingUpdate);
                info!(
                    "Cannot record a ghost - showing retry dialog for active character {:?}",
                    character_entity
                );
            } else {
                // Store request and trigger update
                recording_state.pending_request = Some(RecordingRequest::Start);
                recording_update_events.write(RecordingUpdate);
                info!(
                    "Recording start requested for active character {:?}",
                    character_entity
                );
            }
        }
        RecordingMode::Recording => {
            // Store stop request and trigger update
            recording_state.pending_request = Some(RecordingRequest::Stop {
                reason: StopReason::UserInterrupted,
            });
            recording_update_events.write(RecordingUpdate);
            info!("Recording stop requested - user interrupted");
        }
        _ => {
            // Ignore input in other states
        }
    }
}

/// Block arena/character switching during recording
/// Triggers RecordingUpdate to stop recording if blocked actions attempted
pub fn block_recording_interruptions(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut recording_state: ResMut<RecordingState>,
    mut recording_update_events: EventWriter<RecordingUpdate>,
) {
    // Only check during active recording
    if recording_state.mode != RecordingMode::Recording {
        return;
    }

    let stop_reason =
        if keyboard.just_pressed(KEY_ARENA_PREV) || keyboard.just_pressed(KEY_ARENA_NEXT) {
            Some(StopReason::ArenaTransition)
        } else if keyboard.just_pressed(KEY_CHARACTER_SWITCH) {
            Some(StopReason::CharacterSwitch)
        } else {
            None
        };

    if let Some(reason) = stop_reason {
        recording_state.pending_request = Some(RecordingRequest::Stop { reason });
        recording_update_events.write(RecordingUpdate);
        info!("Blocked action during recording: {:?}", reason);
    }
}

/// Check if recording time limit reached
/// Triggers RecordingUpdate to stop recording when time limit reached
pub fn check_recording_time_limit(
    arena_q: Query<(&Arena, &TimelineClock)>,
    arena_entities: Res<ArenaEntities>,
    current_arena: Res<CurrentArena>,
    mut recording_state: ResMut<RecordingState>,
    mut recording_update_events: EventWriter<RecordingUpdate>,
) {
    // Only check during active recording
    if recording_state.mode != RecordingMode::Recording {
        return;
    }

    // O(1) lookup for current arena entity
    let current_arena_entity = arena_entities.get(current_arena.name());
    let Ok((_, clock)) = arena_q.get(current_arena_entity) else {
        return;
    };

    if clock.current().as_secs() >= 120.0 - 0.1 {
        // 120 second recording limit
        recording_state.pending_request = Some(RecordingRequest::Stop {
            reason: StopReason::TimeComplete,
        });
        recording_update_events.write(RecordingUpdate);
        info!("Recording time limit reached");
    }
}
```

### Step 3: The Recording Orchestrator (Like arena_update)

This is the heart of the new pattern - the single system that handles ALL recording coordination.

Add to `src/recording/mod.rs`:

```rust
/// SINGLE ORCHESTRATOR - Like arena_update(), handles ALL recording logic
/// Processes RecordingUpdate events and coordinates all recording operations
pub fn recording_update(
    mut recording_update_events: EventReader<RecordingUpdate>,
    mut recording_state: ResMut<RecordingState>,
    mut commands: Commands,
    time: Res<Time>,
    // All the queries needed for recording coordination
    _arena_q: Query<(&Arena, &TimelineClock)>,
    _arena_entities: Res<ArenaEntities>,
    current_arena: Res<CurrentArena>,
    // Query for the single Active Character when needed
    active_character_q: Query<
        (Entity, Option<&Recording>, Option<&Ghost>),
        (With<Character>, With<Active>),
    >,
) {
    // Only run when RecordingUpdate event is triggered
    if recording_update_events.is_empty() {
        return;
    }
    recording_update_events.clear();

    let previous_mode = recording_state.mode;

    // Handle countdown progression
    if recording_state.mode == RecordingMode::Countdown {
        if let Some(ref mut remaining) = recording_state.countdown_remaining {
            *remaining = remaining.saturating_sub(time.delta());

            if remaining.is_zero() {
                // Countdown complete - transition to recording
                recording_state.mode = RecordingMode::Recording;
                recording_state.countdown_remaining = None;

                // Add Recording component to Active Character
                if let Ok((entity, _, _)) = active_character_q.single() {
                    commands.entity(entity).insert(Recording);
                    info!("Recording started for active character {:?}", entity);
                } else {
                    warn!("No active character found to start recording");
                    recording_state.mode = RecordingMode::Idle;
                }
            }
        }
    }

    // Process pending requests
    if let Some(request) = recording_state.pending_request.take() {
        match request {
            RecordingRequest::Start => {
                match recording_state.mode {
                    RecordingMode::Idle => {
                        if active_character_q.single().is_ok() {
                            // Valid transition: start countdown
                            recording_state.mode = RecordingMode::Countdown;
                            recording_state.countdown_remaining = Some(Duration::from_secs(3));
                            info!("Started recording countdown for active character");
                        } else {
                            warn!("Cannot start recording - no active character found");
                        }
                    }
                    _ => {
                        warn!(
                            "Cannot start recording from state: {:?}",
                            recording_state.mode
                        );
                    }
                }
            }

            RecordingRequest::Stop { reason } => {
                match recording_state.mode {
                    RecordingMode::Recording => {
                        // Valid transition: stop recording
                        recording_state.mode = RecordingMode::Idle;

                        // Remove Recording component from Active Character
                        if let Ok((entity, _, _)) = active_character_q.single() {
                            commands.entity(entity).remove::<Recording>();
                            info!(
                                "Stopped recording for active character {:?} due to: {:?}",
                                entity, reason
                            );
                        } else {
                            warn!("No active character found to stop recording for");
                        }
                    }
                    _ => {
                        warn!(
                            "Cannot stop recording from state: {:?}",
                            recording_state.mode
                        );
                    }
                }
            }

            RecordingRequest::Commit => {
                // Handle commit recording - convert Active Character to ghost
                if let Ok((entity, _, _)) = active_character_q.single() {
                    commands.entity(entity).insert(Ghost).remove::<Recording>();
                    info!(
                        "Committed recording for active character {:?} in arena {:?}",
                        entity,
                        current_arena.id()
                    );
                } else {
                    warn!("Cannot commit recording - no active character found");
                }
            }

            RecordingRequest::Clear => {
                // Handle clear recording - discard timeline for Active Character
                if let Ok((entity, _, _)) = active_character_q.single() {
                    commands.entity(entity).remove::<Recording>();
                    recording_state.mode = RecordingMode::Idle;
                    info!("Cleared recording for active character {:?}", entity);
                } else {
                    warn!("Cannot clear recording - no active character found");
                }
            }

            RecordingRequest::ShowDialog => {
                // Handle show dialog request - transition to DialogPaused state
                match recording_state.mode {
                    RecordingMode::Idle => {
                        recording_state.mode = RecordingMode::DialogPaused;
                        if let Ok((entity, _, _)) = active_character_q.single() {
                            info!("Showing dialog for active ghost character {:?}", entity);
                        } else {
                            info!("Showing dialog for active character (none found)");
                        }
                    }
                    _ => {
                        warn!("Cannot show dialog from state: {:?}", recording_state.mode);
                    }
                }
            }
        }
    }

    // Additional coordination logic can go here:
    // - Dialog pause/resume handling
    // - Timeline synchronization
    // - Ghost spawning
    // - Material updates
    // - UI state updates

    if previous_mode != recording_state.mode {
        info!(
            "Recording state: {:?} → {:?}",
            previous_mode, recording_state.mode
        );
    }
}
```

### Step 4: Create the Recording Plugin

Add to `src/recording/mod.rs`:

```rust
/// Recording Plugin - unified event architecture like camera system
pub struct RecordingPlugin;

impl Plugin for RecordingPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<RecordingState>()
            
            // Unified event architecture
            .add_event::<RecordingUpdate>()  // Root orchestration event
            
            // Systems with explicit ordering
            .add_systems(Update, (
                // Input detection systems (trigger RecordingUpdate)
                detect_recording_input,
                block_recording_interruptions,
                check_recording_time_limit,
                
                // SINGLE ORCHESTRATOR - handles ALL recording logic
                recording_update,
                
            ).chain());
    }
}
```

### Step 5: Wire Into Main

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
    fn test_recording_mode_display() {
        assert_eq!(RecordingMode::Idle.to_string(), "Idle");
        assert_eq!(RecordingMode::Countdown.to_string(), "Countdown");
        assert_eq!(RecordingMode::Recording.to_string(), "Recording");
        assert_eq!(RecordingMode::DialogPaused.to_string(), "DialogPaused");
    }

    #[test]
    fn test_recording_state_default() {
        let state = RecordingState::default();
        assert_eq!(state.mode, RecordingMode::Idle);
        assert!(state.pending_request.is_none());
        assert!(state.countdown_remaining.is_none());
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
    fn test_unified_recording_architecture() {
        use crate::arena::{ArenaEntities, ArenaId, ArenaName, CurrentArena};
        use bevy::app::App;
        use bevy::prelude::*;
        
        // Create test app with unified architecture
        let mut app = App::new();
        app.add_plugins(bevy::time::TimePlugin);
        app.init_resource::<RecordingState>();
        app.add_event::<RecordingUpdate>();
        app.add_systems(Update, recording_update);

        // Add required resources for recording_update system
        let arena_entities = ArenaEntities::new([
            (ArenaName::Labyrinth, Entity::PLACEHOLDER),
            (ArenaName::GuildHouse, Entity::PLACEHOLDER),
            (ArenaName::Sanctum, Entity::PLACEHOLDER),
            (ArenaName::Mountain, Entity::PLACEHOLDER),
            (ArenaName::Bastion, Entity::PLACEHOLDER),
            (ArenaName::Pawnshop, Entity::PLACEHOLDER),
            (ArenaName::Crucible, Entity::PLACEHOLDER),
            (ArenaName::Casino, Entity::PLACEHOLDER),
            (ArenaName::Gala, Entity::PLACEHOLDER),
        ]);
        app.insert_resource(arena_entities);
        app.insert_resource(CurrentArena(ArenaId::new(ArenaName::GuildHouse)));
        
        // Simulate a start recording request
        {
            let mut state = app.world_mut().resource_mut::<RecordingState>();
            state.pending_request = Some(RecordingRequest::Start);
        }
        
        // Trigger recording update
        app.world_mut().send_event(RecordingUpdate);
        
        // Process the update
        app.update();
        
        // Verify state changed to countdown (note: without Active Character, should stay Idle)
        let state = app.world().resource::<RecordingState>();
        // Since no Active Character exists in test, should remain Idle
        assert_eq!(state.mode, RecordingMode::Idle);
        assert!(state.countdown_remaining.is_none());
    }
    
    #[test]
    fn test_recording_request_variants() {
        // Test all request variants can be created
        let requests = vec![
            RecordingRequest::Start,
            RecordingRequest::Stop {
                reason: StopReason::UserInterrupted,
            },
            RecordingRequest::Commit,
            RecordingRequest::Clear,
            RecordingRequest::ShowDialog,
        ];

        // Test that Debug is implemented
        for request in &requests {
            let debug_str = format!("{:?}", request);
            assert!(!debug_str.is_empty());
        }
    }

    #[test]
    fn test_show_dialog_state_transition() {
        use crate::arena::{ArenaEntities, ArenaId, ArenaName, CurrentArena};
        use bevy::app::App;
        use bevy::prelude::*;

        // Create test app with unified architecture
        let mut app = App::new();
        app.add_plugins(bevy::time::TimePlugin);
        app.init_resource::<RecordingState>();
        app.add_event::<RecordingUpdate>();
        app.add_systems(Update, recording_update);

        // Add required resources for recording_update system
        let arena_entities = ArenaEntities::new([
            (ArenaName::Labyrinth, Entity::PLACEHOLDER),
            (ArenaName::GuildHouse, Entity::PLACEHOLDER),
            (ArenaName::Sanctum, Entity::PLACEHOLDER),
            (ArenaName::Mountain, Entity::PLACEHOLDER),
            (ArenaName::Bastion, Entity::PLACEHOLDER),
            (ArenaName::Pawnshop, Entity::PLACEHOLDER),
            (ArenaName::Crucible, Entity::PLACEHOLDER),
            (ArenaName::Casino, Entity::PLACEHOLDER),
            (ArenaName::Gala, Entity::PLACEHOLDER),
        ]);
        app.insert_resource(arena_entities);
        app.insert_resource(CurrentArena(ArenaId::new(ArenaName::GuildHouse)));

        // Simulate a show dialog request
        {
            let mut state = app.world_mut().resource_mut::<RecordingState>();
            state.pending_request = Some(RecordingRequest::ShowDialog);
        }

        // Trigger recording update
        app.world_mut().send_event(RecordingUpdate);

        // Process the update
        app.update();

        // Verify state changed to DialogPaused
        let state = app.world().resource::<RecordingState>();
        assert_eq!(state.mode, RecordingMode::DialogPaused);
    }
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

1. **Unified Event Architecture**: RecordingUpdate as root orchestration event (like CameraUpdate) prevents event explosion
2. **Single Orchestrator**: recording_update() handles ALL coordination (like arena_update())
3. **Clean Separation**: Input systems trigger RecordingUpdate, orchestrator decides what happens
4. **Race Condition Prevention**: RecordingUpdate events processed sequentially by single authority
5. **Simple State Management**: RecordingState as single source of truth (like CurrentArena)
6. **Let-Else Pattern**: Cleaner early returns with proper error handling
7. **Const Keymaps**: Centralized key definitions prevent magic values
8. **Proven Pattern**: Mirrors successful camera system architecture that already solved these problems

## Why Unified Pattern vs Complex Events?

Your camera system already solved this exact problem! The recording system was heading toward event explosion:

### Problems with Complex Event Architecture (What We Avoided):

```rust
// BAD - This leads to event explosion (like the camera system originally had)
pub enum RecordingCommand {
    StartRecording { entity: Entity },
    StopRecording { reason: StopReason },
    PauseForDialog,
    ResumeFromDialog,
    CommitRecording { entity: Entity },
    ClearRecording,
}
```

1. **Event Explosion**: Multiple complex events create coordination nightmares
2. **Race Conditions**: Systems sending different commands in same frame
3. **Complex State Management**: Logic scattered across multiple event handlers
4. **Hard to Debug**: Multiple event flows make troubleshooting difficult
5. **Same Problem**: This is exactly what your camera system originally faced

### Unified Pattern Benefits (Camera System Success Applied):

```rust
// GOOD - Like CameraUpdate, simple orchestration
#[derive(Event, Debug, Clone)]
pub struct RecordingUpdate;

// Simple requests (like arena navigation inputs)
pub enum RecordingRequest { Start, ... }

// Single orchestrator (like arena_update)
pub fn recording_update() { /* handles everything */ }
```

1. **Single Orchestrator**: recording_update() handles ALL logic (like arena_update())
2. **No Race Conditions**: RecordingUpdate processed sequentially
3. **Centralized Logic**: All coordination lives in one place
4. **Easy to Debug**: Clear trigger → RecordingUpdate → response flow
5. **Proven Pattern**: Already works successfully in your camera system

The key insight: **Use the pattern that already works in your codebase**.

## Production Notes

### What We Got Right:

- **Unified Event Architecture**: RecordingUpdate prevents event explosion (proven pattern)
- **Single Orchestrator**: recording_update() centralizes ALL coordination logic
- **Pattern Consistency**: Mirrors successful camera system architecture
- **Let-else patterns**: Reduce nesting and improve readability
- **Const keymaps**: Make input configuration maintainable
- **Simple State Management**: RecordingState as single source of truth

### What We Learned from Camera System:

Your camera system already solved this exact architectural challenge:
- **CameraUpdate** → **RecordingUpdate** (root orchestration event)
- **arena_update()** → **recording_update()** (single coordinator)
- Multiple trigger systems → Single orchestrator pattern
- Clean event flow → No race conditions

### Why This Pattern Works:

- **Prevention First**: Stop event explosion before it starts
- **Proven Success**: Camera system demonstrates this architecture works
- **Single Authority**: One system owns all recording coordination
- **Easy to Debug**: Clear trigger → RecordingUpdate → response flow
- **Consistent Codebase**: Same pattern across camera and recording systems

This architecture applies the lessons learned from your camera system to prevent the same complexity explosion in recording. By using the proven pattern, we avoid race conditions and maintain clean, debuggable code that's consistent with your existing successful systems.

## Architectural Note: Active Character Query Pattern

### Core Design Principle
The critical architectural insight is that **recording entity = Active Character in Current Arena (ALWAYS!)** This eliminates the need to store or pass entity parameters.

### Implementation Benefits
```rust
// ✅ CORRECT - Query Active Character when needed
pub enum RecordingRequest {
    Start,        // Query active_character_q.single() when processing
    ShowDialog,   // Query active_character_q.single() when showing dialog
}

// ❌ INCORRECT - Storing/passing entities
pub enum RecordingRequest {
    Start { entity: Entity },           // Don't store entities
    ShowDialog { character: Entity },   // Don't pass character data
}
```

### Why This Works
1. **Single Source of Truth**: The `Active` component determines which character receives input
2. **R Key Behavior**: "R always operates on whoever is CURRENTLY active when pressed"
3. **Dynamic Dialog Behavior**: Dialog should switch if player changes active character
4. **Type Safety**: `active_character_q.single()` guarantees exactly one Active Character
5. **Simplicity**: Eliminates entity synchronization and parameter passing complexity

This pattern ensures the recording system is always in sync with the player's current selection without complex entity tracking.