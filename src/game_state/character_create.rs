use super::GameState;
use crate::character::CharacterType;
use crate::ui::styles_config::{Colors, FontSizes, Spacing};
use bevy::prelude::*;
use bevy::window::SystemCursorIcon;
use bevy::winit::cursor::CursorIcon;

/// Event fired when a character is selected
#[derive(Event)]
pub struct CharacterSelectionEvent {
    pub character_type: CharacterType,
    pub previous_character: Option<CharacterType>,
}

/// Resource containing the character creation state
#[derive(Resource)]
pub struct CharacterCreateState {
    pub selected_character: CharacterType,
    pub current_audio_entity: Option<Entity>,
}

impl Default for CharacterCreateState {
    fn default() -> Self {
        Self {
            selected_character: CharacterType::Hunter,
            current_audio_entity: None,
        }
    }
}

/// Plugin for the Character Creation state
pub struct CharacterCreatePlugin;

impl Plugin for CharacterCreatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CharacterSelectionEvent>()
            .init_resource::<CharacterCreateState>()
            .add_systems(
                OnEnter(GameState::CharacterCreate),
                (
                    setup_character_create,
                    initialize_default_selection,
                    setup_cursor_icon,
                    play_default_character_audio,
                ),
            )
            .add_systems(
                Update,
                (
                    character_create_input,
                    character_tile_interaction_system,
                    handle_character_selection_events,
                    update_tile_visuals,
                    update_portrait_visuals,
                    update_character_name_input,
                    update_character_ability_pane,
                    start_button_interaction_system,
                    handle_tile_cursor,
                    handle_start_button_cursor,
                    play_character_selection_audio,
                )
                    .run_if(in_state(GameState::CharacterCreate)),
            )
            .add_systems(OnExit(GameState::CharacterCreate), cleanup_character_create);
    }
}

/// Marker component for character creation screen entities
#[derive(Component)]
struct CharacterCreateScreen;

/// Marker component for the character name input field
#[derive(Component)]
struct CharacterNameInputField;

/// Marker component for the start button
#[derive(Component)]
struct StartButton;

/// Marker component for the character ability pane
#[derive(Component)]
struct CharacterAbilityPane;
/// Marker component for character tiles that stores preloaded icon handles
#[derive(Component)]
struct CharacterTile {
    normal_icon: Handle<Image>,
    selected_icon: Handle<Image>,
}

fn play_default_character_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<CharacterCreateState>,
) {
    let audio_clips = state.selected_character.audio();
    // Play the first audio clip (greeting) for the default selected character
    let (audio_path, _caption) = audio_clips[0];
    let audio_entity = commands
        .spawn(AudioPlayer::new(asset_server.load(audio_path)))
        .id();

    // Track the initial audio entity
    state.current_audio_entity = Some(audio_entity);
}
/// Creates a character tile with icon and class name for the specified character type
fn create_character_tile(
    character_type: CharacterType,
    asset_server: &AssetServer,
    title_font: Handle<Font>,
) -> impl Bundle {
    let icons = character_type.icon();
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            border: UiRect::all(Val::Px(6.0)),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::axes(Val::Px(24.0), Val::Px(16.0)),
            ..Default::default()
        },
        BackgroundColor(Colors::WHITE),
        BorderColor(Colors::BLACK),
        BorderRadius::all(Val::Px(12.0)),
        Interaction::default(),
        CharacterTile {
            normal_icon: asset_server.load(icons.0),
            selected_icon: asset_server.load(icons.1),
        },
        children![
            (
                ImageNode::new(asset_server.load(icons.0)),
                Node {
                    width: Val::Px(48.0),
                    height: Val::Px(48.0),
                    ..Default::default()
                },
            ),
            (
                Text::new(character_type.class_name()),
                TextFont {
                    font: title_font,
                    font_size: FontSizes::XXL,
                    ..Default::default()
                },
                TextColor(Color::BLACK),
                TextLayout::new_with_justify(JustifyText::Center),
            )
        ],
    )
}

/// Creates a character portrait bundle for the specified character type
fn create_character_portrait(
    character_type: CharacterType,
    asset_server: &AssetServer,
) -> impl Bundle {
    (
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            width: Val::Auto,
            height: Val::Auto,
            ..Default::default()
        },
        ImageNode::new(asset_server.load(character_type.portrait()))
            .with_color(Color::srgba(1.0, 1.0, 1.0, 0.0)),
        character_type,
    )
}

