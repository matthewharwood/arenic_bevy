use super::GameState;
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

pub enum CharacterClass {
    Hunter,    // Eagle Eye precision targeting
    Bard,      // Inspiring melodies boost party
    Merchant,  // Trade mastery yields resources
    Warrior,   // Battle fury area attacks
    Cardinal,  // Divine grace heals allies
    Alchemist, // Transmutation creates potions
    Forager,   // Nature's bounty finds resources
    Thief,     // Backstab positional attacks
}

/// Marker component for character creation screen entities
#[derive(Component)]
struct CharacterCreateScreen;

fn setup_character_create(mut commands: Commands) {
    // Spawn character creation UI
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        CharacterCreateScreen,
        children![
            // Header text
            (
                Text::new("Create Your Character"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            // Placeholder text
            (
                Text::new("Character creation coming soon..."),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
                Node {
                    margin: UiRect::top(Val::Px(30.0)),
                    ..default()
                },
            ),
            // Continue instruction
            (
                Text::new("Press SPACE to continue"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::top(Val::Px(50.0)),
                    ..default()
                },
            ),
        ],
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
