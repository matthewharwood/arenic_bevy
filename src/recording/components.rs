use bevy::prelude::{Component, Resource};
use std::time::Duration;

/// Status returned by countdown tick operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountdownStatus {
    /// Countdown is still in progress
    InProgress,
    /// Countdown has completed
    Complete,
}

/// Component for individual arenas that are in playback mode
#[derive(Component)]
pub struct Playback; // Arena is currently in playback mode

impl GlobalRecordingMode {
    /// Start a new countdown with the default 3-second duration, defaulting to Recording destination
    pub fn start_countdown() -> Self {
        Self::Countdown(CountdownState::new())
    }

    /// Start a countdown that will transition to Recording after completion
    pub fn start_countdown_to_recording() -> Self {
        Self::Countdown(CountdownState::new_with_destination(
            CountdownDestination::Recording,
        ))
    }

    /// Start a countdown that will transition to Idle after completion
    pub fn start_countdown_to_idle() -> Self {
        Self::Countdown(CountdownState::new_with_destination(
            CountdownDestination::Idle,
        ))
    }

    /// Default countdown duration of 3 seconds
    pub const COUNTDOWN_DURATION: Duration = Duration::from_secs(3);

    /// Check if we're in countdown mode and it's completed
    pub fn is_countdown_complete(&self) -> bool {
        matches!(self, Self::Recording)
    }

    /// Reset countdown to initial state (useful for repeating)
    pub fn reset_countdown(&mut self) {
        *self = Self::Countdown(CountdownState::new());
    }
}

/// Reasons why recording might be paused
#[derive(Clone, Debug)]
pub enum GlobalPauseReason {
    CommitRequested,
    GhostType,
}

/// Global recording mode state
#[derive(Resource, Clone, Debug)]
pub enum GlobalRecordingMode {
    Idle,
    Countdown(CountdownState),
    Recording,
    Paused(GlobalPauseReason),
}

impl Default for GlobalRecordingMode {
    fn default() -> Self {
        Self::Idle
    }
}
/// Reasons for interrupting recording
#[derive(Clone, Debug)]
pub enum InterruptionReason {
    GhostType,
    MovementOutOfArena,
    ChangeCharacter,
}

/// Destination after countdown completion
#[derive(Debug, Clone)]
pub enum CountdownDestination {
    Recording,
    Idle,
}

/// Simplified countdown state with clear separation of concerns
#[derive(Debug, Clone)]
pub struct CountdownState {
    remaining: Duration,
    last_displayed_second: u32,
    destination: CountdownDestination,
}

impl CountdownState {
    /// Create a new countdown state with 3 seconds remaining, defaulting to Recording destination
    pub fn new() -> Self {
        Self::new_with_destination(CountdownDestination::Recording)
    }

    /// Create a new countdown state with a specific destination
    pub fn new_with_destination(destination: CountdownDestination) -> Self {
        let initial_seconds = 3;
        Self {
            remaining: Duration::from_secs(initial_seconds as u64),
            last_displayed_second: initial_seconds + 1, // Start at 4 so that 3 gets displayed
            destination,
        }
    }

    /// Get the destination for after countdown completion
    pub fn destination(&self) -> &CountdownDestination {
        &self.destination
    }

    /// Get the current countdown seconds (for display)
    pub fn current_seconds(&self) -> u32 {
        self.remaining.as_secs_f32().ceil() as u32
    }

    /// Check if countdown should display a new number
    pub fn should_display_number(&self) -> Option<u32> {
        let current = self.current_seconds();
        if current != self.last_displayed_second && current > 0 {
            Some(current)
        } else {
            None
        }
    }

    /// Update the countdown and return the current status
    pub fn tick(&mut self, delta: Duration) -> CountdownStatus {
        if let Some(new_remaining) = self.remaining.checked_sub(delta) {
            self.remaining = new_remaining;
            CountdownStatus::InProgress
        } else {
            CountdownStatus::Complete
        }
    }

    /// Mark that we've displayed a countdown number
    pub fn mark_displayed(&mut self, seconds: u32) {
        self.last_displayed_second = seconds;
    }
}
