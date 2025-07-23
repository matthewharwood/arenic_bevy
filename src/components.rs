//! Game components definitions.
//!
//! This module contains all the component types used in the ECS architecture.
//! Components represent data that can be attached to entities.

use crate::movement::MovementDirection;
use bevy::prelude::*;
use std::time::Duration;

/// Tracks the currently active arena (0-8)
#[derive(Component, Debug, Clone)]
pub struct CurrentArena(pub u8);

impl CurrentArena {
    /// Increment arena index cyclically (0-8)
    pub fn increment(value: u8) -> u8 {
        (value + 1) % 9
    }

    /// Decrement arena index cyclically (0-8)  
    pub fn decrement(value: u8) -> u8 {
        if value == 0 { 8 } else { value - 1 }
    }
}

/// Component for entities that represent game characters
#[derive(Component, Debug, Clone)]
pub struct Character {
    pub name: String,
}

impl Character {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// Marker component for the currently selected character
#[derive(Component, Debug, Clone)]
pub struct CharacterSelected;

/// Arena status for controlling timer and playback behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArenaStatus {
    /// Default state - timer paused, no recording or playback
    Paused,
    /// Recording state - timer ticking, selected character recording
    Recording,
    /// Playback state - timer ticking, all characters replaying saved sessions
    Playback,
}

impl Default for ArenaStatus {
    fn default() -> Self {
        ArenaStatus::Paused
    }
}

/// Timer component for each arena with status management
#[derive(Component, Debug, Clone)]
pub struct ArenaTimer {
    pub timer: Timer,
    pub arena: ArenaName,
    pub status: ArenaStatus,
}

impl ArenaTimer {
    /// Create a new arena timer with a default 2-minute duration (starts Paused)
    pub fn new(arena: ArenaName) -> Self {
        let mut timer = Timer::new(Duration::from_secs(120), TimerMode::Repeating);
        timer.pause(); // Start paused
        Self {
            timer,
            arena,
            status: ArenaStatus::Paused,
        }
    }

    /// Set arena status and manage timer accordingly
    pub fn set_status(&mut self, status: ArenaStatus) {
        self.status = status;
        match status {
            ArenaStatus::Paused => {
                self.timer.pause();
            }
            ArenaStatus::Recording | ArenaStatus::Playback => {
                if self.timer.paused() {
                    self.timer.unpause();
                }
            }
        }
    }

    /// Get current arena status
    pub fn get_status(&self) -> ArenaStatus {
        self.status
    }

    /// Check if an arena is in playback state
    pub fn is_playback(&self) -> bool {
        self.status == ArenaStatus::Playback
    }

    /// Check if an arena is paused
    pub fn is_paused(&self) -> bool {
        self.status == ArenaStatus::Paused
    }
}

/// Arena identification by name
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArenaName {
    Labyrinth = 0,
    GuildHouse = 1,
    Sanctum = 2,
    Mountain = 3,
    Bastion = 4,
    Pawnshop = 5,
    Crucible = 6,
    Casino = 7,
    Gala = 8,
}

impl ArenaName {
    /// Create an ArenaName from an index (0-8)
    ///
    /// # Panics /// if the index is not in the range 0-8
    pub fn from_index(index: u8) -> ArenaName {
        match index {
            0 => ArenaName::Labyrinth,
            1 => ArenaName::GuildHouse,
            2 => ArenaName::Sanctum,
            3 => ArenaName::Mountain,
            4 => ArenaName::Bastion,
            5 => ArenaName::Pawnshop,
            6 => ArenaName::Crucible,
            7 => ArenaName::Casino,
            8 => ArenaName::Gala,
            _ => panic!("Invalid arena index: {}", index),
        }
    }

    /// Convert ArenaName to its numeric index
    pub fn to_index(&self) -> u8 {
        *self as u8
    }

    /// Get the human-readable name of the arena
    pub fn name(&self) -> &'static str {
        match self {
            ArenaName::Labyrinth => "Labyrinth",
            ArenaName::GuildHouse => "Guild House",
            ArenaName::Sanctum => "Sanctum",
            ArenaName::Mountain => "Mountain",
            ArenaName::Bastion => "Bastion",
            ArenaName::Pawnshop => "Pawnshop",
            ArenaName::Crucible => "Crucible",
            ArenaName::Casino => "Casino",
            ArenaName::Gala => "Gala",
        }
    }
}

