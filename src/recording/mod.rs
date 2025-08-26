mod components;
mod systems;

use crate::recording::components::{GlobalPauseReason, GlobalRecordingMode};
use crate::recording::systems::{
    handle_recording_input, show_commit_dialog, show_ghost_dialog, tick_countdown,
};
use bevy::prelude::*;
pub use components::Playback;

/// Plugin for managing recording state and input
pub struct RecordingPlugin;

impl Plugin for RecordingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalRecordingMode>()
            .add_systems(Update, handle_recording_input)
            .add_systems(Update, show_commit_dialog.run_if(in_commit_requested_state))
            .add_systems(Update, show_ghost_dialog.run_if(in_ghost_requested_state))
            .add_systems(Update, tick_countdown.run_if(in_countdown_state));
    }
}

/// Run condition that checks if we're in the CommitRequested state
pub fn in_commit_requested_state(recording_mode: Res<GlobalRecordingMode>) -> bool {
    matches!(
        *recording_mode,
        GlobalRecordingMode::Paused(GlobalPauseReason::CommitRequested)
    )
}
pub fn in_ghost_requested_state(recording_mode: Res<GlobalRecordingMode>) -> bool {
    matches!(
        *recording_mode,
        GlobalRecordingMode::Paused(GlobalPauseReason::GhostType)
    )
}

/// Run condition that checks if we're in the Countdown state
pub fn in_countdown_state(recording_mode: Res<GlobalRecordingMode>) -> bool {
    matches!(*recording_mode, GlobalRecordingMode::Countdown(_))
}
