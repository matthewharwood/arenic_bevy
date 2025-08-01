use super::GameState;
use crate::character::alchemist::CharacterAlchemist;
use crate::character::bard::CharacterBard;
use crate::character::cardinal::CharacterCardinal;
use crate::character::forager::CharacterForager;
use crate::character::hunter::CharacterHunter;
use crate::character::merchant::CharacterMerchant;
use crate::character::thief::CharacterThief;
use crate::character::warrior::CharacterWarrior;
use crate::character::Character;
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
                )
                    .run_if(in_state(GameState::CharacterCreate)),
            )
            .add_systems(OnExit(GameState::CharacterCreate), cleanup_character_create);
    }
}

/// Marker component for character creation screen entities
#[derive(Component)]
struct CharacterCreateScreen;

/// Marker component for character tiles that stores preloaded icon handles
#[derive(Component)]
struct CharacterTile {
    normal_icon: Handle<Image>,
    selected_icon: Handle<Image>,
}

/// Creates a character tile with icon and class name for any character type that implements Character
fn create_character_tile<T: Character>(
    asset_server: &AssetServer,
    title_font: Handle<Font>,
) -> impl Bundle {
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
            normal_icon: asset_server.load(T::ICON.0),
            selected_icon: asset_server.load(T::ICON.1),
        },
        children![
            (
                ImageNode::new(asset_server.load(T::ICON.0)),
                Node {
                    width: Val::Px(48.0),
                    height: Val::Px(48.0),
                    ..Default::default()
                },
            ),
            (
                Text::new(T::CLASS_NAME),
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

/// Creates a character portrait bundle for any character type that implements Character
fn create_character_portrait<T: Character + Component + Default>(
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
        ImageNode::new(asset_server.load(T::PORTRAIT)),
        T::default(),
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
                            create_character_tile::<CharacterWarrior>(
                                &asset_server,
                                title_font.clone()
                            ),
                            // Tile 2 - Hunter (default selected)
                            (
                                create_character_tile::<CharacterHunter>(
                                    &asset_server,
                                    title_font.clone()
                                ),
                                Selected,
                            ),
                            // Tile 3 - Thief
                            create_character_tile::<CharacterThief>(
                                &asset_server,
                                title_font.clone()
                            ),
                            // Tile 4 - Alchemist
                            create_character_tile::<CharacterAlchemist>(
                                &asset_server,
                                title_font.clone()
                            ),
                            // Tile 5 - Bard
                            create_character_tile::<CharacterBard>(
                                &asset_server,
                                title_font.clone()
                            ),
                            // Tile 6 - Cardinal
                            create_character_tile::<CharacterCardinal>(
                                &asset_server,
                                title_font.clone()
                            ),
                            // Tile 7 - Forager
                            create_character_tile::<CharacterForager>(
                                &asset_server,
                                title_font.clone()
                            ),
                            // Tile 8 - Merchant
                            create_character_tile::<CharacterMerchant>(
                                &asset_server,
                                title_font.clone()
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
                        create_character_portrait::<CharacterWarrior>(&asset_server),
                        create_character_portrait::<CharacterThief>(&asset_server),
                        create_character_portrait::<CharacterHunter>(&asset_server),
                        create_character_portrait::<CharacterAlchemist>(&asset_server),
                        create_character_portrait::<CharacterBard>(&asset_server),
                        create_character_portrait::<CharacterCardinal>(&asset_server),
                        create_character_portrait::<CharacterForager>(&asset_server),
                        create_character_portrait::<CharacterMerchant>(&asset_server),
                    ],
                ),
                (
                    Node {
                        position_type: PositionType::Relative,
                        grid_row: GridPlacement::start_end(13, -1),
                        grid_column: GridPlacement::start_end(5, -1),
                        border: UiRect::all(Val::Px(6.0)),
                        height: Val::Percent(100.0),
                        ..Default::default()
                    },
                    BackgroundColor(Colors::WHITE),
                    BorderColor(Colors::BLACK),
                    BorderRadius::all(Val::Px(12.0))
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

fn character_create_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Intro);
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
                // Remove Selected from currently selected tile
                commands.entity(*selected_entity).remove::<Selected>();

                // Add Selected to clicked tile
                commands.entity(entity).insert(Selected);
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
    mut text_query: Query<&mut TextColor>,
    mut image_query: Query<&mut ImageNode>,
) {
    let (mut bg_color, mut border_color, children, character_tile) = selected_tile.into_inner();
    // Apply selected appearance
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
}

fn character_tile_deselection_system(
    mut removed: RemovedComponents<Selected>,
    mut query: Query<(
        &mut BackgroundColor,
        &mut BorderColor,
        &Children,
        &CharacterTile,
    )>,
    mut text_query: Query<&mut TextColor>,
    mut image_query: Query<&mut ImageNode>,
) {
    for entity in removed.read() {
        if let Ok((mut bg_color, mut border_color, children, character_tile)) =
            query.get_mut(entity)
        {
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
}

fn cleanup_character_create(
    mut commands: Commands,
    query: Query<Entity, With<CharacterCreateScreen>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