// UI component markers
/// Marker component for the top navigation bar
#[derive(Component, Debug, Clone)]
pub struct TopNavBar;

/// Marker component for side navigation bars
#[derive(Component, Debug, Clone)]
pub struct SideNavBar;

/// Marker component for the bottom navigation bar
#[derive(Component, Debug, Clone)]
pub struct BottomNavBar;

/// Character timer for recording sessions
#[derive(Component, Debug, Clone)]
pub struct CharacterTimer {
    pub timer: Timer,
    pub is_recording: bool,
}

impl CharacterTimer {
    /// Create a new character timer (starts paused)
    pub fn new() -> Self {
        let mut timer = Timer::new(Duration::from_secs(120), TimerMode::Once); // 2 minutes to match ArenaTimer
        timer.pause();
        Self {
            timer,
            is_recording: false,
        }
    }

    /// Start recording
    pub fn start_recording(&mut self) {
        self.timer.reset();
        self.timer.unpause();
        self.is_recording = true;
    }

    /// Stop recording
    pub fn stop_recording(&mut self) {
        self.timer.pause();
        self.is_recording = false;
    }
}

/// Action event for recording character actions
#[derive(Debug, Clone)]
pub enum ActionEvent {
    /// Character position at a specific time
    Position { x: f32, y: f32, timestamp: f64 },
    /// Movement action
    Move {
        direction: MovementDirection,
        timestamp: f64,
    },
}

/// A single recording session containing actions and metadata
#[derive(Debug, Clone)]
pub struct RecordingSession {
    pub actions: Vec<ActionEvent>,
    pub arena_index: u8,
    pub session_start_time: f64,
    pub session_end_time: Option<f64>,
    pub is_saved: bool,
}

impl RecordingSession {
    pub fn new(arena_index: u8, start_time: f64) -> Self {
        Self {
            actions: Vec::new(),
            arena_index,
            session_start_time: start_time,
            session_end_time: None,
            is_saved: false,
        }
    }

    pub fn add_action(&mut self, action: ActionEvent) {
        self.actions.push(action);
    }

    pub fn end_session(&mut self, end_time: f64) {
        self.session_end_time = Some(end_time);
    }

    pub fn is_active(&self) -> bool {
        self.session_end_time.is_none()
    }

    pub fn save_session(&mut self) {
        self.is_saved = true;
    }
}

/// Component to store recorded actions for a character
///
/// ## Recording Behavior:
/// - **Recording Start**: Only starts when an R key is pressed (no automatic recording)
/// - **Recording Stop**: Stops when an R key is pressed again, OR a Tab key switches character
/// - **Draft Recording**: Single working copy that gets cleared on Tab or arena transitions
/// - **Per Arena**: Up to 40 selectable characters
/// - **Total System**: 9 arenas × 40 characters = 360 characters
///
/// ## Memory Usage per Character:
/// - `draft_recording`: Option<RecordingSession> ≈ ~3KB when active
/// - `saved_sessions`: 9 × Option<RecordingSession> for revert capability
/// - Max per character: ~30KB (1 draft + 9 saved sessions)
/// - System total: 360 characters × 30KB = ~10.8MB for full capacity
#[derive(Component, Debug, Clone)]
pub struct RecordedActions {
    /// Single draft recording (working copy) - cleared on Tab or arena transitions
    pub draft_recording: Option<RecordingSession>,
    /// Saved recording sessions for revert capability (one per arena)
    pub saved_sessions: [Option<RecordingSession>; 9],
}

impl Default for RecordedActions {
    fn default() -> Self {
        Self {
            draft_recording: None,
            saved_sessions: [None, None, None, None, None, None, None, None, None],
        }
    }
}

impl RecordedActions {
    /// Start recording with R key - creates new draft recording
    /// **Returns**: `true` if a draft was already in progress (replaced)
    pub fn start_recording(&mut self, arena_index: u8, start_time: f64) -> bool {
        let had_previous_draft = self.draft_recording.is_some();

        // Create a new draft recording (replaces any existing draft)
        self.draft_recording = Some(RecordingSession::new(arena_index, start_time));

        had_previous_draft
    }

