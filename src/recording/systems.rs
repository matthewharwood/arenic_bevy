use crate::character::Character;
use crate::recording::components::{CountdownStatus, GlobalPauseReason};
use crate::recording::GlobalRecordingMode;
use crate::selectors::Active;
use bevy::input::ButtonInput;
use bevy::log::{debug, info};
use bevy::prelude::{KeyCode, Res, ResMut, Single, Time, With};

/// System that ticks the countdown and transitions to Recording when complete
pub fn tick_countdown(mut recording_mode: ResMut<GlobalRecordingMode>, time: Res<Time>) {
    if let GlobalRecordingMode::Countdown(countdown_state) = &mut *recording_mode {
        let delta = time.delta();

        // Check if we should display a new countdown number before ticking
        if let Some(seconds) = countdown_state.should_display_number() {
            info!("Countdown: {}...", seconds);
            countdown_state.mark_displayed(seconds);
        }

        // Tick the countdown
        match countdown_state.tick(delta) {
            CountdownStatus::Complete => {
                // Countdown complete - transition to Recording
                info!("Countdown: GO! Recording started!");
                *recording_mode = GlobalRecordingMode::Recording;
            }
            CountdownStatus::InProgress => {
                // Continue countdown - nothing to do
            }
        }
    }
}

/// System that shows the commit dialog (only runs when in CommitRequested state)
pub fn show_commit_dialog(recording_mode: Res<GlobalRecordingMode>) {
    // This system will only run when the run condition is true
    info!("Showing commit dialog - recording is paused and waiting for commit");

    // For now, we just log that the dialog would be shown
    if let GlobalRecordingMode::Paused(GlobalPauseReason::CommitRequested) = *recording_mode {
        debug!("Commit dialog is active. Waiting for user input...");
    }
}

/// System that handles recording input and state transitions
pub fn handle_recording_input(
    mut recording_mode: ResMut<GlobalRecordingMode>,
    keyboard: Res<ButtonInput<KeyCode>>,
    active_character: Option<Single<(), (With<Character>, With<Active>)>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        // Check if there's an active character selected (should be exactly one by convention)
        if active_character.is_none() {
            info!(
                "Cannot start recording: No active character selected. Press Tab to select a character."
            );
            return;
        }

        match *recording_mode {
            GlobalRecordingMode::Idle => {
                // Start countdown - the countdown system will handle all logging
                *recording_mode = GlobalRecordingMode::start_countdown();
                info!("Starting countdown before recording...");
            }
            _ => {
                // Default arm - do nothing, return false conceptually
                // (systems don't return values, but we're not changing state)
            }
        }
    }
}
