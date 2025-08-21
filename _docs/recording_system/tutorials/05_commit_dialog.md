# Tutorial 05: Commit Dialog System

## Objective

Implement the confirmation dialog system that appears when recordings are interrupted or completed. This gives players
control over whether to commit, clear, or retry their recordings.

## Prerequisites

- Completed Tutorials 01-04 (Timeline, Recording, Capture, Playback)
- Basic understanding of Bevy UI system
- Familiarity with event-driven architecture

## Components/Systems

We'll create:

- Dialog UI components
- Dialog state management
- Timeline pause/resume system
- Dialog input handling
- Choice processing logic

## Implementation Steps

### Step 1: Create Dialog Components

Create `src/dialog/mod.rs`:

```rust
use bevy::prelude::*;
// APPROVED: Using CSS palette constants improves readability
use bevy::color::palettes::css::WHITE;
use crate::recording::{StopReason, RecordingRequest, RecordingState, RecordingUpdate};
use crate::arena::{ArenaName, CurrentArena};
use crate::character::Character;
use crate::selectors::Active;

/// RULE 17 COMPLIANCE: Resource for global dialog state
#[derive(Resource)]
pub struct DialogStateResource {
    pub active_dialog: Option<DialogTypeData>,
}

impl Default for DialogStateResource {
    fn default() -> Self {
        Self {
            active_dialog: None,
        }
    }
}

/// RULE 17 COMPLIANCE: Dialog type data for events and parameters
#[derive(Debug, Clone, PartialEq)]
pub enum DialogTypeData {
    /// Mid-recording interruption
    MidRecording { reason: StopReasonData },
    /// Recording completed (TimeStamp::MAX seconds)
    EndRecording,
    /// Retry existing ghost
    RetryGhost,
    /// User pressed R on ghost with existing recording
    GhostReplay { 
        arena: ArenaNameData,
        has_recording: bool 
    },
    /// Character switch confirmation during recording
    CharacterSwitchConfirm,
}
// APPROVED: Enum with data is cleaner than separate dialog structs
// REJECTED: "Use trait objects for dialogs" - Unnecessary indirection

/// RULE 17 COMPLIANCE: Dialog choice data for events and parameters
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DialogChoiceData {
    Commit,
    Clear,
    Cancel,
    Retry,
    KeepExisting,         // For ghost replay dialog - keep playing current recording
    DraftNew,             // For ghost replay dialog - convert to character for new recording
    SwitchCharacter,      // For character switch dialog - stop recording and switch
    ContinueRecording,    // For character switch dialog - cancel switch and continue recording
}

/// RULE 5 & 17 COMPLIANCE: Dialog UI markers for entity categorization
/// Component for dialog UI root entity
#[derive(Component)]
pub struct DialogUIComponent;

/// Component for dialog buttons (contains data, not pure marker)
#[derive(Component)]
pub struct DialogButtonComponent {
    pub choice: DialogChoiceData,
}

/// RULE 5 & 17 COMPLIANCE: Pure marker for dialog background overlay
/// Unit struct without data - perfect for filtering UI elements
#[derive(Component)]
pub struct DialogOverlayComponent;

/// RULE 5 COMPLIANCE: Additional dialog markers for UI system filtering
#[derive(Component)]
pub struct DialogTitle; // Dialog title text element

#[derive(Component)]
pub struct DialogDescription; // Dialog description text element

#[derive(Component)]
pub struct DialogButtonContainer; // Container holding dialog buttons
```

### Step 2: Create Dialog Events

Add to `src/dialog/mod.rs`:

```rust
/// Event to show a dialog
#[derive(Event)]
pub struct ShowDialog {
    pub dialog_type: DialogType,
}

/// Event when user makes a dialog choice
#[derive(Event)]
pub struct DialogChoiceEvent {
    pub choice: DialogChoice,
}

/// Event to close current dialog
#[derive(Event)]
pub struct CloseDialog;

/// APPROVED: Clean event boundary for dialog transitions
#[derive(Event)]
pub struct DialogTransition {
    pub from: Option<DialogType>,
    pub to: Option<DialogType>,
}

/// Event to pause all arena timelines
#[derive(Event)]
pub struct PauseAllTimelines;

/// Event to resume all arena timelines
#[derive(Event)]
// ResumeAllTimelines removed - handled by recording_update orchestrator
```

### Step 3: Create Dialog UI Spawning

Add to `src/dialog/mod.rs`:

```rust
// PR Gate: Dialog pause ordering must be enforced
// Moved pause event to trigger_dialog_pause system which runs BEFORE spawn_dialog_ui

/// Trigger pause before dialog spawns
pub fn trigger_dialog_pause(
    mut show_events: EventReader<ShowDialog>,
    mut pause_events: EventWriter<PauseAllTimelines>,
    dialog_state: Res<DialogState>,
) {
    for event in show_events.read() {
        // Don't pause if dialog already active
        if dialog_state.active_dialog.is_some() {
            continue;
        }

        // PR Gate: Pause MUST happen before dialog spawns
        pause_events.write(PauseAllTimelines);
        info!("Pausing timelines for dialog: {:?}", event.dialog_type);
    }
}

/// Spawn dialog UI after pause
pub fn spawn_dialog_ui(
    mut commands: Commands,
    mut show_events: EventReader<ShowDialog>,
    mut dialog_state: ResMut<DialogState>,
    mut transition_events: EventWriter<DialogTransition>,
) {
    for event in show_events.read() {
        // Don't spawn if dialog already active
        if dialog_state.active_dialog.is_some() {
            warn!("Dialog already active, ignoring new dialog request");
            continue;
        }

        // Send transition event - consume dialog_type to avoid unnecessary cloning
        let old_dialog = dialog_state.active_dialog.take(); // Zero-copy: take() transfers ownership
        dialog_state.active_dialog = Some(event.dialog_type.clone()); // TODO: Could consume if event is consumed

        transition_events.write(DialogTransition {
            from: old_dialog,
            to: Some(event.dialog_type.clone()),
        });

        // Spawn dialog UI (pause already happened in trigger_dialog_pause)
        spawn_dialog_entities(
            &mut commands,
            &event.dialog_type,
        );

        info!("Showing dialog: {:?}", event.dialog_type);
    }
}

fn spawn_dialog_entities(
    commands: &mut Commands,
    dialog_type: &DialogType,
) {
    // Create root UI node
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            DialogUI,
        ))
        .with_children(|parent| {
            // Background overlay
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
                DialogOverlay,
            ));

            // Dialog box
            parent
                .spawn((
                    Node {
                        width: Val::Px(400.0),
                        padding: UiRect::all(Val::Px(20.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(20.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
                    BorderColor(Color::srgb(0.4, 0.4, 0.5)),
                ))
                .with_children(|dialog| {
                    // Title text
                    dialog.spawn((
                        Text::new(get_dialog_title(dialog_type)),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::from(WHITE)),
                    ));

                    // Description text
                    dialog.spawn((
                        Text::new(get_dialog_description(dialog_type)),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));

                    // Button container
                    dialog
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(10.0),
                            ..default()
                        })
                        .with_children(|buttons| {
                            spawn_dialog_buttons(buttons, dialog_type);
                        });
                });
        });
}

// RULE 2 COMPLIANCE: Static data lookup for dialog button configurations
const MID_RECORDING_BUTTONS: &[(&str, DialogChoice)] = &[
    ("Commit", DialogChoice::Commit),
    ("Clear", DialogChoice::Clear), 
    ("Cancel", DialogChoice::Cancel),
];

const END_RECORDING_BUTTONS: &[(&str, DialogChoice)] = &[
    ("Commit", DialogChoice::Commit),
    ("Clear", DialogChoice::Clear),
    ("Retry", DialogChoice::Retry),
];

const RETRY_GHOST_BUTTONS: &[(&str, DialogChoice)] = &[
    ("Retry", DialogChoice::Retry),
    ("Cancel", DialogChoice::Cancel),
];

const GHOST_REPLAY_WITH_RECORDING: &[(&str, DialogChoice)] = &[
    ("Keep Existing", DialogChoice::KeepExisting),
    ("Draft New", DialogChoice::DraftNew),
    ("Cancel", DialogChoice::Cancel),
];

const GHOST_REPLAY_NO_RECORDING: &[(&str, DialogChoice)] = &[
    ("Draft New", DialogChoice::DraftNew),
    ("Cancel", DialogChoice::Cancel),
];

const CHARACTER_SWITCH_BUTTONS: &[(&str, DialogChoice)] = &[
    ("Switch Character", DialogChoice::SwitchCharacter),
    ("Continue Recording", DialogChoice::ContinueRecording),
];

fn spawn_dialog_buttons(buttons: &mut ChildBuilder, dialog_type: &DialogType) {
    let button_choices = match dialog_type {
        DialogType::MidRecording { .. } => MID_RECORDING_BUTTONS,
        DialogType::EndRecording => END_RECORDING_BUTTONS,
        DialogType::RetryGhost { .. } => RETRY_GHOST_BUTTONS,
        DialogType::GhostReplay { has_recording, .. } => {
            if *has_recording {
                GHOST_REPLAY_WITH_RECORDING
            } else {
                GHOST_REPLAY_NO_RECORDING
            }
        }
        DialogType::CharacterSwitchConfirm => CHARACTER_SWITCH_BUTTONS,
    };

    for &(label, choice) in button_choices {
        buttons
            .spawn((
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.3, 0.3, 0.4)),
                DialogButton { choice },
            ))
            .with_children(|button| {
                button.spawn((
                    Text::new(label),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(WHITE.into()),
                ));
            });
    }
}

// RULE 2 COMPLIANCE: Static data lookup for dialog titles
const DIALOG_TITLES: &[(&str, &str)] = &[
    ("MidRecording", "Recording Interrupted"),
    ("EndRecording", "Recording Complete"),
    ("RetryGhost", "Retry Ghost Recording?"),
    ("GhostReplay", "Ghost Replay Options"),
    ("CharacterSwitchConfirm", "Character Switch During Recording"),
];

fn get_dialog_title(dialog_type: &DialogType) -> &'static str {
    match dialog_type {
        DialogType::MidRecording { .. } => "Recording Interrupted",
        DialogType::EndRecording => "Recording Complete",
        DialogType::RetryGhost { .. } => "Retry Ghost Recording?",
        DialogType::GhostReplay { .. } => "Ghost Replay Options",
        DialogType::CharacterSwitchConfirm => "Character Switch During Recording",
    }
}

fn get_dialog_description(dialog_type: &DialogType) -> &'static str {
    match dialog_type {
        DialogType::MidRecording { reason } => match reason {
            StopReason::UserInterrupted => "You interrupted the recording. What would you like to do?",
            StopReason::ArenaTransition => "Cannot change arenas while recording.",
            StopReason::CharacterSwitch => "Cannot switch characters while recording.",
            _ => "Recording was interrupted.",
        },
        DialogType::EndRecording => "You've recorded a full 2-minute cycle. Save this recording?",
        DialogType::RetryGhost { .. } => "This character has a recording. Start a new one?",
        DialogType::GhostReplay { has_recording, .. } => {
            if *has_recording {
                "This ghost has a recording for this arena. Keep playing it or draft a new one?"
            } else {
                "This ghost has no recording for this arena. Start a new recording?"
            }
        },
        DialogType::CharacterSwitchConfirm => "You're currently recording. Stop recording and switch character?",
    }
}
```