/// Fires initial selection event to set up default state
fn initialize_default_selection(mut selection_events: EventWriter<CharacterSelectionEvent>) {
    selection_events.write(CharacterSelectionEvent {
        character_type: CharacterType::Hunter,
        previous_character: None,
    });
}

/// Sets up the cursor icon component on the primary window at startup
fn setup_cursor_icon(mut commands: Commands, windows: Query<Entity, With<Window>>) {
    if let Ok(window_entity) = windows.single() {
        commands
            .entity(window_entity)
            .insert(CursorIcon::System(SystemCursorIcon::Default));
    }
}

fn setup_character_create(mut commands: Commands, asset_server: Res<AssetServer>) {
    let title_font = asset_server.load("fonts/Migra-Extrabold.ttf");
    let title_font_sans_regular = asset_server.load("fonts/BeVietnamPro-Regular.ttf");
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Spacing::XL),
            ..Default::default()
        },
        BackgroundColor(Colors::WHITE),
        CharacterCreateScreen,
        children![(
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::flex(12, 1.0),
                grid_template_rows: RepeatedGridTrack::flex(14, 1.0),
                row_gap: Val::Px(12.0),
                column_gap: Val::Px(12.0),
                position_type: PositionType::Relative,
                ..Default::default()
            },
            BackgroundColor(Colors::WHITE),
            children![
                (
                    Text::new("Choose Your Class"),
                    TextFont {
                        font: title_font.clone(),
                        font_size: 58.0,
                        ..default()
                    },
                    TextColor(Color::BLACK),
                    Node {
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    }
                ),
                (
                    Node {
                        position_type: PositionType::Relative,
                        grid_row: GridPlacement::start_span(3, 12),
                        grid_column: GridPlacement::span(4),
                        height: Val::Percent(100.0),
                        ..Default::default()
                    },
                    BackgroundColor(Colors::WHITE),
                    children![(
                        Node {
                            display: Display::Grid,
                            grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
                            grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                            column_gap: Val::Px(12.0),
                            row_gap: Val::Px(12.0),
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..Default::default()
                        },
                        children![
                            // Tile 1 - Warrior
                            (
                                create_character_tile(
                                    CharacterType::Warrior,
                                    &asset_server,
                                    title_font.clone()
                                ),
                                CharacterType::Warrior,
                            ),
                            // Tile 2 - Hunter (default selected)
                            (
                                create_character_tile(
                                    CharacterType::Hunter,
                                    &asset_server,
                                    title_font.clone()
                                ),
                                CharacterType::Hunter,
                            ),
                            (
                                create_character_tile(
                                    CharacterType::Thief,
                                    &asset_server,
                                    title_font.clone()
                                ),
                                CharacterType::Thief,
                            ),
                            // Tile 4 - Alchemist
                            (
                                create_character_tile(
                                    CharacterType::Alchemist,
                                    &asset_server,
                                    title_font.clone()
                                ),
                                CharacterType::Alchemist,
                            ),
                            // Tile 5 - Bard
                            (
                                create_character_tile(
                                    CharacterType::Bard,
                                    &asset_server,
                                    title_font.clone()
                                ),
                                CharacterType::Bard,
                            ),
                            // Tile 6 - Cardinal
                            (
                                create_character_tile(
                                    CharacterType::Cardinal,
                                    &asset_server,
                                    title_font.clone()
                                ),
                                CharacterType::Cardinal,
                            ),
                            // Tile 7 - Forager
                            (
                                create_character_tile(
                                    CharacterType::Forager,
                                    &asset_server,
                                    title_font.clone()
                                ),
                                CharacterType::Forager,
                            ),
                            // Tile 8 - Merchant
                            (
                                create_character_tile(
                                    CharacterType::Merchant,
                                    &asset_server,
                                    title_font.clone()
                                ),
                                CharacterType::Merchant,
                            ),
                        ]
                    )],
                ),
                (
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(50.0),
                        margin: UiRect::left(Val::Px(-250.0)),
                        width: Val::Auto,
                        height: Val::Auto,
                        ..Default::default()
                    },
                    children![
                        (create_character_portrait(
                            CharacterType::Warrior,
                            &asset_server
                        ),),
                        (create_character_portrait(
                            CharacterType::Thief,
                            &asset_server
                        ),),
                        (create_character_portrait(
                            CharacterType::Hunter,
                            &asset_server
                        ),),
                        (create_character_portrait(
                            CharacterType::Alchemist,
                            &asset_server
                        ),),
                        (create_character_portrait(
                            CharacterType::Bard,
                            &asset_server
                        ),),
                        (create_character_portrait(
                            CharacterType::Cardinal,
                            &asset_server
                        ),),
                        (create_character_portrait(
                            CharacterType::Forager,
                            &asset_server
                        ),),
                        (create_character_portrait(
                            CharacterType::Merchant,
                            &asset_server
                        ),),
                    ],
                ),
                // Container for CharacterNameInputField (left) and StartButton (right)
                (
                    Node {
                        position_type: PositionType::Relative,
                        grid_row: GridPlacement::start_end(13, -1),
                        grid_column: GridPlacement::start_end(5, -1),
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0), // Adjusted to fit both components
                        display: Display::Flex,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    children![
                        (
                            Node {
                                width: Val::Px(398.0), // CharacterNameInputField
                                height: Val::Percent(100.0),
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(6.0)),
                                ..Default::default()
                            },
                            CharacterNameInputField,
                            BackgroundColor(Colors::WHITE),
                            BorderColor(Colors::BLACK),
                            BorderRadius::all(Val::Px(12.0)),
                            children![(
                                Text::new(CharacterType::Hunter.default_name()),
                                TextFont {
                                    font: title_font.clone(),
                                    font_size: FontSizes::XXL,
                                    ..Default::default()
                                },
                                TextColor(Colors::BLACK),
                                TextLayout::new_with_justify(JustifyText::Center),
                            )]
                        ),
                        (
                            Node {
                                width: Val::Px(220.0), // Approx. 20% / 2 sub-columns
                                height: Val::Percent(100.0),
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            StartButton,
                            BackgroundColor(Colors::BLACK),
                            BorderRadius::all(Val::Px(4.0)),
                            Interaction::default(),
                            children![(
                                Text::new("Start"),
                                TextFont {
                                    font: title_font.clone(),
                                    font_size: 24.0,
                                    ..Default::default()
                                },
                                TextColor(Colors::WHITE),
                                TextLayout::new_with_justify(JustifyText::Center),
                            )]
                        )
                    ]
                ),
                (
                    Node {
                        position_type: PositionType::Relative,
                        grid_row: GridPlacement::start_span(3, 4),
                        grid_column: GridPlacement::start_end(9, -1),
                        border: UiRect::all(Val::Px(6.0)),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Spacing::MD),
                        ..Default::default()
                    },
                    BackgroundColor(Colors::WHITE),
                    BorderColor(Colors::BLACK),
                    BorderRadius::all(Val::Px(12.0)),
                    CharacterAbilityPane,
                    children![
                        (
                            Node {
                                padding: UiRect::bottom(Val::Px(12.0)),
                                ..Default::default()
                            },
                            Text::new(format!("{} Skills", CharacterType::Hunter.class_name())),
                            TextFont {
                                font: title_font.clone(),
                                font_size: 32.0,
                                ..default()
                            },
                            TextColor(Color::BLACK),
                        ),
                        (
                            Text::new({
                                let (ability_name, ability_description) =
                                    CharacterType::Hunter.ability_1();
                                format!("{}: {}", ability_name, ability_description)
                            }),
                            TextFont {
                                font: title_font_sans_regular.clone(),
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::BLACK),
                        )
                    ]
                ),
            ]
        )],
    ));
}

