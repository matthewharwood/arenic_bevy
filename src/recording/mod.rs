// Recording module - implements unified event architecture
// Based on Tutorial 02: Recording State Machine (unified orchestrator pattern)

use crate::arena::{Arena, ArenaEntities, CurrentArena};
use crate::character::Character;
use crate::selectors::Active;
use crate::timeline::TimelineClock;
use bevy::prelude::*;
use std::fmt::{self, Display, Formatter};
use std::time::Duration;

// === ROOT EVENT - Like CameraUpdate ===
/// Root orchestration event that triggers recording coordination
/// Multiple systems can trigger this, single orchestrator handles all logic
#[derive(Event, Debug, Clone)]
pub struct RecordingUpdate;

// === TRIGGER EVENTS - Simple requests ===
/// Simple request events that express intent (like arena navigation inputs)
#[derive(Event, Debug)]
pub enum RecordingRequest {
    Start { entity: Entity },
    Stop { reason: StopReason },
    Commit, // Uses state.recording_entity and CurrentArena resource
    Clear,  // Uses state.recording_entity
    ShowDialog { character: Entity },
}

// === STATE RESOURCE - Single source of truth ===
/// Global recording state for the game (like CurrentArena)
#[derive(Resource, Debug)]
pub struct RecordingState {
    pub mode: RecordingMode,
    pub recording_entity: Option<Entity>,
    pub pending_request: Option<RecordingRequest>,
    pub countdown_remaining: Option<Duration>,
}

impl Default for RecordingState {
    fn default() -> Self {
        Self {
            mode: RecordingMode::Idle,
            recording_entity: None,
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
    /// Dialog shown, all timelines paused
    DialogPaused { character: Entity },
}

impl Display for RecordingMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Idle => write!(f, "Idle"),
            Self::Countdown => write!(f, "Countdown"),
            Self::Recording => write!(f, "Recording"),
            Self::DialogPaused { .. } => write!(f, "DialogPaused"),
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

// === INPUT DETECTION SYSTEMS ===
// These systems detect user input and trigger RecordingUpdate events

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
                recording_state.pending_request = Some(RecordingRequest::ShowDialog {
                    character: character_entity,
                });
                recording_update_events.write(RecordingUpdate);
                info!(
                    "Cannot record a ghost - showing retry dialog for character {:?}",
                    character_entity
                );
            } else {
                // Store request and trigger update
                recording_state.pending_request = Some(RecordingRequest::Start {
                    entity: character_entity,
                });
                recording_update_events.write(RecordingUpdate);
                info!(
                    "Recording start requested for character {:?}",
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

// === SINGLE ORCHESTRATOR - Like arena_update() ===

/// SINGLE ORCHESTRATOR - Like arena_update(), handles ALL recording logic
/// Processes RecordingUpdate events and coordinates all recording operations
pub fn recording_update(
    mut recording_update_events: EventReader<RecordingUpdate>,
    mut recording_state: ResMut<RecordingState>,
    mut commands: Commands,
    time: Res<Time>,
    // All the queries needed for recording coordination
    arena_q: Query<(&Arena, &TimelineClock)>,
    arena_entities: Res<ArenaEntities>,
    current_arena: Res<CurrentArena>,
    characters_q: Query<
        (Entity, Option<&Active>, Option<&Recording>, Option<&Ghost>),
        With<Character>,
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

                if let Some(entity) = recording_state.recording_entity {
                    commands.entity(entity).insert(Recording);
                    info!("Recording started for entity {:?}", entity);
                }
            }
        }
    }

