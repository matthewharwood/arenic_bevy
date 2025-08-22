use bevy::input::ButtonInput;
// Standard library and external crates
use bevy::prelude::{
    App, Component, KeyCode, Plugin, Res, ResMut, Resource, Single, Time, Update, With,
};
use std::time::Duration;

// Local crate modules
use crate::arena::CurrentArena;
use crate::character::Character;
use crate::selectors::Active;
use crate::timeline::{DraftTimeline, TimelineManager};

#[derive(Component)]
pub struct PlaybackMode;

#[derive(Debug, Clone, PartialEq)]
pub enum RecordingMode {
    /// Not recording
    Idle,
    /// Countdown before recording starts
    CountdownRecording,
    /// Countdown playback
    CountdownPlayback,
    /// Actively recording character actions
    Recording,
    /// Dialog shown, all timelines paused
    DialogPaused,
}

impl Default for RecordingMode {
    fn default() -> Self {
        RecordingMode::Idle
    }
}

#[derive(Resource)]
pub struct RecordingState {
    pub mode: RecordingMode,
    pub countdown_time: Duration,
}
impl Default for RecordingState {
    fn default() -> Self {
        Self {
            mode: RecordingMode::Idle,
            countdown_time: Duration::from_secs(3),
        }
    }
}
impl RecordingState {
    /// Start countdown phase before recording
    pub fn start_countdown_recording(&mut self) {
        self.mode = RecordingMode::CountdownRecording;
        self.countdown_time = Duration::from_secs(3); // Reset timer
        println!("⏱️ Starting 3-second countdown...");
    }
    pub fn start_countdown_playback(&mut self) {
        self.mode = RecordingMode::CountdownPlayback;
        self.countdown_time = Duration::from_secs(3); // Reset timer
        println!("⏱️ Starting 3-second countdown...");
    }

    /// Start active recording
    pub fn start_recording(&mut self) {
        self.mode = RecordingMode::Recording;
    }

    /// Pause recording due to dialog
    pub fn pause_for_dialog(&mut self) {
        self.mode = RecordingMode::DialogPaused;
    }

    /// Return to idle state
    pub fn stop_recording(&mut self) {
        self.mode = RecordingMode::Idle;
    }

    /// Check if currently idle
    pub fn is_idle(&self) -> bool {
        self.mode == RecordingMode::Idle
    }

    /// Check if in countdown phase
    pub fn is_countdown_recording(&self) -> bool {
        self.mode == RecordingMode::CountdownRecording
    }
    pub fn is_countdown_playback(&self) -> bool {
        self.mode == RecordingMode::CountdownPlayback
    }

    /// Check if actively recording
    pub fn is_recording(&self) -> bool {
        self.mode == RecordingMode::Recording
    }

    /// Check if paused due to dialog
    pub fn is_dialog_paused(&self) -> bool {
        self.mode == RecordingMode::DialogPaused
    }
}
const KEY_RECORD: KeyCode = KeyCode::KeyR;

pub fn recording_phases(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_arena: Res<CurrentArena>,
    mut current_recording_state: ResMut<RecordingState>,
    mut active_character_q: Single<&mut TimelineManager, (With<Character>, With<Active>)>,
    _draft_timeline: ResMut<DraftTimeline>,
) {
    if (!keyboard.just_pressed(KEY_RECORD)) {
        return;
    }
    if current_recording_state.is_idle() {
        current_recording_state.start_countdown_recording();
        return;
    }
    if current_recording_state.is_countdown_recording() {
        return;
    }
    if current_recording_state.is_recording() {
        current_recording_state.pause_for_dialog();
    }
    //
    // let mut timeline_manager = match active_character_q.into() {
    //     Ok(manager) => manager,
    //     Err(_) => return, // No active character found
    // };
    //
    // // Example: Check if character has a recording for current arena
    // if timeline_manager.has_recording_for(current_arena.0) {
    //     bevy::log::trace!("Character has existing recording for {}", current_arena.0);
    // }
}
pub fn countdown_recording_update(mut recording_state: ResMut<RecordingState>, time: Res<Time>) {
    // Only update if we're in countdown mode
    if recording_state.mode != RecordingMode::CountdownRecording {
        return;
    }

    // Get the delta time since last frame
    let delta = time.delta();

    // Store the previous whole second value for logging
    let prev_whole_seconds = recording_state.countdown_time.as_secs_f32().ceil() as u32;

    // Check if countdown has finished BEFORE subtracting delta
    if recording_state.countdown_time <= delta {
        // Countdown finished - start recording
        recording_state.mode = RecordingMode::Recording;
        println!("🎬 Recording started!");
        return; // Important: return early to prevent further processing
    }

    // Subtract delta time from countdown
    recording_state.countdown_time -= delta;

    // Log countdown progress (only when crossing whole second boundaries)
    let current_whole_seconds = recording_state.countdown_time.as_secs_f32().ceil() as u32;

    if current_whole_seconds != prev_whole_seconds && current_whole_seconds > 0 {
        match current_whole_seconds {
            3 => println!("🔴 Recording in 3..."),
            2 => println!("🟡 Recording in 2..."),
            1 => println!("🟢 Recording in 1..."),
            _ => {}
        }
    }
}

pub fn countdown_playback_update(mut recording_state: ResMut<RecordingState>, time: Res<Time>) {
    // Only update if we're in countdown mode
    if recording_state.mode != RecordingMode::CountdownPlayback {
        return;
    }

    // Get the delta time since last frame
    let delta = time.delta();

    // Store the previous whole second value for logging
    let prev_whole_seconds = recording_state.countdown_time.as_secs_f32().ceil() as u32;

    // Check if countdown has finished BEFORE subtracting delta
    if recording_state.countdown_time <= delta {
        // Countdown finished - start recording
        recording_state.mode = RecordingMode::Idle;
        // Add recording update event.
        println!("🎬 Recording started!");
        return; // Important: return early to prevent further processing
    }

    // Subtract delta time from countdown
    recording_state.countdown_time -= delta;

    // Log countdown progress (only when crossing whole second boundaries)
    let current_whole_seconds = recording_state.countdown_time.as_secs_f32().ceil() as u32;

    if current_whole_seconds != prev_whole_seconds && current_whole_seconds > 0 {
        match current_whole_seconds {
            3 => println!("🔴 Recording in 3..."),
            2 => println!("🟡 Recording in 2..."),
            1 => println!("🟢 Recording in 1..."),
            _ => {}
        }
    }
}
pub struct RecordingPlugin;

impl Plugin for RecordingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RecordingState>().add_systems(
            Update,
            (
                recording_phases,
                countdown_recording_update,
                countdown_playback_update,
            ),
        );
    }
}
