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
use crate::ui::{Colors, FontSizes, Spacing};
use bevy::prelude::*;

/// Plugin for the Character Creation state
pub struct CharacterCreatePlugin;

impl Plugin for CharacterCreatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::CharacterCreate), setup_character_create)
            .add_systems(
                Update,
                character_create_input.run_if(in_state(GameState::CharacterCreate)),
            )
            .add_systems(OnExit(GameState::CharacterCreate), cleanup_character_create);
    }
}

/// Marker component for character creation screen entities
#[derive(Component)]
struct CharacterCreateScreen;

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
        children![
            (
                ImageNode::new(asset_server.load(T::ICON)),
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
                            // Tile 2 - Hunter
                            create_character_tile::<CharacterHunter>(
                                &asset_server,
                                title_font.clone()
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
                        (
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(0.0),
                                width: Val::Auto,
                                height: Val::Auto,
                                ..Default::default()
                            },
                            ImageNode::new(asset_server.load(CharacterWarrior::PORTRAIT)),
                            CharacterWarrior,
                        ),
                        (
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(0.0),
                                width: Val::Auto,
                                height: Val::Auto,
                                ..Default::default()
                            },
                            ImageNode::new(asset_server.load(CharacterThief::PORTRAIT)),
                            CharacterThief,
                        )
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

fn cleanup_character_create(
    mut commands: Commands,
    query: Query<Entity, With<CharacterCreateScreen>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
