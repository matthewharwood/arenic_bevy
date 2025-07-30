//! Character action playback plugin.
//!
//! This plugin handles playback functionality (currently disabled pending new recording system).

use bevy::prelude::*;

pub const ARENA_TIMER_DURATION_SECONDS: f64 = 120.0;
pub const MAX_ARENAS: usize = 9;

/// Core arena timer component using Required Components pattern
///
/// In Bevy 0.16, components can require other components automatically.
/// This eliminates bundle boilerplate and makes entity composition cleaner.
#[derive(Component, Debug, Clone)]
#[require(ArenaIndex)]
pub struct ArenaTimer {
    /// Current position in seconds within the 2-minute cycle (0.0 to 120.0)
    current_seconds: f64,
    /// Current state of the timer
    state: TimerState,
    /// Epoch timestamp - the Bevy time when this timer was last updated
    /// This creates the mathematical bridge between global time and arena time
    last_updated: f64,
}

impl ArenaTimer {
    pub fn new() -> Self {
        Self {
            current_seconds: 0.0,
            state: TimerState::Paused,
            last_updated: 0.0,
        }
    }

    /// Create a new timer for a specific arena
    pub fn for_arena(index: u8) -> (Self, ArenaIndex) {
        (Self::new(), ArenaIndex(index))
    }

    #[inline]
    pub fn current_seconds(&self) -> f64 {
        self.current_seconds
    }

    #[inline]
    pub fn current_milliseconds(&self) -> f64 {
        self.current_seconds * 1000.0
    }

    #[inline]
    pub fn state(&self) -> TimerState {
        self.state
    }

    #[inline]
    pub fn last_updated(&self) -> f64 {
        self.last_updated
    }

    #[inline]
    pub fn is_running(&self) -> bool {
        self.state.is_running()
    }

    #[inline]
    pub fn time_remaining(&self) -> f64 {
        ARENA_TIMER_DURATION_SECONDS - self.current_seconds
    }

    #[inline]
    pub fn progress_normalized(&self) -> f64 {
        self.current_seconds / ARENA_TIMER_DURATION_SECONDS
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArenaIndex(pub u8);

impl ArenaIndex {
    pub fn new(index: u8) -> Result<Self, String> {
        if index < MAX_ARENAS as u8 {
            Ok(Self(index))
        } else {
            Err(format!("Arena index {} is out of bounds", index))
        }
    }
    pub fn get_index(&self) -> u8 {
        self.0
    }
}

impl Default for ArenaIndex {
    fn default() -> Self {
        ArenaIndex(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimerState {
    Running,
    Paused,
}

impl TimerState {
    pub const fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Running => "Running",
            Self::Paused => "Paused",
        }
    }
}

/// Plugin that handles character action playback
pub struct RecordingPlugin;

impl Plugin for RecordingPlugin {
    fn build(&self, _: &mut App) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_recording_constants() {
        assert_eq!(
            ARENA_TIMER_DURATION_SECONDS, 120.0,
            "ARENA_TIMER_DURATION_SECONDS should be 120.0 seconds"
        );
        assert_eq!(MAX_ARENAS, 9, "MAX_ARENAS should be 9");
    }
    #[test]
    fn test_arena_timing_precision_for_1s_under_1ms() {
        const ONE_MS: f64 = 0.001;
        const FRAME_RATE: f64 = 60.0;
        const SECOND: f64 = 1.0;
        let frames_per_second: f64 = SECOND / FRAME_RATE;
        let frames_in_an_arena_cycle: f64 = ARENA_TIMER_DURATION_SECONDS / frames_per_second;
        let reconstructed_duration = frames_in_an_arena_cycle * frames_per_second;
        let absolute_error = (reconstructed_duration - ARENA_TIMER_DURATION_SECONDS).abs();
        // Ensure that the floating point precision is correct (within 1ms).
        assert!(
            absolute_error < ONE_MS,
            "Accumulated timing error ({:.6}ms) exceeded the 1ms tolerance.", // Format string
            absolute_error * 1000.0 // Value to format, converted to milliseconds
        );
    }

    #[test]
    fn test_high_framerate_arena_timing_percision_for_4hr_under_1ms() {
        const ONE_MS: f64 = 0.001;
        const FRAME_RATE: f64 = 240.0;
        const SECOND: f64 = 1.0;
        const SECONDS_IN_HOUR: f64 = 60.0 * 60.0;
        const FOUR_HOURS: f64 = 4.0;
        const FOUR_HOURS_IN_SECONDS: f64 = SECONDS_IN_HOUR * FOUR_HOURS;
        const TOTAL_FRAMES_IN_4_HOURS: f64 = FRAME_RATE * FOUR_HOURS_IN_SECONDS;

        let frames_per_second: f64 = SECOND / FRAME_RATE;

        let mut acc = 0.0;

        for _ in 0..(TOTAL_FRAMES_IN_4_HOURS as i32) {
            acc += frames_per_second;
        }
        let err = (acc - FOUR_HOURS_IN_SECONDS).abs();
        assert!(
            err < ONE_MS,
            "Accumulated timing error ({:.6}ms) exceeded the 1ms tolerance.",
            err * 1000.0
        );
    }
    #[test]
    fn test_timer_state_constants() {
        let states = [TimerState::Running, TimerState::Paused];
        for state in states {
            let is_running = state.is_running();
            let name = state.as_str();

            match state {
                TimerState::Running => {
                    assert!(is_running);
                    assert_eq!(name, "Running");
                }
                TimerState::Paused => {
                    assert!(!is_running);
                    assert_eq!(name, "Paused");
                }
            }
        }
    }
}
