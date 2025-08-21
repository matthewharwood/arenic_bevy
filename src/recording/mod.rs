// Recording module - implements unified event architecture

use crate::arena::{Arena, CameraUpdate, CurrentArena, CurrentArenaEntity};
use crate::character::Character;
use crate::selectors::Active;
use crate::timeline::{TimeStamp, TimelineClock};
use bevy::prelude::*;
use std::fmt::{self, Display, Formatter};
use std::time::Duration;

/// Root orchestration event that triggers recording coordination
#[derive(Event, Debug, Clone)]
pub struct RecordingUpdate;

/// Simple request events that express intent
#[derive(Event, Debug)]
pub enum RecordingRequest {
    Start,
    Stop { reason: StopReason },
    Commit,
    Clear,
    ShowDialog,
    ShowCancelDialog,
    ShowSwitchDialog,
}

/// Global recording state for the game
#[derive(Resource, Debug)]
pub struct RecordingState {
    pub mode: RecordingMode,
    pub pending_request: Option<RecordingRequest>,
    pub countdown_remaining: Option<Duration>,
}

impl RecordingState {
    pub const COUNTDOWN_DURATION: Duration = Duration::from_secs(3);
    pub const MAX_RECORDING_TIME: f32 = 120.0;
    pub const MIN_RECORDING_TIME: f32 = 1.0;
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
    UserInterrupted,
    TimeComplete,
    ArenaTransition,
    CharacterSwitch,
}

/// Recording component marker for entity state
#[derive(Component)]
pub struct Recording;

/// Ghost component marker for entity state
#[derive(Component)]
pub struct Ghost;

/// Additional recording state markers for precise filtering
#[derive(Component)]
pub struct RecordingPending;

#[derive(Component)]
pub struct RecordingActive;

#[derive(Component)]
pub struct RecordingPaused;

const KEY_RECORD: KeyCode = KeyCode::KeyR;
const KEY_ARENA_PREV: KeyCode = KeyCode::BracketLeft;
const KEY_ARENA_NEXT: KeyCode = KeyCode::BracketRight;
const KEY_CHARACTER_SWITCH: KeyCode = KeyCode::Tab;

const RECORDING_CONTROLS: [(KeyCode, &str); 4] = [
    (KEY_RECORD, "Record/Stop"),
    (KEY_ARENA_PREV, "Previous Arena"),
    (KEY_ARENA_NEXT, "Next Arena"),
    (KEY_CHARACTER_SWITCH, "Switch Character"),
];

/// Detect when player presses R to start/stop recording
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
                recording_state.pending_request = Some(RecordingRequest::ShowDialog);
                info!(
                    "Cannot record a ghost - showing retry dialog for active character {:?}",
                    character_entity
                );
            } else {
                recording_state.pending_request = Some(RecordingRequest::Start);
                info!(
                    "Recording start requested for active character {:?}",
                    character_entity
                );
            }
            recording_update_events.write(RecordingUpdate);
        }
        RecordingMode::Recording => {
            recording_state.pending_request = Some(RecordingRequest::ShowCancelDialog);
            recording_update_events.write(RecordingUpdate);
            info!("Recording interruption requested - showing confirmation dialog");
        }
        _ => {}
    }
}

/// Block arena switching during recording
pub fn block_recording_interruptions(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_update_events: EventReader<CameraUpdate>,
    mut recording_state: ResMut<RecordingState>,
    mut recording_update_events: EventWriter<RecordingUpdate>,
) {
    // Only check during active recording
    if recording_state.mode != RecordingMode::Recording {
        return;
    }
    if camera_update_events.read().next().is_some() || keyboard.just_pressed(KEY_CHARACTER_SWITCH) {
        recording_state.pending_request = Some(RecordingRequest::ShowSwitchDialog);
        recording_update_events.write(RecordingUpdate);
    }
}

/// Check if the recording time limit reached
pub fn check_recording_time_limit(
    arena_q: Query<&TimelineClock, With<Arena>>,
    current: CurrentArenaEntity,
    mut recording_state: ResMut<RecordingState>,
    mut recording_update_events: EventWriter<RecordingUpdate>,
) {
    // Only check during active recording
    if recording_state.mode != RecordingMode::Recording {
        return;
    }

    let Ok(clock) = arena_q.get(current.get()) else {
        return;
    };

    if clock.current().as_secs() > TimeStamp::MAX.0 {
        recording_state.pending_request = Some(RecordingRequest::ShowDialog);
        recording_update_events.write(RecordingUpdate);
        info!("Recording time limit reached");
    }
}