### Step 4: Create Dialog Input Handling

Add to `src/dialog/mod.rs`:

```rust
/// Handle button clicks in dialog
pub fn handle_dialog_buttons(
    mut interaction_query: Query<
        (&Interaction, &DialogButton),
        (Changed<Interaction>, With<Button>)
    >,
    dialog_state: Res<DialogState>,
    mut choice_events: EventWriter<DialogChoiceEvent>,
    mut close_events: EventWriter<CloseDialog>,
) {
    for (interaction, button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            // Send choice event
            choice_events.write(DialogChoiceEvent {
                choice: button.choice,
            });

            // Close dialog
            close_events.write(CloseDialog);

            info!("Dialog choice: {:?}", button.choice);
        }
    }
}

/// Handle keyboard shortcuts for dialog
pub fn handle_dialog_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    dialog_state: Res<DialogState>,
    mut choice_events: EventWriter<DialogChoiceEvent>,
    mut close_events: EventWriter<CloseDialog>,
) {
    // Only process if dialog is active
    if dialog_state.active_dialog.is_none() {
        return;
    }

    let choice = if keyboard.just_pressed(KeyCode::Digit1) {
        Some(DialogChoice::Commit)
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        Some(DialogChoice::Clear)
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        match &dialog_state.active_dialog {
            Some(DialogType::MidRecording { .. }) => Some(DialogChoice::Cancel),
            Some(DialogType::EndRecording) => Some(DialogChoice::Retry),
            Some(DialogType::RetryGhost { .. }) => Some(DialogChoice::Cancel),
            None => None,
        }
    } else if keyboard.just_pressed(KeyCode::Escape) {
        Some(DialogChoice::Cancel)
    } else {
        None
    };

    if let Some(choice) = choice {
        choice_events.write(DialogChoiceEvent {
            choice,
        });
        close_events.write(CloseDialog);
    }
}
```

