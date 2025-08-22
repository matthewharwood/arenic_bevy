use bevy::prelude::*;
use crate::recording::{RecordingMode, RecordingState};

/// Marker component for the dialog UI
#[derive(Component)]
pub struct DialogUI;

/// System to spawn the dialog UI when entering DialogPaused mode
pub fn spawn_dialog_ui(
    mut commands: Commands,
    recording_state: Res<RecordingState>,
    dialog_query: Query<Entity, With<DialogUI>>,
) {
    match recording_state.mode {
        RecordingMode::DialogPaused => {
            // Only spawn if dialog doesn't already exist
            if dialog_query.is_empty() {
                spawn_dialog(&mut commands);
            }
        }
        _ => {
            // Remove dialog if it exists and we're not in DialogPaused mode
            for entity in dialog_query.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn spawn_dialog(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                width: Val::Px(300.0),
                height: Val::Px(300.0),
                // Center the dialog by offsetting by half its size
                margin: UiRect {
                    left: Val::Px(-150.0), // -width/2
                    top: Val::Px(-150.0),  // -height/2
                    ..default()
                },
                ..default()
            },
            BackgroundColor(Color::WHITE),
            DialogUI,
        ));
}

pub struct DialogUIPlugin;

impl Plugin for DialogUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_dialog_ui);
    }
}