    // Process pending requests
    if let Some(request) = recording_state.pending_request.take() {
        match request {
            RecordingRequest::Start { entity } => {
                match recording_state.mode {
                    RecordingMode::Idle => {
                        // Valid transition: start countdown
                        recording_state.mode = RecordingMode::Countdown;
                        recording_state.recording_entity = Some(entity);
                        recording_state.countdown_remaining = Some(Duration::from_secs(3));

                        info!("Started recording countdown for entity {:?}", entity);
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

                        if let Some(entity) = recording_state.recording_entity.take() {
                            commands.entity(entity).remove::<Recording>();
                            info!(
                                "Stopped recording for entity {:?} due to: {:?}",
                                entity, reason
                            );
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
                // Handle commit recording - convert to ghost using state.recording_entity
                if let Some(entity) = recording_state.recording_entity {
                    commands.entity(entity).insert(Ghost);
                    info!(
                        "Committed recording for entity {:?} in arena {:?}",
                        entity,
                        current_arena.id()
                    );
                } else {
                    warn!("Cannot commit recording - no recording entity in state");
                }
            }

            RecordingRequest::Clear => {
                // Handle clear recording - discard timeline using state.recording_entity
                if let Some(entity) = recording_state.recording_entity {
                    commands.entity(entity).remove::<Recording>();
                    recording_state.recording_entity = None;
                    recording_state.mode = RecordingMode::Idle;
                    info!("Cleared recording for entity {:?}", entity);
                } else {
                    warn!("Cannot clear recording - no recording entity in state");
                }
            }

            RecordingRequest::ShowDialog { character } => {
                // Handle show dialog request - transition to DialogPaused state
                match recording_state.mode {
                    RecordingMode::Idle => {
                        recording_state.mode = RecordingMode::DialogPaused { character };
                        info!("Showing dialog for ghost character {:?}", character);
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

// === RECORDING PLUGIN ===

/// Recording Plugin - unified event architecture like camera system
pub struct RecordingPlugin;

impl Plugin for RecordingPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<RecordingState>()
            // Unified event architecture
            .add_event::<RecordingUpdate>() // Root orchestration event
            // Systems with explicit ordering
            .add_systems(
                Update,
                (
                    // Input detection systems (trigger RecordingUpdate)
                    detect_recording_input,
                    block_recording_interruptions,
                    check_recording_time_limit,
                    // SINGLE ORCHESTRATOR - handles ALL recording logic
                    recording_update,
                )
                    .chain(),
            );
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
        assert_eq!(
            RecordingMode::DialogPaused {
                character: Entity::PLACEHOLDER
            }
            .to_string(),
            "DialogPaused"
        );
    }

    #[test]
    fn test_recording_state_default() {
        let state = RecordingState::default();
        assert_eq!(state.mode, RecordingMode::Idle);
        assert!(state.recording_entity.is_none());
        assert!(state.pending_request.is_none());
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
            state.pending_request = Some(RecordingRequest::Start {
                entity: Entity::PLACEHOLDER,
            });
        }

        // Trigger recording update
        app.world_mut().send_event(RecordingUpdate);

        // Process the update
        app.update();

        // Verify state changed to countdown
        let state = app.world().resource::<RecordingState>();
        assert_eq!(state.mode, RecordingMode::Countdown);
        assert!(state.recording_entity.is_some());
        assert!(state.countdown_remaining.is_some());
    }

    #[test]
    fn test_recording_request_variants() {
        // Test all request variants can be created
        let requests = vec![
            RecordingRequest::Start {
                entity: Entity::PLACEHOLDER,
            },
            RecordingRequest::Stop {
                reason: StopReason::UserInterrupted,
            },
            RecordingRequest::Commit,
            RecordingRequest::Clear,
            RecordingRequest::ShowDialog {
                character: Entity::PLACEHOLDER,
            },
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

        let test_character = Entity::from_raw(42);

        // Simulate a show dialog request
        {
            let mut state = app.world_mut().resource_mut::<RecordingState>();
            state.pending_request = Some(RecordingRequest::ShowDialog {
                character: test_character,
            });
        }

        // Trigger recording update
        app.world_mut().send_event(RecordingUpdate);

        // Process the update
        app.update();

        // Verify state changed to DialogPaused with correct character
        let state = app.world().resource::<RecordingState>();
        match state.mode {
            RecordingMode::DialogPaused { character } => {
                assert_eq!(character, test_character);
            }
            _ => panic!("Expected DialogPaused state, got: {:?}", state.mode),
        }
    }
}
