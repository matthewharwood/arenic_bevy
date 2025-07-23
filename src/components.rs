//! Game components definitions.
//! 
//! This module contains all the component types used in the ECS architecture.
//! Components represent data that can be attached to entities.

use bevy::prelude::*;
use std::time::Duration;
use crate::movement::MovementDirection;

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
        Self {
            name: name.into(),
        }
    }
}

/// Marker component for the currently selected character
#[derive(Component, Debug, Clone)]
pub struct CharacterSelected;

/// Timer component for each arena
#[derive(Component, Debug, Clone)]
pub struct ArenaTimer {
    pub timer: Timer,
    pub arena: ArenaName,
}

impl ArenaTimer {
    /// Create a new arena timer with default 2-minute duration
    pub fn new(arena: ArenaName) -> Self {
        let mut timer = Timer::new(Duration::from_secs(120), TimerMode::Repeating);
        timer.pause(); // Start paused until a CharacterSelected enters
        Self {
            timer,
            arena,
        }
    }
    
    /// Create a new arena timer with custom duration
    pub fn new_with_duration(arena: ArenaName, duration: Duration) -> Self {
        let mut timer = Timer::new(duration, TimerMode::Repeating);
        timer.pause(); // Start paused until a CharacterSelected enters
        Self {
            timer,
            arena,
        }
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
    /// # Panics
    /// Panics if the index is not in the range 0-8
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
    Move { direction: MovementDirection, timestamp: f64 },
}

/// Recording session version identifier using arena names
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionVersion {
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

impl SessionVersion {
    /// Get the next version in sequence (wraps around after Gala)
    pub fn next(self) -> Self {
        match self {
            SessionVersion::Labyrinth => SessionVersion::GuildHouse,
            SessionVersion::GuildHouse => SessionVersion::Sanctum,
            SessionVersion::Sanctum => SessionVersion::Mountain,
            SessionVersion::Mountain => SessionVersion::Bastion,
            SessionVersion::Bastion => SessionVersion::Pawnshop,
            SessionVersion::Pawnshop => SessionVersion::Crucible,
            SessionVersion::Crucible => SessionVersion::Casino,
            SessionVersion::Casino => SessionVersion::Gala,
            SessionVersion::Gala => SessionVersion::Labyrinth, // Wrap around
        }
    }
    
    /// Get the arena name representation of this version
    pub fn name(&self) -> &'static str {
        match self {
            SessionVersion::Labyrinth => "Labyrinth",
            SessionVersion::GuildHouse => "Guild House",
            SessionVersion::Sanctum => "Sanctum",
            SessionVersion::Mountain => "Mountain",
            SessionVersion::Bastion => "Bastion",
            SessionVersion::Pawnshop => "Pawnshop",
            SessionVersion::Crucible => "Crucible",
            SessionVersion::Casino => "Casino",
            SessionVersion::Gala => "Gala",
        }
    }
    
    /// Convert from ArenaName to SessionVersion
    pub fn from_arena_name(arena_name: ArenaName) -> Self {
        match arena_name {
            ArenaName::Labyrinth => SessionVersion::Labyrinth,
            ArenaName::GuildHouse => SessionVersion::GuildHouse,
            ArenaName::Sanctum => SessionVersion::Sanctum,
            ArenaName::Mountain => SessionVersion::Mountain,
            ArenaName::Bastion => SessionVersion::Bastion,
            ArenaName::Pawnshop => SessionVersion::Pawnshop,
            ArenaName::Crucible => SessionVersion::Crucible,
            ArenaName::Casino => SessionVersion::Casino,
            ArenaName::Gala => SessionVersion::Gala,
        }
    }
    
    /// Convert to ArenaName
    pub fn to_arena_name(self) -> ArenaName {
        match self {
            SessionVersion::Labyrinth => ArenaName::Labyrinth,
            SessionVersion::GuildHouse => ArenaName::GuildHouse,
            SessionVersion::Sanctum => ArenaName::Sanctum,
            SessionVersion::Mountain => ArenaName::Mountain,
            SessionVersion::Bastion => ArenaName::Bastion,
            SessionVersion::Pawnshop => ArenaName::Pawnshop,
            SessionVersion::Crucible => ArenaName::Crucible,
            SessionVersion::Casino => ArenaName::Casino,
            SessionVersion::Gala => ArenaName::Gala,
        }
    }
}

/// A single recording session containing actions and metadata
#[derive(Debug, Clone)]
pub struct RecordingSession {
    pub actions: Vec<ActionEvent>,
    pub arena_index: u8,
    pub session_start_time: f64,
    pub session_end_time: Option<f64>,
    pub version: SessionVersion,
    pub is_saved: bool,
}

impl RecordingSession {
    pub fn new(arena_index: u8, start_time: f64, version: SessionVersion) -> Self {
        Self {
            actions: Vec::new(),
            arena_index,
            session_start_time: start_time,
            session_end_time: None,
            version,
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
    
    pub fn is_saved(&self) -> bool {
        self.is_saved
    }
}

/// Component to store recorded actions for a character
/// 
/// ## Recording Behavior:
/// - **Recording Start**: Only starts when R key is pressed (no automatic recording)
/// - **Recording Stop**: Stops when R key is pressed again OR Tab key switches character
/// - **Per Character**: Up to 9 recordings (1 per arena, replaced when recording again in same arena)
/// - **Per Arena**: Up to 40 selectable characters
/// - **Total System**: 9 arenas × 40 characters = 360 characters
/// - **Total Recordings**: 360 characters × 9 recordings = 3,240 max recordings
/// 
/// ## Memory Usage per Character:
/// - `arena_recordings`: 9 × Option<RecordingSession> ≈ 72 bytes base + recording data
/// - Each RecordingSession contains Vec<ActionEvent> which grows with recorded actions
/// - Typical recording: ~100 actions × ~32 bytes = ~3KB per recording
/// - Max per character: 9 recordings × 3KB = ~27KB per character
/// - System total: 360 characters × 27KB = ~9.7MB for full capacity
#[derive(Component, Debug, Clone)]
pub struct RecordedActions {
    /// One recording per arena (indexed 0-8), each arena can only have one recording
    /// When starting a new recording in an arena, any existing recording for that arena is replaced
    pub arena_recordings: [Option<RecordingSession>; 9],
    /// Index of the currently active arena (if any recording is in progress)
    pub active_arena: Option<u8>,
    /// Current session version for new recordings (cycles A-J)
    pub current_version: SessionVersion,
    /// Saved recording sessions for revert capability (one per arena)
    pub saved_sessions: [Option<RecordingSession>; 9],
}

impl Default for RecordedActions {
    fn default() -> Self {
        Self {
            arena_recordings: [None, None, None, None, None, None, None, None, None],
            active_arena: None,
            current_version: SessionVersion::Labyrinth,
            saved_sessions: [None, None, None, None, None, None, None, None, None],
        }
    }
}

impl RecordedActions {
    /// Start recording with R key
    /// **Returns**: `true` if a previous recording was replaced for this arena
    pub fn start_recording(&mut self, arena_index: u8, start_time: f64) -> bool {
        // Stop any currently active recording first
        if self.is_currently_recording() {
            self.stop_current_recording(start_time);
        }
        
        // Check if we're replacing an existing recording for this arena
        let had_previous_recording = self.has_recording_for_arena(arena_index);
        
        // Create new recording session for this arena (replaces any existing one)
        let new_recording = RecordingSession::new(arena_index, start_time, self.current_version);
        self.arena_recordings[arena_index as usize] = Some(new_recording);
        
        // Advance to next version for future recordings
        self.current_version = self.current_version.next();
        self.active_arena = Some(arena_index);
        
        had_previous_recording
    }
    
    /// Stop the currently active recording
    /// 
    /// **Returns**: `true` if a recording was successfully stopped, `false` if no recording was active
    pub fn stop_recording(&mut self, end_time: f64) -> bool {
        if let Some(active_arena_idx) = self.active_arena {
            if let Some(ref mut recording) = self.arena_recordings[active_arena_idx as usize] {
                recording.end_session(end_time);
                self.active_arena = None;
                return true;
            }
        }
        false
    }
    
    /// Stop the currently active recording (internal helper)
    fn stop_current_recording(&mut self, end_time: f64) {
        if let Some(active_arena_idx) = self.active_arena {
            if let Some(ref mut recording) = self.arena_recordings[active_arena_idx as usize] {
                if recording.is_active() {
                    recording.end_session(end_time);
                }
            }
            self.active_arena = None;
        }
    }
    
    /// Add an action to the currently active recording
    pub fn add_action(&mut self, action: ActionEvent) -> bool {
        if let Some(active_arena_idx) = self.active_arena {
            if let Some(ref mut recording) = self.arena_recordings[active_arena_idx as usize] {
                recording.add_action(action);
                return true;
            }
        }
        false
    }
    
    /// Get the recording for a specific arena (returns None if no recording exists for that arena)
    pub fn get_arena_recording(&self, arena_index: u8) -> Option<&RecordingSession> {
        if arena_index < 9 {
            self.arena_recordings[arena_index as usize].as_ref()
        } else {
            None
        }
    }
    
    /// Get the currently active recording
    pub fn get_current_recording(&self) -> Option<&RecordingSession> {
        if let Some(active_arena_idx) = self.active_arena {
            self.arena_recordings[active_arena_idx as usize].as_ref()
        } else {
            None
        }
    }
    
    /// Get the currently active arena index (if recording)
    pub fn get_current_arena(&self) -> Option<u8> {
        self.active_arena
    }
    
    /// Get the total number of arenas with recordings
    pub fn count_recorded_arenas(&self) -> usize {
        self.arena_recordings.iter().filter(|recording| recording.is_some()).count()
    }
    
    /// Check if an arena has a recording
    pub fn has_recording_for_arena(&self, arena_index: u8) -> bool {
        if arena_index < 9 {
            self.arena_recordings[arena_index as usize].is_some()
        } else {
            false
        }
    }
    
    /// Check if currently recording (any arena)
    pub fn is_currently_recording(&self) -> bool {
        self.active_arena.is_some()
    }
    
    /// Check if currently recording in the specified arena
    pub fn is_recording_in_arena(&self, arena_index: u8) -> bool {
        if let Some(active_arena_idx) = self.active_arena {
            active_arena_idx == arena_index
        } else {
            false
        }
    }
    
    /// Get all arenas that have recordings (returns arena indices)
    pub fn get_recorded_arena_indices(&self) -> Vec<u8> {
        self.arena_recordings
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
    
    /// Save the current recording session for an arena (for revert capability)
    /// Returns true if a session was saved, false if no active session exists
    pub fn save_current_session(&mut self, arena_index: u8) -> bool {
        if arena_index < 9 {
            if let Some(ref mut recording) = self.arena_recordings[arena_index as usize] {
                if recording.is_active() {
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
    
    /// Revert to the saved session for an arena (clears current unsaved recording)
    /// Returns true if reverted to a saved session, false if no saved session exists
    pub fn revert_to_saved_session(&mut self, arena_index: u8) -> bool {
        if arena_index < 9 {
            if let Some(saved_session) = self.saved_sessions[arena_index as usize].clone() {
                // Restore the saved session
                self.arena_recordings[arena_index as usize] = Some(saved_session);
                // Clear active arena if it was this arena
                if self.active_arena == Some(arena_index) {
                    self.active_arena = None;
                }
                return true;
            } else {
                // No saved session, clear the current recording completely
                self.arena_recordings[arena_index as usize] = None;
                // Clear active arena if it was this arena
                if self.active_arena == Some(arena_index) {
                    self.active_arena = None;
                }
                return true; // Successfully reverted to empty state
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
    
    /// Get the current session version for the next recording
    pub fn get_current_version(&self) -> SessionVersion {
        self.current_version
    }
    
    /// Get the version of the current recording in an arena (if any)
    pub fn get_arena_recording_version(&self, arena_index: u8) -> Option<SessionVersion> {
        if arena_index < 9 {
            self.arena_recordings[arena_index as usize].as_ref().map(|r| r.version)
        } else {
            None
        }
    }
}

