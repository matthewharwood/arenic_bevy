use super::GameState;
use bevy::prelude::*;
use bevy::winit::cursor::CursorIcon;
use bevy::window::SystemCursorIcon;

/// Plugin for the Title screen state
pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Title), setup_title)
            .add_systems(
                Update,
                (title_input, handle_button_cursor).run_if(in_state(GameState::Title)),
            )
            .add_systems(OnExit(GameState::Title), cleanup_title);
    }
}

/// Marker component for title screen entities
#[derive(Component)]
struct TitleScreen;

/// Marker component for the New Game button
#[derive(Component)]
struct NewGameButton;

fn setup_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load fonts
    let title_font = asset_server.load("fonts/Migra-Extrabold.ttf");
    let button_font = asset_server.load("fonts/Migra-Extralight.ttf");

    // Spawn title screen UI
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::WHITE),
        TitleScreen,
        children![
            // Title text
            (
                Text::new("Arenic"),
                TextFont {
                    font: title_font,
                    font_size: 182.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ),
            // Button container with shadow effect
            (
                Node {
                    margin: UiRect::top(Val::Px(60.0)),
                    position_type: PositionType::Relative,
                    ..default()
                },
                TitleScreen,
                children![
                    // Shadow layer (behind the button)
                    (
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(3.0),
                            top: Val::Px(5.0),
                            padding: UiRect {
                                top: Val::Px(12.0),    // Match button padding
                                bottom: Val::Px(10.0), // Match button padding
                                left: Val::Px(36.0),
                                right: Val::Px(36.0),
                            },
                            border: UiRect::all(Val::Px(1.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.3)),
                        BorderColor(Color::NONE),
                        children![(
                            Text::new("New Game"),
                            TextFont {
                                font: button_font.clone(),
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::NONE), // Invisible text for sizing
                        ),],
                    ),
                    // Actual button (on top)
                    (
                        Button,
                        Node {
                            padding: UiRect {
                                top: Val::Px(12.0),    // Reduced top padding
                                bottom: Val::Px(10.0), // Increased bottom padding to compensate for text baseline
                                left: Val::Px(36.0),
                                right: Val::Px(36.0),
                            },
                            border: UiRect::all(Val::Px(1.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb_u8(0xF0, 0xE9, 0xE9)),
                        BorderColor(Color::BLACK),
                        NewGameButton,
                        TitleScreen,
                        children![(
                            Text::new("New Game"),
                            TextFont {
                                font: button_font,
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::BLACK),
                        ),],
                    ),
                ],
            ),
        ],
    ));
}

fn title_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    button_query: Query<&Interaction, (Changed<Interaction>, With<NewGameButton>)>,
) {
    // Handle Enter key for focused button
    if keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::CharacterCreate);
    }

    // Handle button clicks
    for interaction in &button_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::CharacterCreate);
        }
    }
}


fn handle_button_cursor(
    mut commands: Commands,
    windows: Query<Entity, With<Window>>,
    button_query: Query<&Interaction, (Changed<Interaction>, With<NewGameButton>)>,
) {
    if let Ok(window_entity) = windows.single() {
        for interaction in &button_query {
            match *interaction {
                Interaction::Hovered => {
                    commands.entity(window_entity).insert(
                        CursorIcon::System(SystemCursorIcon::Pointer)
                    );
                }
                Interaction::None => {
                    commands.entity(window_entity).insert(
                        CursorIcon::System(SystemCursorIcon::Default)
                    );
                }
                Interaction::Pressed => {
                    // Keep pointer cursor while pressed
                    commands.entity(window_entity).insert(
                        CursorIcon::System(SystemCursorIcon::Pointer)
                    );
                }
            }
        }
    }
}

fn cleanup_title(mut commands: Commands, query: Query<Entity, With<TitleScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