/// Single orchestrator that handles ALL recording logic
pub fn recording_update(
    mut recording_update_events: EventReader<RecordingUpdate>,
    mut recording_state: ResMut<RecordingState>,
    mut commands: Commands,
    time: Res<Time>,
    current_arena: Res<CurrentArena>,
    active_character_q: Query<
        (Entity, Option<&Recording>, Option<&Ghost>),
        (With<Character>, With<Active>),
    >,
) {
    if let Some(_) = recording_update_events.read().next() {
        let previous_mode = recording_state.mode;

        // Handle countdown progression
        if recording_state.mode == RecordingMode::Countdown {
            if let Some(ref mut remaining) = recording_state.countdown_remaining {
                *remaining = remaining.saturating_sub(time.delta());

                if remaining.is_zero() {
                    recording_state.mode = RecordingMode::Recording;
                    recording_state.countdown_remaining = None;

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

        if let Some(request) = recording_state.pending_request.take() {
            match request {
                RecordingRequest::Start => match recording_state.mode {
                    RecordingMode::Idle => {
                        if active_character_q.single().is_ok() {
                            recording_state.mode = RecordingMode::Countdown;
                            recording_state.countdown_remaining =
                                Some(RecordingState::COUNTDOWN_DURATION);
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
                },

                RecordingRequest::Stop { reason } => match recording_state.mode {
                    RecordingMode::Recording => {
                        recording_state.mode = RecordingMode::Idle;

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
                },

                RecordingRequest::Commit => {
                    if let Ok((entity, _, _)) = active_character_q.single() {
                        commands.entity(entity).insert(Ghost).remove::<Recording>();
                        info!(
                            "Committed recording for active character {:?} in arena {:?}",
                            entity, current_arena.0
                        );
                    } else {
                        warn!("Cannot commit recording - no active character found");
                    }
                }

                RecordingRequest::Clear => {
                    if let Ok((entity, _, _)) = active_character_q.single() {
                        commands.entity(entity).remove::<Recording>();
                        recording_state.mode = RecordingMode::Idle;
                        info!("Cleared recording for active character {:?}", entity);
                    } else {
                        warn!("Cannot clear recording - no active character found");
                    }
                }

                RecordingRequest::ShowDialog => match recording_state.mode {
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
                },

                RecordingRequest::ShowCancelDialog => match recording_state.mode {
                    RecordingMode::Recording => {
                        recording_state.mode = RecordingMode::DialogPaused;
                        if let Ok((entity, _, _)) = active_character_q.single() {
                            info!(
                                "Showing recording cancellation dialog for active character {:?}",
                                entity
                            );
                        } else {
                            info!(
                                "Showing recording cancellation dialog for active character (none found)"
                            );
                        }
                    }
                    _ => {
                        warn!(
                            "Cannot show cancel dialog from state: {:?}",
                            recording_state.mode
                        );
                    }
                },

                RecordingRequest::ShowSwitchDialog => match recording_state.mode {
                    RecordingMode::Recording => {
                        recording_state.mode = RecordingMode::DialogPaused;
                        if let Ok((entity, _, _)) = active_character_q.single() {
                            info!(
                                "Showing character switch confirmation dialog for active character {:?}",
                                entity
                            );
                        } else {
                            info!(
                                "Showing character switch confirmation dialog for active character (none found)"
                            );
                        }
                    }
                    _ => {
                        warn!(
                            "Cannot show switch dialog from state: {:?}",
                            recording_state.mode
                        );
                    }
                },
            }
        }

        if previous_mode != recording_state.mode {
            info!(
                "Recording state: {:?} → {:?}",
                previous_mode, recording_state.mode
            );
        }
    }
}

/// Recording Plugin
pub struct RecordingPlugin;

impl Plugin for RecordingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RecordingState>()
            .add_event::<RecordingUpdate>()
            .add_systems(
                Update,
                (
                    detect_recording_input,
                    block_recording_interruptions,
                    check_recording_time_limit,
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
        use crate::arena::{ArenaName, CurrentArena};
        use bevy::app::App;
        use bevy::prelude::*;

        // Create test app with unified architecture
        let mut app = App::new();
        app.add_plugins(bevy::time::TimePlugin);
        app.init_resource::<RecordingState>();
        app.add_event::<RecordingUpdate>();
        app.add_systems(Update, recording_update);

        // Add required resources for recording_update system
        app.insert_resource(CurrentArena(ArenaName::GuildHouse));

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
            RecordingRequest::ShowCancelDialog,
            RecordingRequest::ShowSwitchDialog,
        ];

        // Test that Debug is implemented
        for request in &requests {
            let debug_str = format!("{:?}", request);
            assert!(!debug_str.is_empty());
        }
    }

    #[test]
    fn test_show_dialog_state_transition() {
        use crate::arena::{ArenaName, CurrentArena};
        use bevy::app::App;
        use bevy::prelude::*;

        // Create test app with unified architecture
        let mut app = App::new();
        app.add_plugins(bevy::time::TimePlugin);
        app.init_resource::<RecordingState>();
        app.add_event::<RecordingUpdate>();
        app.add_systems(Update, recording_update);

        // Add required resources for recording_update system
        app.insert_resource(CurrentArena(ArenaName::GuildHouse));

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