/// Handle navigation to next state via Space key or StartButton click
fn character_create_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    start_button_query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
) {
    // Check for Space key press OR StartButton click
    let should_advance = keyboard.just_pressed(KeyCode::Space)
        || start_button_query
            .iter()
            .any(|&interaction| interaction == Interaction::Pressed);

    if should_advance {
        next_state.set(GameState::Intro);
    }
}

/// StartButton interaction system - handles hover and click visual feedback
fn start_button_interaction_system(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<StartButton>),
    >,
    mut text_query: Query<&mut TextColor>,
) {
    for (interaction, mut bg_color, children) in &mut query {
        match *interaction {
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Colors::PRIMARY);
                // Update text color for better contrast
                for child in children.iter() {
                    if let Ok(mut text_color) = text_query.get_mut(child) {
                        *text_color = TextColor(Colors::WHITE);
                    }
                }
            }
            Interaction::None => {
                // Reset to default appearance
                *bg_color = BackgroundColor(Colors::BLACK);
                for child in children.iter() {
                    if let Ok(mut text_color) = text_query.get_mut(child) {
                        *text_color = TextColor(Colors::WHITE);
                    }
                }
            }
            Interaction::Pressed => {
                // Visual feedback on press (handled by character_create_input for navigation)
                *bg_color = BackgroundColor(Colors::PRIMARY_ACTIVE);
                for child in children.iter() {
                    if let Ok(mut text_color) = text_query.get_mut(child) {
                        *text_color = TextColor(Colors::WHITE);
                    }
                }
            }
        }
    }
}

