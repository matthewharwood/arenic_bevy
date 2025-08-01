use super::GameState;
use crate::character::CharacterType;
use crate::pseudo_states::Selected;
use crate::ui::{Colors, FontSizes, Spacing};
use bevy::prelude::*;

/// Plugin for the Character Creation state
pub struct CharacterCreatePlugin;

impl Plugin for CharacterCreatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::CharacterCreate), setup_character_create)
            .add_systems(
                Update,
                (
                    character_create_input,
                    character_tile_interaction_system,
                    character_tile_selection_system,
                    character_tile_deselection_system,
                    character_portrait_selection_system,
                    character_portrait_deselection_system,
                    // Unified synchronization system for all character types
                    sync_character_tile_to_portrait,
                    // Character name input field synchronization
                    sync_character_name_input_field,
                    // Start button interaction system
                    start_button_interaction_system,
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

/// Marker component for character tiles that stores preloaded icon handles
#[derive(Component)]
struct CharacterTile {
    normal_icon: Handle<Image>,
    selected_icon: Handle<Image>,
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

fn setup_character_create(mut commands: Commands, asset_server: Res<AssetServer>) {
    let title_font = asset_server.load("fonts/Migra-Extrabold.ttf");

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
                                Selected,
                            ),
                            // Tile 3 - Thief
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
                        (
                            create_character_portrait(CharacterType::Hunter, &asset_server),
                            Selected
                        ),
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
                                Text::new(CharacterType::Hunter.class_name()),
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
                        ..Default::default()
                    },
                    BackgroundColor(Colors::WHITE),
                    BorderColor(Colors::BLACK),
                    BorderRadius::all(Val::Px(12.0))
                )
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
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &CharacterTile,
            Option<&Selected>,
        ),
        (Changed<Interaction>, With<CharacterTile>),
    >,
    mut text_query: Query<&mut TextColor>,
    mut image_query: Query<&mut ImageNode>,
    selected_entity: Single<Entity, (With<Selected>, With<CharacterTile>)>,
) {
    for (
        entity,
        interaction,
        mut bg_color,
        mut border_color,
        children,
        character_tile,
        is_selected,
    ) in &mut query
    {
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
                if is_selected.is_some() {
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
                // Only change selection if clicking on a different tile
                if entity != *selected_entity {
                    // Remove Selected from currently selected tile
                    commands.entity(*selected_entity).remove::<Selected>();

                    // Add Selected to clicked tile
                    commands.entity(entity).insert(Selected);
                }
            }
        }
    }
}

fn character_tile_selection_system(
    selected_tile: Single<
        (
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &CharacterTile,
        ),
        (Added<Selected>, With<CharacterTile>),
    >,
    mut child_query: Query<(Option<&mut TextColor>, Option<&mut ImageNode>)>,
) {
    let (mut bg_color, mut border_color, children, character_tile) = selected_tile.into_inner();
    // Apply selected appearance
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
}

fn character_tile_deselection_system(
    mut removed: RemovedComponents<Selected>,
    mut query: Query<(
        &mut BackgroundColor,
        &mut BorderColor,
        &Children,
        &CharacterTile,
    )>,
    mut child_query: Query<(Option<&mut TextColor>, Option<&mut ImageNode>)>,
) {
    for entity in removed.read() {
        if let Ok((mut bg_color, mut border_color, children, character_tile)) =
            query.get_mut(entity)
        {
            // Reset to normal appearance
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

fn character_portrait_selection_system(
    mut selected_portraits: Query<
        &mut ImageNode,
        (Added<Selected>, With<CharacterType>, Without<CharacterTile>),
    >,
) {
    for mut image_node in &mut selected_portraits {
        image_node.color = Color::srgba(1.0, 1.0, 1.0, 1.0);
    }
}

fn character_portrait_deselection_system(
    mut removed: RemovedComponents<Selected>,
    mut query: Query<&mut ImageNode, (With<CharacterType>, Without<CharacterTile>)>,
) {
    for entity in removed.read() {
        if let Ok(mut image_node) = query.get_mut(entity) {
            image_node.color = Color::srgba(1.0, 1.0, 1.0, 0.0);
        }
    }
}

/// Unified system that synchronizes Selected marker between tiles and portraits using CharacterType
fn sync_character_tile_to_portrait(
    mut commands: Commands,
    // Query for tiles that were just selected
    tiles_selected: Query<&CharacterType, (Added<Selected>, With<CharacterTile>)>,
    // Query for all portraits by character type, excluding tiles
    portraits: Query<(Entity, &CharacterType), (Without<CharacterTile>, With<CharacterType>)>,
    // Get entities that just had Selected removed
    mut removed_selected: RemovedComponents<Selected>,
    // Query to check if a removed entity was a tile with a specific character type
    tile_query: Query<&CharacterType, (With<CharacterTile>, With<CharacterType>)>,
) {
    // Handle Selected being added to tiles
    for &character_type in &tiles_selected {
        // Find the portrait with matching character type
        for (portrait_entity, &portrait_character_type) in &portraits {
            if character_type == portrait_character_type {
                commands.entity(portrait_entity).insert(Selected);
                break; // Only one portrait per character type
            }
        }
    }

    // Handle Selected being removed from tiles
    for removed_entity in removed_selected.read() {
        if let Ok(&character_type) = tile_query.get(removed_entity) {
            // Find the portrait with matching character type and remove Selected
            for (portrait_entity, &portrait_character_type) in &portraits {
                if character_type == portrait_character_type {
                    commands.entity(portrait_entity).remove::<Selected>();
                    break; // Only one portrait per character type
                }
            }
        }
    }
}

/// System that synchronizes the character name input field with the selected character type
fn sync_character_name_input_field(
    // Query for tiles that were just selected
    tiles_selected: Query<&CharacterType, (Added<Selected>, With<CharacterTile>)>,
    // Query for the character name input field
    input_field_query: Query<&Children, With<CharacterNameInputField>>,
    // Query for text components to update
    mut text_query: Query<&mut Text>,
) {
    // Handle Selected being added to tiles - update input field text
    for &character_type in &tiles_selected {
        // Find the input field and update its text child
        if let Ok(children) = input_field_query.single() {
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.0 = character_type.class_name().to_string();
                    break; // Only one text child in the input field
                }
            }
        }
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