    /// Stop the currently active draft recording
    /// **Returns**: `true` if a recording was successfully stopped, `false` if no recording was active
    pub fn stop_recording(&mut self, end_time: f64) -> bool {
        if let Some(ref mut recording) = self.draft_recording {
            recording.end_session(end_time);
            return true;
        }
        false
    }

    /// Add an action to the currently active draft recording
    /// **Returns**: `true` if action was added, `false` if no active recording
    pub fn add_action(&mut self, action: ActionEvent) -> bool {
        if let Some(ref mut recording) = self.draft_recording {
            recording.add_action(action);
            return true;
        }
        false
    }

    /// Clear the draft recording (used for Tab key and arena transitions)
    pub fn clear_draft(&mut self) {
        self.draft_recording = None;
    }

    /// Get the total number of arenas with saved recordings
    pub fn count_recorded_arenas(&self) -> usize {
        self.saved_sessions
            .iter()
            .filter(|recording| recording.is_some())
            .count()
    }

    /// Get all arenas that have saved recordings (returns arena indices)
    pub fn get_recorded_arena_indices(&self) -> Vec<u8> {
        self.saved_sessions
            .iter()
            .enumerate()
            .filter_map(|(idx, recording)| {
                if recording.is_some() {
                    Some(idx as u8)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Save the current draft recording for an arena (for revert capability)
    /// Returns true if a session was saved, false if no active draft exists
    pub fn save_current_session(&mut self, arena_index: u8) -> bool {
        if arena_index < 9 {
            if let Some(ref mut recording) = self.draft_recording {
                if recording.arena_index == arena_index && recording.is_active() {
                    // Mark the recording as saved
                    recording.save_session();
                    // Store a copy in saved_sessions for potential revert
                    self.saved_sessions[arena_index as usize] = Some(recording.clone());
                    return true;
                }
            }
        }
        false
    }

    /// Check if there's a saved session for an arena
    pub fn has_saved_session(&self, arena_index: u8) -> bool {
        if arena_index < 9 {
            self.saved_sessions[arena_index as usize].is_some()
        } else {
            false
        }
    }

    /// Get the saved recording for a specific arena
    pub fn get_saved_recording(&self, arena_index: u8) -> Option<&RecordingSession> {
        if arena_index < 9 {
            self.saved_sessions[arena_index as usize].as_ref()
        } else {
            None
        }
    }
}

/// Component to track playback state for a character
#[derive(Component, Debug, Clone)]
pub struct PlaybackState {
    /// The recording session being played back
    pub recording: RecordingSession,
    /// Current action index being played
    pub current_action_index: usize,
}

impl PlaybackState {
    /// Create a new playback state from a recording session
    pub fn new(recording: RecordingSession) -> Option<Self> {
        // Only create if there are actions to play back
        if recording.actions.is_empty() {
            return None;
        }

        Some(Self {
            recording,
            current_action_index: 0,
        })
    }

    /// Get the next action to execute based on the timer timestamp
    pub fn get_current_action(&self, arena_timer_elapsed: f64) -> Option<&ActionEvent> {
        // Get the current action if we haven't processed all actions yet
        if self.current_action_index >= self.recording.actions.len() {
            return None;
        }

        let action = &self.recording.actions[self.current_action_index];

        // Calculate when this action should occur relative to recording start
        let action_relative_time = match action {
            ActionEvent::Position { timestamp, .. } => {
                *timestamp - self.recording.session_start_time
            }
            ActionEvent::Move { timestamp, .. } => *timestamp - self.recording.session_start_time,
        };

        // Check if it's time to execute this action
        if action_relative_time <= arena_timer_elapsed {
            Some(action)
        } else {
            None
        }
    }

    /// Advance to the next action
    pub fn advance_action(&mut self) {
        if self.current_action_index < self.recording.actions.len() {
            self.current_action_index += 1;
        }
    }

    /// Check if playback is complete
    pub fn is_complete(&self) -> bool {
        self.current_action_index >= self.recording.actions.len()
    }
}
