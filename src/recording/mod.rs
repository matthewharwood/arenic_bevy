// Recording module - implements unified event architecture
// Based on Tutorial 02: Recording State Machine (refactored version)

use bevy::prelude::*;
use std::fmt::{self, Display, Formatter};
use std::time::Duration;
use crate::timeline::TimelineClock;
use crate::arena::{Arena, ArenaId, CurrentArena, ArenaEntities};
use crate::character::Character;
use crate::selectors::Active;


// === TRIGGER EVENTS - Simple requests ===
/// Simple request events that express intent (like arena navigation inputs)
/// SIMPLIFIED: Recording entity is ALWAYS the active character in current arena
#[derive(Event, Debug)]
pub enum RecordingRequest {
    Start,  // System finds active character automatically
    Stop { reason: StopReason },
    Commit, // Uses active character with Recording component
    Clear,  // Uses active character with Recording component
}

// === STATE RESOURCE - Single source of truth ===
/// Global recording state for the game (like CurrentArena)
/// SIMPLIFIED: No recording_entity - can always query for active character with Recording component
#[derive(Resource, Debug)]
pub struct RecordingState {
    pub mode: RecordingMode,
    pub countdown_remaining: Option<Duration>,
}

impl Default for RecordingState {
    fn default() -> Self {
        Self {
            mode: RecordingMode::Idle,
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

#[derive(Debug, Clone, Copy)]
pub enum StopReason {
    UserInterrupted,    // User pressed R again
    TimeComplete,       // 120 seconds elapsed
    ArenaTransition,    // Tried to leave arena
    CharacterSwitch,    // Tried to switch characters
}

/// Marker component for characters currently being recorded
#[derive(Component)]
pub struct Recording;

/// Marker for entities that already have a published timeline
#[derive(Component)]
pub struct Ghost;

// === INPUT DETECTION SYSTEMS ===
// These systems detect user input and trigger RecordingUpdate events

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

/// SIMPLIFIED: Detect when player presses R to start/stop recording
/// Recording entity is ALWAYS the active character - no entity checking needed
pub fn detect_recording_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    recording_state: Res<RecordingState>,
    active_character: Option<Single<(Entity, Option<&Ghost>), (With<Character>, With<Active>)>>,
    mut recording_request_events: EventWriter<RecordingRequest>,
    mut retry_dialog_events: EventWriter<ShowRetryDialog>,
    current_arena: Res<CurrentArena>,
) {
    if !keyboard.just_pressed(KEY_RECORD) {
        return;
    }

    match recording_state.mode {
        RecordingMode::Idle => {
            let Some(single) = active_character else {
                return; // No active character to record
            };
            let (character_entity, ghost_marker) = single.into_inner();

            if ghost_marker.is_some() {
                // Ghost selected - show retry dialog
                retry_dialog_events.write(ShowRetryDialog {
                    character: character_entity,
                    arena: current_arena.id(),
                });
                info!("Cannot record a ghost - showing retry dialog for character {:?}", character_entity);
            } else {
                // SIMPLIFIED: Just send start request - system will find active character
                recording_request_events.write(RecordingRequest::Start);
                info!("Recording start requested - system will find active character");
            }
        }
        RecordingMode::Recording => {
            // Send stop request directly
            recording_request_events.write(RecordingRequest::Stop {
                reason: StopReason::UserInterrupted,
            });
            info!("Recording stop requested - user interrupted");
        }
        _ => {
            // Ignore input in other states
        }
    }
}

/// Block arena/character switching during recording
/// Directly sends stop recording request if blocked actions attempted
pub fn block_recording_interruptions(
    keyboard: Res<ButtonInput<KeyCode>>,
    recording_state: Res<RecordingState>,
    mut recording_request_events: EventWriter<RecordingRequest>,
) {
    // Only check during active recording
    if recording_state.mode != RecordingMode::Recording {
        return;
    }

    let stop_reason = if keyboard.just_pressed(KEY_ARENA_PREV) || 
                         keyboard.just_pressed(KEY_ARENA_NEXT) {
        Some(StopReason::ArenaTransition)
    } else if keyboard.just_pressed(KEY_CHARACTER_SWITCH) {
        Some(StopReason::CharacterSwitch)
    } else {
        None
    };

    if let Some(reason) = stop_reason {
        recording_request_events.write(RecordingRequest::Stop { reason });
        info!("Blocked action during recording: {:?}", reason);
    }
}

/// Check if recording time limit reached
/// Directly sends stop recording request when time limit reached
pub fn check_recording_time_limit(
    arena_q: Query<(&Arena, &TimelineClock)>,
    arena_entities: Res<ArenaEntities>,
    current_arena: Res<CurrentArena>,
    recording_state: Res<RecordingState>,
    mut recording_request_events: EventWriter<RecordingRequest>,
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

    if clock.current().as_secs() >= 120.0 - 0.1 { // 120 second recording limit
        recording_request_events.write(RecordingRequest::Stop {
            reason: StopReason::TimeComplete,
        });
        info!("Recording time limit reached");
    }
}

// === SINGLE ORCHESTRATOR - Like arena_update() ===

/// SIMPLIFIED ORCHESTRATOR - Like arena_update(), handles ALL recording logic
/// Recording entity is ALWAYS the active character - no entity storage needed
pub fn recording_update(
    mut recording_request_events: EventReader<RecordingRequest>,
    mut recording_state: ResMut<RecordingState>,
    mut commands: Commands,
    time: Res<Time>,
    current_arena: Res<CurrentArena>,
    // SIMPLIFIED: Query for active character directly instead of storing entity
    active_character_q: Query<Entity, (With<Character>, With<Active>, Without<Ghost>)>,
    recording_character_q: Query<Entity, (With<Character>, With<Active>, With<Recording>)>,
) {
    let previous_mode = recording_state.mode;
    
    // Handle countdown progression (always check this)
    if recording_state.mode == RecordingMode::Countdown {
        if let Some(ref mut remaining) = recording_state.countdown_remaining {
            *remaining = remaining.saturating_sub(time.delta());
            
            if remaining.is_zero() {
                // Countdown complete - transition to recording
                recording_state.mode = RecordingMode::Recording;
                recording_state.countdown_remaining = None;
                
                // SIMPLIFIED: Find active character and add Recording component
                if let Ok(active_entity) = active_character_q.single() {
                    commands.entity(active_entity).insert(Recording);
                    info!("Recording started for active character {:?}", active_entity);
                } else {
                    warn!("No active character found to start recording");
                    recording_state.mode = RecordingMode::Idle;
                }
            }
        }
    }
    
    // Process RecordingRequest events directly
    for request in recording_request_events.read() {
        match request {
            RecordingRequest::Start => {
                match recording_state.mode {
                    RecordingMode::Idle => {
                        // SIMPLIFIED: Just verify active character exists and start countdown
                        if active_character_q.single().is_ok() {
                            recording_state.mode = RecordingMode::Countdown;
                            recording_state.countdown_remaining = Some(Duration::from_secs(3));
                            info!("Started recording countdown for active character");
                        } else {
                            warn!("Cannot start recording - no active character found");
                        }
                    }
                    _ => {
                        warn!("Cannot start recording from state: {:?}", recording_state.mode);
                    }
                }
            }
            
            RecordingRequest::Stop { reason } => {
                match recording_state.mode {
                    RecordingMode::Recording => {
                        // SIMPLIFIED: Find character with Recording component and remove it
                        if let Ok(recording_entity) = recording_character_q.single() {
                            commands.entity(recording_entity).remove::<Recording>();
                            recording_state.mode = RecordingMode::Idle;
                            info!("Stopped recording for entity {:?} due to: {:?}", recording_entity, reason);
                        } else {
                            warn!("Cannot stop recording - no recording character found");
                        }
                    }
                    _ => {
                        warn!("Cannot stop recording from state: {:?}", recording_state.mode);
                    }
                }
            }
            
            RecordingRequest::Commit => {
                // SIMPLIFIED: Find character with Recording component and convert to ghost
                if let Ok(recording_entity) = recording_character_q.single() {
                    commands.entity(recording_entity)
                        .remove::<Recording>()
                        .insert(Ghost);
                    recording_state.mode = RecordingMode::Idle;
                    info!("Committed recording for entity {:?} in arena {:?}", recording_entity, current_arena.id());
                } else {
                    warn!("Cannot commit recording - no recording character found");
                }
            }
            
            RecordingRequest::Clear => {
                // SIMPLIFIED: Find character with Recording component and clear it
                if let Ok(recording_entity) = recording_character_q.single() {
                    commands.entity(recording_entity).remove::<Recording>();
                    recording_state.mode = RecordingMode::Idle;
                    info!("Cleared recording for entity {:?}", recording_entity);
                } else {
                    warn!("Cannot clear recording - no recording character found");
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
        info!("Recording state: {:?} → {:?}", previous_mode, recording_state.mode);
    }
}

// === RECORDING PLUGIN ===

/// Recording Plugin - unified event architecture like camera system
pub struct RecordingPlugin;

impl Plugin for RecordingPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<RecordingState>()
            
            // Simplified event architecture - direct events only
            .add_event::<RecordingRequest>()  // Direct recording requests
            .add_event::<ShowRetryDialog>()   // UI events
            
            // Systems with explicit ordering
            .add_systems(Update, (
                // Input detection systems (send RecordingRequest events)
                detect_recording_input,
                block_recording_interruptions,
                check_recording_time_limit,
                
                // SINGLE ORCHESTRATOR - handles ALL recording logic
                recording_update,
                
            ).chain());
    }
}

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
        assert!(state.countdown_remaining.is_none());
    }

    #[test]
    fn test_stop_reason_variants() {
        let reasons = vec![
            StopReason::UserInterrupted,
            StopReason::TimeComplete,
            StopReason::ArenaTransition,
            StopReason::CharacterSwitch,
        ];

        for reason in &reasons {
            let debug_str = format!("{:?}", reason);
            assert!(!debug_str.is_empty());
        }
    }
    
    #[test]
    fn test_simplified_recording_architecture() {
        use bevy::app::App;
        use bevy::prelude::*;
        use crate::arena::{ArenaName, ArenaId, ArenaEntities, CurrentArena};
        
        // Create test app with simplified architecture
        let mut app = App::new();
        app.add_plugins(bevy::time::TimePlugin);
        app.init_resource::<RecordingState>();
        app.add_event::<RecordingRequest>();
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
        
        // SIMPLIFIED: Send start recording request without entity
        app.world_mut().send_event(RecordingRequest::Start);
        
        // Process the update
        app.update();
        
        // Verify state changed to countdown (would need active character to fully work)
        let state = app.world().resource::<RecordingState>();
        // Note: mode won't change without an active character, but that's correct behavior
        assert_eq!(state.mode, RecordingMode::Idle); // No active character = no recording
        assert!(state.countdown_remaining.is_none());
    }
    
    #[test]
    fn test_recording_request_variants() {
        let requests = vec![
            RecordingRequest::Start, // SIMPLIFIED: No entity parameter
            RecordingRequest::Stop { reason: StopReason::UserInterrupted },
            RecordingRequest::Commit,
            RecordingRequest::Clear,
        ];

        for request in &requests {
            let debug_str = format!("{:?}", request);
            assert!(!debug_str.is_empty());
        }
    }
}