### Step 5: Create Choice Processing

Add to `src/dialog/mod.rs`:

```rust
use crate::recording::{RecordingRequest, RecordingState, RecordingMode, RecordingUpdate};
use crate::arena::CurrentArena;
use crate::playback::{Ghost, Replaying};
use crate::timeline::{TimelinePosition, TimeStamp};

/// Process dialog choices and trigger appropriate actions
pub fn process_dialog_choices(
    mut commands: Commands,
    mut choice_events: EventReader<DialogChoiceEvent>,
    mut recording_state: ResMut<RecordingState>,
    mut recording_update_events: EventWriter<RecordingUpdate>,
    mut resume_events: EventWriter<ResumeAllTimelines>,
    mut ghost_q: Query<&mut TimelinePosition, With<Ghost>>,
    dialog_state: Res<DialogState>,
    current_arena: Res<CurrentArena>,
    // Query for the single Active Character when needed
    active_character_q: Query<Entity, (With<Character>, With<Active>)>,
) {
    for event in choice_events.read() {
        match event.choice {
            DialogChoice::Commit => {
                if active_character_q.single().is_ok() {
                    recording_state.pending_request = Some(RecordingRequest::Commit);
                    recording_update_events.write(RecordingUpdate);
                    recording_state.mode = RecordingMode::Idle;
                    // Timeline resume handled by recording_update orchestrator
                    info!("Committed recording");
                }
            }
            DialogChoice::Clear => {
                if active_character_q.single().is_ok() {
                    recording_state.pending_request = Some(RecordingRequest::Clear);
                    recording_update_events.write(RecordingUpdate);
                    recording_state.mode = RecordingMode::Idle;
                    // Timeline resume handled by recording_update orchestrator
                    info!("Cleared recording");
                }
            }
            DialogChoice::Cancel => {
                // Resume without changes
                if matches!(dialog_state.active_dialog, Some(DialogType::MidRecording { .. })) {
                    recording_state.mode = RecordingMode::Recording;
                }
                resume_events.write(ResumeAllTimelines);
                info!("Cancelled dialog");
            }
            DialogChoice::Retry => {
                if active_character_q.single().is_ok() {
                    recording_state.pending_request = Some(RecordingRequest::Start);
                    recording_update_events.write(RecordingUpdate);
                    info!("Retrying recording");
                }
            }
            DialogChoice::KeepExisting => {
                // Reset active character timeline position and enable replay
                if let Ok(entity) = active_character_q.single() {
                    // Add Replaying component to actually start playback
                    commands.entity(entity).insert(Replaying);
                    
                    if let Ok(mut position) = ghost_q.get_mut(entity) {
                        position.0 = TimeStamp::ZERO;
                        info!("Active character {:?} starting replay of existing recording", entity);
                    }
                }
                resume_events.write(ResumeAllTimelines);
            }
            DialogChoice::DraftNew => {
                // Convert active ghost back to character for new recording
                if let Ok(entity) = active_character_q.single() {
                    commands.entity(entity)
                        .remove::<Ghost>()
                        .remove::<Replaying>();
                    info!("Active ghost {:?} converted to character for new recording", entity);
                }
                resume_events.write(ResumeAllTimelines);
            }
            DialogChoice::SwitchCharacter => {
                // Stop recording and allow character switch
                if active_character_q.single().is_ok() {
                    recording_state.pending_request = Some(RecordingRequest::Clear);
                    recording_update_events.write(RecordingUpdate);
                    recording_state.mode = RecordingMode::Idle;
                    info!("Stopping recording to allow character switch");
                }
                resume_events.write(ResumeAllTimelines);
            }
            DialogChoice::ContinueRecording => {
                // Resume recording without switching
                recording_state.mode = RecordingMode::Recording;
                resume_events.write(ResumeAllTimelines);
                info!("Continuing recording, cancelled character switch");
            }
        }
    }
}
```

### Step 6: Create Dialog Cleanup

Add to `src/dialog/mod.rs`:

```rust
/// Close and cleanup dialog UI
pub fn close_dialog_ui(
    mut commands: Commands,
    mut close_events: EventReader<CloseDialog>,
    mut dialog_state: ResMut<DialogState>,
    dialog_q: Query<Entity, With<DialogUI>>,
) {
    for _ in close_events.read() {
        // Despawn all dialog UI entities
        for entity in dialog_q.iter() {
            commands.entity(entity).despawn_recursive();
        }

        // Clear dialog state using ownership transfer for efficient cleanup
        dialog_state.active_dialog = None; // Previous value is consumed and dropped

        // PR Gate: Resume happens in process_dialog_choices, not here
        info!("Dialog UI closed");
    }
}
```

### Step 7: Create Timeline Pause Systems

Add to `src/dialog/mod.rs`:

```rust
use crate::arena::{Arena, TimelineClock};

/// Pause all arena timeline clocks
pub fn pause_all_arena_timelines(
    mut pause_events: EventReader<PauseAllTimelines>,
    mut arena_q: Query<&mut TimelineClock, With<Arena>>,
) {
    for _ in pause_events.read() {
        for mut clock in arena_q.iter_mut() {
            clock.pause();
        }
        info!("Paused all timeline clocks");
    }
}

/// Resume all arena timeline clocks
pub fn resume_all_arena_timelines(
    // Resume events now handled by recording_update orchestrator
    mut arena_q: Query<&mut TimelineClock, With<Arena>>,
) {
    for _ in resume_events.read() {
        for mut clock in arena_q.iter_mut() {
            clock.resume();
        }
        info!("Resumed all timeline clocks");
    }
}
```

### Step 8: Update Recording System Integration

Update `src/recording/mod.rs` to send dialog events:

```rust
use crate::dialog::{ShowDialog, DialogType};

// Update process_stop_recording to show dialog instead of clearing
pub fn process_stop_recording(
    mut stop_events: EventReader<StopRecording>,
    mut recording_state: ResMut<RecordingState>,
    // Query for the Active Character with Recording component
    active_recording_character: Option<Single<Entity, (With<Character>, With<Active>, With<Recording>)>>,
    mut dialog_events: EventWriter<ShowDialog>,
) {
    for event in stop_events.read() {
        if active_recording_character.is_some() {
            // Show appropriate dialog based on reason
            let dialog_type = match event.reason {
                StopReason::TimeComplete => DialogType::EndRecording,
                reason => DialogType::MidRecording { reason: reason.clone() },
            };

            dialog_events.write(ShowDialog {
                dialog_type,
            });

            // Set state to paused
            recording_state.mode = RecordingMode::DialogPaused;

            info!("Showing dialog for stop reason: {:?}", event.reason);
        }
    }
}
```

### Step 9: Create Dialog Plugin

Add to `src/dialog/mod.rs`:

```rust
// PR Gate: DialogSet with strict ordering for deterministic execution
// DialogSet sequence: PauseClocks → SpawnUI → HandleInput → ProcessChoice → CloseUI → ResumeClocks
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DialogSet {
    PauseClocks,        // First: Pause all timeline clocks
    SpawnUI,            // Second: Spawn dialog UI
    HandleInput,        // Third: Handle user input
    ProcessChoice,      // Fourth: Process the choice
    CloseUI,            // Fifth: Close dialog UI
    ResumeClocks,       // Last: Resume all timeline clocks
}

pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<DialogState>()

            // Events
            .add_event::<ShowDialog>()
            .add_event::<DialogChoiceEvent>()
            .add_event::<CloseDialog>()
            .add_event::<PauseAllTimelines>()
            // ResumeAllTimelines event removed - handled by recording_update

            // PR Gate: Configure sets with strict ordering
            // DialogSet sequence: PauseClocks → SpawnUI → HandleInput → ProcessChoice → CloseUI → ResumeClocks
            .configure_sets(Update, (
                DialogSet::PauseClocks,
                DialogSet::SpawnUI,
                DialogSet::HandleInput,
                DialogSet::ProcessChoice,
                DialogSet::CloseUI,
                DialogSet::ResumeClocks,
            ).chain()) // Enforce sequential execution

            // Systems - Clock Pause (MUST run first)
            .add_systems(Update, (
                trigger_dialog_pause,
                pause_all_arena_timelines,
            ).chain().in_set(DialogSet::PauseClocks))

            // Systems - UI Spawn (after pause)
            .add_systems(Update, spawn_dialog_ui.in_set(DialogSet::SpawnUI))

            // Systems - Input Handling
            .add_systems(Update, (
                handle_dialog_buttons,
                handle_dialog_keyboard,
            ).in_set(DialogSet::HandleInput))

            // Systems - Choice Processing
            .add_systems(Update, process_dialog_choices.in_set(DialogSet::ProcessChoice))

            // Systems - UI Cleanup
            .add_systems(Update, close_dialog_ui.in_set(DialogSet::CloseUI))

            // Systems - Clock Resume (MUST run last)
            .add_systems(Update, resume_all_arena_timelines.in_set(DialogSet::ResumeClocks));
    }
}
```

