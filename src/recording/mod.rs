use bevy::prelude::{Component, Resource};
use std::time::Duration;

/// Component for individual arenas that are in playback mode
#[derive(Component)]
pub struct Playback; // Arena is currently in playback mode

/// Global recording state that affects the entire game
#[derive(Resource, Default)]
pub enum GlobalRecordingMode {
    Paused(PauseReason),
    Recording,
    Countdown(Duration), // Store countdown time remaining
    #[default]
    Idle,
}

impl GlobalRecordingMode {
    /// Start a new countdown with the default 3 second duration
    pub fn start_countdown() -> Self {
        Self::Countdown(Self::COUNTDOWN_DURATION)
    }

    /// Default countdown duration of 3 seconds
    pub const COUNTDOWN_DURATION: Duration = Duration::from_secs(3);

    /// Update the countdown timer and return true if countdown is complete
    pub fn tick_countdown(&mut self, delta: Duration) -> bool {
        match self {
            Self::Countdown(remaining) => {
                // Log the current countdown state
                let seconds_left = remaining.as_secs_f32().ceil() as u32;
                let prev_seconds = (*remaining + delta).as_secs_f32().ceil() as u32;

                // Only log when we cross a second boundary
                if seconds_left != prev_seconds && seconds_left > 0 {
                    bevy::log::info!("Countdown: {}...", seconds_left);
                }

                // Update the remaining time
                if let Some(new_duration) = remaining.checked_sub(delta) {
                    *remaining = new_duration;
                    false // Still counting down
                } else {
                    // Countdown complete!
                    bevy::log::info!("Countdown: GO! Recording started!");
                    *self = Self::Recording;
                    true // Countdown finished
                }
            }
            _ => false, // Not in countdown state
        }
    }

    /// Reset countdown to initial duration (useful for repeating)
    pub fn reset_countdown(&mut self) {
        *self = Self::Countdown(Self::COUNTDOWN_DURATION);
    }
}

/// Reasons why recording might be paused
#[derive(Clone, Debug)]
pub enum PauseReason {
    /// User wants to commit the recording
    CommitRequested,
    /// Recording was interrupted (e.g., by system event, dialog, etc.)
    Interrupted,
    /// User manually paused
    Manual,
}
