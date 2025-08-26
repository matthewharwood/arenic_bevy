use crate::character::{Character, Ghost};
use crate::recording::GlobalRecordingMode;
use crate::recording::components::{CountdownDestination, CountdownStatus, GlobalPauseReason};
use crate::selectors::Active;
use bevy::input::ButtonInput;
use bevy::log::{debug, info};
use bevy::prelude::{Entity, KeyCode, Res, ResMut, Single, Time, With};

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
                // Countdown complete - transition based on destination
                match countdown_state.destination() {
                    CountdownDestination::Recording => {
                        info!("Countdown: GO! Recording started!");
                        *recording_mode = GlobalRecordingMode::Recording;
                    }
                    CountdownDestination::Idle => {
                        info!("Countdown: GO! Returning to idle state!");
                        *recording_mode = GlobalRecordingMode::Idle;
                    }
                }
            }
            CountdownStatus::InProgress => {
                // Continue countdown - nothing to do
            }
        }
    }
}

/// System that shows the commit dialog (only runs when in CommitRequested state)
pub fn show_commit_dialog(
    mut recording_mode: ResMut<GlobalRecordingMode>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // This system will only run when the run condition is true
    info!("Showing commit dialog - recording is paused and waiting for commit");

    // For now, we just log that the dialog would be shown
    if let GlobalRecordingMode::Paused(GlobalPauseReason::CommitRequested) = *recording_mode {
        debug!("Commit dialog is active. Waiting for user input...");

        // Check if 'A' key is pressed to accept and start countdown
        if keyboard.just_pressed(KeyCode::KeyA) {
            *recording_mode = GlobalRecordingMode::start_countdown_to_idle();
            info!("Commit accepted. Starting countdown to return to idle...");
        }
    }
}
pub fn show_ghost_dialog(
    mut recording_mode: ResMut<GlobalRecordingMode>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    info!("Showing ghost dialog - recording is paused and waiting for ghost type");

    if let GlobalRecordingMode::Paused(GlobalPauseReason::GhostType) = *recording_mode {
        debug!("Ghost dialog is active. Waiting for user input...");

        // Check if 'A' key is pressed to accept and start countdown
        if keyboard.just_pressed(KeyCode::KeyA) {
            *recording_mode = GlobalRecordingMode::start_countdown_to_recording();
            info!("Ghost type accepted. Starting countdown to recording...");
        }
    }
}

/// System that handles recording input and state transitions
pub fn handle_recording_input(
    mut recording_mode: ResMut<GlobalRecordingMode>,
    keyboard: Res<ButtonInput<KeyCode>>,
    active_character: Option<Single<(Entity, Option<&Ghost>), (With<Character>, With<Active>)>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        // Check if there's an active character selected (should be exactly one by convention)
        let Some(active_char) = active_character else {
            info!(
                "Cannot start recording: No active character selected. Press Tab to select a character."
            );
            return;
        };

        let (_entity, ghost) = active_char.into_inner();

        match *recording_mode {
            GlobalRecordingMode::Idle => {
                // Check if the active character is a ghost
                if ghost.is_some() {
                    // Pause recording with GhostType reason
                    *recording_mode = GlobalRecordingMode::Paused(GlobalPauseReason::GhostType);
                    info!("Cannot record with a ghost character. Recording paused.");
                } else {
                    // Start countdown - the countdown system will handle all logging
                    *recording_mode = GlobalRecordingMode::start_countdown();
                    info!("Starting countdown before recording...");
                }
            }
            _ => {
                // Default arm - do nothing, return false conceptually
                // (systems don't return values, but we're not changing state)
            }
        }
    }
}