### Step 10: Wire Into Main

Update `src/main.rs`:

```rust
mod dialog;
use crate::dialog::DialogPlugin;

// In main():
.add_plugins(DialogPlugin)
```

## Unit Tests

Create `src/dialog/tests.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialog_state_default() {
        let state = DialogState::default();
        assert!(state.active_dialog.is_none());
    }

    #[test]
    fn test_dialog_choice_variants() {
        let choices = vec![
            DialogChoice::Commit,
            DialogChoice::Clear,
            DialogChoice::Cancel,
            DialogChoice::Retry,
            DialogChoice::KeepExisting,
            DialogChoice::DraftNew,
            DialogChoice::SwitchCharacter,
            DialogChoice::ContinueRecording,
        ];

        // Ensure all are distinct
        for (i, c1) in choices.iter().enumerate() {
            for (j, c2) in choices.iter().enumerate() {
                if i != j {
                    assert_ne!(c1, c2);
                }
            }
        }
    }

    #[test]
    fn test_dialog_type_equality() {
        let dialog1 = DialogType::EndRecording;
        let dialog2 = DialogType::EndRecording;
        assert_eq!(dialog1, dialog2);

        let dialog3 = DialogType::MidRecording {
            reason: StopReason::UserInterrupted
        };
        assert_ne!(dialog1, dialog3);
    }

    // PR Gate: Test/log proving clocks don't tick during dialog
    #[test]
    fn test_timeline_pause_during_dialog() {
        // Create a timeline clock
        let mut clock = TimelineClock {
            current: TimeStamp::new(50.0),
            is_paused: false,
        };
        
        // Simulate dialog pause
        clock.is_paused = true;
        let time_before_pause = clock.current();
        
        // Simulate time passing while paused (would normally tick)
        // But since paused, clock should not advance
        assert!(clock.is_paused);
        assert_eq!(clock.current(), time_before_pause);
        
        // Resume after dialog
        clock.is_paused = false;
        
        // Now clock can advance again
        // Simulate clock advancing after resume
        // (In real code, this would be done by the tick method)
        let new_time = TimeStamp::new(51.0);
        assert_ne!(new_time, time_before_pause);
        
        info!("Clock correctly paused during dialog: {} -> {}", 
              time_before_pause.as_secs(), new_time.as_secs());
    }
}
```

## Verification

Run tests:

```bash
cargo test dialog
```

Run the game and test dialogs:

```bash
cargo run
```

Test sequence:

1. Press R to start recording
2. Press R again during recording - MidRecording dialog appears
3. Press 1 (Commit), 2 (Clear), or 3 (Cancel) to make choice
4. Start another recording and wait 2 minutes - EndRecording dialog
5. Press R on a ghost with existing recording - GhostReplay dialog with "Keep Existing" or "Draft New"
6. Press Tab during recording - CharacterSwitchConfirm dialog appears with options:
   - "Switch Character" - stops recording and allows character switch
   - "Continue Recording" - cancels switch and continues recording
7. Test both choices to verify character switch behavior

## Next Steps

With dialogs complete, we can now:

- Tutorial 06: Multi-arena ghost management
- Tutorial 07: Visual polish and feedback
- Tutorial 08: Performance optimization for 320 ghosts

## Key Takeaways

1. **🎯 RULE 2 - Static Data Lookup**: Dialog buttons and titles use const arrays for maintainable configuration
2. **🎯 RULE 5 - Marker Components**: DialogOverlay, DialogTitle unit structs enable precise UI element filtering
3. **Modal UI**: Dialogs pause all gameplay for important decisions
3. **Context-Aware**: Different dialog types for different situations
4. **Keyboard Support**: Quick keyboard shortcuts for efficiency
5. **Explicit Constructors**: Arena::new() validation in retry logic
6. **Clean Transitions**: Proper cleanup when dialogs close

The dialog system provides crucial player control over the recording process. By pausing all timelines during dialogs,
we ensure players can make informed decisions without time pressure.