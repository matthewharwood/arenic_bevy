use crate::arena::{ArenaId, ArenaName};
use crate::character::Character;
use crate::selectors::Active;
use bevy::prelude::*;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::time::Duration;

#[cfg(test)]
mod tests;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecordingMode {
    Idle,
    Countdown,
    Recording,
    DialogPause,
}

impl Display for RecordingMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            RecordingMode::Idle => write!(f, "Idle"),
            RecordingMode::Countdown => write!(f, "Countdown"),
            RecordingMode::Recording => write!(f, "Recording"),
            RecordingMode::DialogPause => write!(f, "Dialog Pause"),
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
    StartRequest(Entity),
    CountdownComplete,
    UserInterrupted,
    TimeComplete,
    ArenaTransition,
    CharacterSwitch,
    DialogOpened,
    DialogClosed,
}

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

/// Event to start recording a character
#[derive(Event)]
pub struct StartRecording {
    pub character: Entity,
    pub arena: ArenaId,
}

/// Event to stop recording (user interruption)
#[derive(Event)]
pub struct StopRecording {
    pub reason: StopReason,
}

#[derive(Debug, Clone)]
pub enum StopReason {
    UserInterrupted, // User pressed R again
    TimeComplete,    // 120 seconds elapsed
    ArenaTransition, // Tried to leave an arena
    CharacterSwitch, // Tried to switch characters
}
/// Event to commit a recorded timeline for a specific character in a specific arena
/// This fixes the critical architectural issue: we now track which arena the recording is for
#[derive(Event)]
pub struct CommitRecording {
    pub character: Entity,
    pub arena: ArenaId,
}
#[derive(Event)]
pub struct ResetArenaTimeline {
    pub arena: ArenaId,
}

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
            let Some(active_single) = active_character else {
                return;
            };
            let (character_entity, ghost_marker) = active_single.into_inner();

            // Check if this character is a ghost
            if ghost_marker.is_some() {
                // Ghost selected - show retry dialog
                retry_dialog_events.write(ShowRetryDialog {
                    character: character_entity,
                    arena: ArenaId::new(ArenaName::GuildHouse), // TODO: get current arena
                });
                info!("Cannot record a ghost - showing retry dialog for character {:?}", character_entity);
            } else {
                // Regular character - send start recording command
                command_writer.write(RecordingCommand::StartRecording { 
                    entity: character_entity 
                });
                info!("Sending start recording command for character {:?}", character_entity);
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

/// State machine that processes commands and emits state change events
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
        }
    }
}

/// Recording Plugin - configures all recording systems
pub struct RecordingPlugin;

impl Plugin for RecordingPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<RecordingState>()
            
            // Command/Event architecture
            .add_event::<RecordingCommand>()
            .add_event::<RecordingStateChanged>()
            
            // Legacy events (for backward compatibility)
            .add_event::<StartRecording>()
            .add_event::<StopRecording>()
            .add_event::<CommitRecording>()
            .add_event::<ResetArenaTimeline>()
            .add_event::<ShowRetryDialog>()
            
            // Systems with explicit ordering
            .add_systems(Update, (
                // Input detection (generates commands)
                detect_recording_input,
                
                // State machine (processes commands, emits events)
                process_recording_commands,
                
                // Internal state handling
                handle_countdown_completion,
                
            ).chain());
    }
}
