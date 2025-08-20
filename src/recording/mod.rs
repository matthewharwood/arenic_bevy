use bevy::prelude::*;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::time::Duration;

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

#[derive(Debug)]
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

#[derive(Event, Debug)]
pub struct RecordingTransition {
    pub from: RecordingMode,
    pub to: RecordingMode,
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