/// Combined hover and click system for character tiles - processes all interaction states in one query
fn character_tile_interaction_system(
    mut query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &CharacterTile,
            &CharacterType,
        ),
        (Changed<Interaction>, With<CharacterTile>),
    >,
    mut text_query: Query<&mut TextColor>,
    mut image_query: Query<&mut ImageNode>,
    state: Res<CharacterCreateState>,
    mut selection_events: EventWriter<CharacterSelectionEvent>,
) {
    for (interaction, mut bg_color, mut border_color, children, character_tile, character_type) in
        &mut query
    {
        let is_selected = *character_type == state.selected_character;

        match *interaction {
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Colors::PRIMARY_HOVER);
                *border_color = BorderColor(Colors::PRIMARY);

                // Update text color and icon
                for child in children.iter() {
                    if let Ok(mut text_color) = text_query.get_mut(child) {
                        *text_color = TextColor(Colors::PRIMARY);
                    }
                    if let Ok(mut image_node) = image_query.get_mut(child) {
                        image_node.image = character_tile.selected_icon.clone();
                    }
                }
            }
            Interaction::None => {
                if is_selected {
                    // Keep selected appearance
                    *bg_color = BackgroundColor(Colors::PRIMARY_HOVER);
                    *border_color = BorderColor(Colors::PRIMARY);

                    for child in children.iter() {
                        if let Ok(mut text_color) = text_query.get_mut(child) {
                            *text_color = TextColor(Colors::PRIMARY);
                        }
                        if let Ok(mut image_node) = image_query.get_mut(child) {
                            image_node.image = character_tile.selected_icon.clone();
                        }
                    }
                } else {
                    // Reset to normal appearance
                    *bg_color = BackgroundColor(Colors::WHITE);
                    *border_color = BorderColor(Colors::BLACK);
                    for child in children.iter() {
                        if let Ok(mut text_color) = text_query.get_mut(child) {
                            *text_color = TextColor(Color::BLACK);
                        }
                        if let Ok(mut image_node) = image_query.get_mut(child) {
                            image_node.image = character_tile.normal_icon.clone();
                        }
                    }
                }
            }
            Interaction::Pressed => {
                // Only fire event if selecting a different character
                if *character_type != state.selected_character {
                    selection_events.write(CharacterSelectionEvent {
                        character_type: *character_type,
                        previous_character: Some(state.selected_character),
                    });
                }
            }
        }
    }
}

/// Central event handler that updates the SelectedCharacter resource
fn handle_character_selection_events(
    mut selection_events: EventReader<CharacterSelectionEvent>,
    mut state: ResMut<CharacterCreateState>,
) {
    for event in selection_events.read() {
        state.selected_character = event.character_type;
    }
}

