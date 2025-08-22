use crate::arena::CurrentArena;
use crate::character::Character;
use crate::recording::{RecordingMode, RecordingState};
use crate::selectors::Active;
use crate::timeline::{DraftTimeline, PublishTimeline, TimelineManager};
use bevy::prelude::*;

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

/// System to handle keyboard input for the dialog
pub fn handle_dialog_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut recording_state: ResMut<RecordingState>,
    dialog_query: Query<Entity, With<DialogUI>>,
    current_arena: Res<CurrentArena>,
    mut active_character_q: Single<&mut TimelineManager, (With<Character>, With<Active>)>,
    mut draft_timeline: ResMut<DraftTimeline>,
) {
    // Only handle input if dialog is open
    if dialog_query.is_empty() {
        return;
    }

    // C key commits the recording
    if keyboard.just_pressed(KeyCode::KeyC) {
        // Get the active character's timeline manager
        let mut timeline_manager = active_character_q.into_inner();

        // Move DraftTimeline out of the resource and convert to PublishTimeline (zero-copy)
        let published_timeline = PublishTimeline::from_draft(std::mem::take(&mut *draft_timeline));

        // Commit the timeline to the current arena
        timeline_manager.set_timeline(current_arena.0, published_timeline);

        let event_count = timeline_manager.event_count_for_arena(current_arena.0);
        println!("🎬 Recording committed to {} arena!", current_arena.0);
        println!(
            "📊 Character now has {} recorded arenas with {} events in {}",
            timeline_manager.arena_count(),
            event_count,
            current_arena.0
        );
        recording_state.is_countdown_playback();
    }
}

fn spawn_dialog(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                width: Val::Px(600.0),
                height: Val::Px(300.0),
                // Center the dialog by offsetting by half its size
                margin: UiRect {
                    left: Val::Px(-300.0), // -width/2
                    top: Val::Px(-150.0),  // -height/2
                    ..default()
                },
                padding: UiRect::all(Val::Px(20.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::WHITE),
            BorderColor(Color::BLACK),
            DialogUI,
        ))
        .with_children(|dialog| {
            // Title
            dialog.spawn((
                Text::new("Like the Recording?"),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));

            // Message
            dialog.spawn((
                Text::new("If you move out of the arena you will cancel"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));

            // Button container
            dialog
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(80.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                })
                .with_children(|buttons| {
                    // Continue Recording Button (inactive)
                    buttons
                        .spawn((
                            Node {
                                width: Val::Px(160.0),
                                height: Val::Px(60.0),
                                border: UiRect::all(Val::Px(2.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::WHITE),
                            BorderColor(Color::BLACK),
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("Continue Recording"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                            ));
                        });

                    // Cancel Recording Button (inactive)
                    buttons
                        .spawn((
                            Node {
                                width: Val::Px(160.0),
                                height: Val::Px(60.0),
                                border: UiRect::all(Val::Px(2.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::WHITE),
                            BorderColor(Color::BLACK),
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("Cancel Recording &\nContinue Action"),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                            ));
                        });

                    // Commit Recording Button (active - responds to C key)
                    buttons
                        .spawn((
                            Node {
                                width: Val::Px(160.0),
                                height: Val::Px(60.0),
                                border: UiRect::all(Val::Px(2.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::WHITE),
                            BorderColor(Color::BLACK),
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("Commit Recording"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::BLACK),
                            ));
                        });
                });
        });
}

pub struct DialogUIPlugin;

impl Plugin for DialogUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_dialog_ui, handle_dialog_input));
    }
}