/// Updates tile visuals when selection changes
fn update_tile_visuals(
    mut selection_events: EventReader<CharacterSelectionEvent>,
    mut tile_query: Query<(
        &mut BackgroundColor,
        &mut BorderColor,
        &Children,
        &CharacterTile,
        &CharacterType,
    )>,
    mut child_query: Query<(Option<&mut TextColor>, Option<&mut ImageNode>)>,
) {
    for event in selection_events.read() {
        for (mut bg_color, mut border_color, children, character_tile, character_type) in
            &mut tile_query
        {
            if *character_type == event.character_type {
                // Apply selected appearance to new selection
                *bg_color = BackgroundColor(Colors::PRIMARY_HOVER);
                *border_color = BorderColor(Colors::PRIMARY);
                for child in children.iter() {
                    if let Ok((text_color, image_node)) = child_query.get_mut(child) {
                        if let Some(mut text_color) = text_color {
                            *text_color = TextColor(Colors::PRIMARY);
                        }
                        if let Some(mut image_node) = image_node {
                            image_node.image = character_tile.selected_icon.clone();
                        }
                    }
                }
            } else if let Some(previous) = event.previous_character {
                if *character_type == previous {
                    // Reset previous selection to normal appearance
                    *bg_color = BackgroundColor(Colors::WHITE);
                    *border_color = BorderColor(Colors::BLACK);
                    for child in children.iter() {
                        if let Ok((text_color, image_node)) = child_query.get_mut(child) {
                            if let Some(mut text_color) = text_color {
                                *text_color = TextColor(Color::BLACK);
                            }
                            if let Some(mut image_node) = image_node {
                                image_node.image = character_tile.normal_icon.clone();
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Updates portrait visuals when selection changes
fn update_portrait_visuals(
    mut selection_events: EventReader<CharacterSelectionEvent>,
    mut portrait_query: Query<
        (&mut ImageNode, &CharacterType),
        (With<CharacterType>, Without<CharacterTile>),
    >,
) {
    for event in selection_events.read() {
        for (mut image_node, character_type) in &mut portrait_query {
            if *character_type == event.character_type {
                // Show selected portrait
                image_node.color = Color::srgba(1.0, 1.0, 1.0, 1.0);
            } else if let Some(previous) = event.previous_character {
                if *character_type == previous {
                    // Hide previous portrait
                    image_node.color = Color::srgba(1.0, 1.0, 1.0, 0.0);
                }
            }
        }
    }
}

/// Updates the character name input field when selection changes
fn update_character_name_input(
    mut selection_events: EventReader<CharacterSelectionEvent>,
    input_field_query: Query<&Children, With<CharacterNameInputField>>,
    mut text_query: Query<&mut Text>,
) {
    for event in selection_events.read() {
        // Find the input field and update its text child
        if let Ok(children) = input_field_query.single() {
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.0 = event.character_type.default_name().to_string();
                    break; // Only one text child in the input field
                }
            }
        }
    }
}

/// Updates the character ability pane when selection changes
fn update_character_ability_pane(
    mut selection_events: EventReader<CharacterSelectionEvent>,
    ability_pane_query: Query<&Children, With<CharacterAbilityPane>>,
    mut text_query: Query<&mut Text>,
) {
    for event in selection_events.read() {
        // Find the ability pane and update its text children
        if let Ok(children) = ability_pane_query.single() {
            let mut child_iter = children.iter();

            // Update title (first child)
            if let Some(title_child) = child_iter.next() {
                if let Ok(mut text) = text_query.get_mut(title_child) {
                    text.0 = format!("{} Skills", event.character_type.class_name());
                }
            }

            // Update ability description (second child)
            if let Some(ability_child) = child_iter.next() {
                if let Ok(mut text) = text_query.get_mut(ability_child) {
                    let ability_data = event.character_type.data();
                    let (ability_name, ability_description) = ability_data.ability_1;
                    text.0 = format!("{}: {}", ability_name, ability_description);
                }
            }
        }
    }
}
fn handle_tile_cursor(
    mut cursor_query: Query<&mut CursorIcon, With<Window>>,
    tile_query: Query<&Interaction, (Changed<Interaction>, With<CharacterTile>)>,
) {
    if let Ok(mut cursor_icon) = cursor_query.single_mut() {
        for interaction in &tile_query {
            let system_cursor = match *interaction {
                Interaction::Hovered => SystemCursorIcon::Pointer,
                Interaction::Pressed => SystemCursorIcon::Pointer,
                Interaction::None => SystemCursorIcon::Default,
            };
            *cursor_icon = CursorIcon::System(system_cursor);
        }
    }
}

fn handle_start_button_cursor(
    mut cursor_query: Query<&mut CursorIcon, With<Window>>,
    start_button_interaction: Single<&Interaction, (Changed<Interaction>, With<StartButton>)>,
) {
    if let Ok(mut cursor_icon) = cursor_query.single_mut() {
        let system_cursor = match start_button_interaction.into_inner() {
            Interaction::Hovered => SystemCursorIcon::Pointer,
            Interaction::Pressed => SystemCursorIcon::Pointer,
            Interaction::None => SystemCursorIcon::Default,
        };
        *cursor_icon = CursorIcon::System(system_cursor);
    }
}

/// Plays audio when character selection changes, ensuring only one plays at a time
fn play_character_selection_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut selection_events: EventReader<CharacterSelectionEvent>,
    mut state: ResMut<CharacterCreateState>,
) {
    for event in selection_events.read() {
        // Cancel previous audio if it exists
        if let Some(previous_entity) = state.current_audio_entity.take() {
            commands.entity(previous_entity).despawn();
        }

        // Play new character audio
        let audio_clips = event.character_type.audio();
        let (audio_path, _caption) = audio_clips[0];
        let audio_entity = commands
            .spawn(AudioPlayer::new(asset_server.load(audio_path)))
            .id();

        // Track the new audio entity
        state.current_audio_entity = Some(audio_entity);
    }
}

fn cleanup_character_create(
    mut commands: Commands,
    query: Query<Entity, With<CharacterCreateScreen>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
