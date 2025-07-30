use bevy::prelude::*;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use super::GameState;

/// Plugin for the Character Creation state
pub struct CharacterCreatePlugin;

impl Plugin for CharacterCreatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CharacterCreationState>()
            .add_systems(OnEnter(GameState::CharacterCreate), setup_character_create)
            .add_systems(
                Update, 
                (
                    handle_character_selection,
                    handle_naming_input,
                    update_card_hover_effects,
                ).run_if(in_state(GameState::CharacterCreate))
            )
            .add_systems(OnExit(GameState::CharacterCreate), cleanup_character_create);
    }
}

/// The 8 character classes available for selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterClass {
    Trapper,      // "Hunter" from spec -> Trapper from codebase
    Alchemist,
    Sprinter,
    Gatherer,
    Thief,
    Tank,
    Cardinal,
    Collector,
}

impl CharacterClass {
    /// Get all character classes in grid order (4x2)
    pub fn all() -> [Self; 8] {
        [
            Self::Trapper, Self::Alchemist, Self::Sprinter, Self::Gatherer,
            Self::Thief, Self::Tank, Self::Cardinal, Self::Collector,
        ]
    }
    
    /// Get the display name for the character class
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Trapper => "The Trapper",
            Self::Alchemist => "The Alchemist", 
            Self::Sprinter => "The Sprinter",
            Self::Gatherer => "The Gatherer",
            Self::Thief => "The Thief",
            Self::Tank => "The Tank",
            Self::Cardinal => "The Cardinal",
            Self::Collector => "The Collector",
        }
    }
    
    /// Get Adam's narrative tagline for each class
    pub fn tagline(self) -> &'static str {
        match self {
            Self::Trapper => "Set cunning snares, control the battlefield",
            Self::Alchemist => "Transform matter, brew ancient mysteries",
            Self::Sprinter => "Strike swift, vanish without trace", 
            Self::Gatherer => "Harvest wisdom, hoard precious resources",
            Self::Thief => "Shadow and stealth, claim what isn't yours",
            Self::Tank => "Unyielding fortress, absorb all punishment",
            Self::Cardinal => "Divine authority, command through faith",
            Self::Collector => "Acquire everything, leave nothing behind",
        }
    }
    
    /// Get the asset path for the character icon
    pub fn texture_path(self) -> &'static str {
        match self {
            Self::Trapper => "bosses/trapper.png",
            Self::Alchemist => "bosses/alchemist.png",
            Self::Sprinter => "bosses/sprinter.png",
            Self::Gatherer => "bosses/gatherer.png",
            Self::Thief => "bosses/thief.png",
            Self::Tank => "bosses/tank.png",
            Self::Cardinal => "bosses/cardinal.png",
            Self::Collector => "bosses/collector.png",
        }
    }
}

/// Character creation phases
#[derive(Debug, Clone, PartialEq, Eq)]
enum CreationPhase {
    Selection,
    Naming(CharacterClass),
}

/// Resource to track character creation state
#[derive(Resource, Debug)]
struct CharacterCreationState {
    phase: CreationPhase,
    character_name: String,
}

impl Default for CharacterCreationState {
    fn default() -> Self {
        Self {
            phase: CreationPhase::Selection,
            character_name: String::new(),
        }
    }
}

/// Marker component for character creation screen entities
#[derive(Component)]
struct CharacterCreateScreen;

/// Component for character selection cards
#[derive(Component)]
struct CharacterCard {
    class: CharacterClass,
}

/// Component for interactive buttons
#[derive(Component)]
struct SelectableCard;

/// Component for tracking hover state (Damien's lighting effects)
#[derive(Component)]
struct HoverState {
    is_hovered: bool,
}

/// Component marker for the input text field display
#[derive(Component)]
struct InputText;

/// Resource to store the created character data for other states to access
#[derive(Resource, Debug, Clone)]
pub struct CreatedCharacter {
    pub class: CharacterClass,
    pub name: String,
}

fn setup_character_create(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut creation_state: ResMut<CharacterCreationState>,
) {
    // Reset creation state
    *creation_state = CharacterCreationState::default();
    
    // Spawn main character creation container
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center, 
            flex_direction: FlexDirection::Column,
            ..default()
        },
        // Calvin's red background (#E3334B)
        BackgroundColor(Color::srgb_u8(227, 51, 75)),
        CharacterCreateScreen,
    )).with_children(|parent| {
        // Header text - Adam's narrative
        parent.spawn((
            Text::new("Choose Your Path, Commander"),
            TextFont {
                font_size: 48.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::bottom(Val::Px(40.0)),
                ..default()
            },
        ));
        
        // Character cards grid container
        parent.spawn((
            Node {
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                grid_template_rows: RepeatedGridTrack::flex(2, 1.0), 
                column_gap: Val::Px(24.0),
                row_gap: Val::Px(24.0),
                ..default()
            },
        )).with_children(|grid| {
            // Spawn 8 character cards in 4x2 grid
            for class in CharacterClass::all() {
                spawn_character_card(grid, class, &asset_server);
            }
        });
    });
}

fn spawn_character_card(
    parent: &mut ChildSpawnerCommands,
    class: CharacterClass,
    asset_server: &AssetServer,
) {
    parent.spawn((
        Button,
        Node {
            width: Val::Px(200.0),  // Calvin's card dimensions 
            height: Val::Px(160.0),
            border: UiRect::all(Val::Px(3.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        // Calvin's white cards with thick borders - start at base brightness 0.92
        BackgroundColor(Color::srgb(0.92, 0.92, 0.92)),
        BorderColor(Color::WHITE),
        // Damien's hover effects - start with base brightness
        HoverState { is_hovered: false },
        CharacterCard { class },
        SelectableCard,
    )).with_children(|card| {
        // Character icon
        card.spawn((
            ImageNode::new(asset_server.load(class.texture_path())),
            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                margin: UiRect::bottom(Val::Px(8.0)),
                ..default()
            },
        ));
        
        // Character class name
        card.spawn((
            Text::new(class.display_name()),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::BLACK),
            Node {
                margin: UiRect::bottom(Val::Px(4.0)),
                ..default()
            },
        ));
        
        // Adam's tagline
        card.spawn((
            Text::new(class.tagline()),
            TextFont {
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::srgb(0.4, 0.4, 0.4)),
            Node {
                margin: UiRect::horizontal(Val::Px(8.0)),
                ..default()
            },
        ));
    });
}

/// Damien's lighting effects - update hover states and visual feedback
fn update_card_hover_effects(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut HoverState),
        (Changed<Interaction>, With<SelectableCard>)
    >,
) {
    for (interaction, mut bg_color, mut hover_state) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                if !hover_state.is_hovered {
                    // Damien's hover effect - brighten to 1.0 
                    *bg_color = BackgroundColor(Color::srgb(1.0, 1.0, 1.0));
                    hover_state.is_hovered = true;
                }
            }
            Interaction::None => {
                if hover_state.is_hovered {
                    // Return to base brightness 0.92
                    *bg_color = BackgroundColor(Color::srgb(0.92, 0.92, 0.92));
                    hover_state.is_hovered = false;
                }
            }
            _ => {}
        }
    }
}

/// Handle character card selection during Selection phase
fn handle_character_selection(
    mut interaction_query: Query<
        (&Interaction, &CharacterCard),
        (Changed<Interaction>, With<Button>)
    >,
    mut creation_state: ResMut<CharacterCreationState>,
    mut commands: Commands,
    screen_query: Query<Entity, With<CharacterCreateScreen>>,
) {
    // Only process selection during Selection phase
    if !matches!(creation_state.phase, CreationPhase::Selection) {
        return;
    }
    
    for (interaction, card) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Move to naming phase
            creation_state.phase = CreationPhase::Naming(card.class);
            creation_state.character_name.clear();
            
            // Clear current UI and setup naming interface
            for entity in &screen_query {
                commands.entity(entity).despawn();
            }
            
            setup_naming_interface(&mut commands, card.class);
            break;
        }
    }
}

/// Setup the character naming interface after selection
fn setup_naming_interface(commands: &mut Commands, selected_class: CharacterClass) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        // Keep Calvin's red background
        BackgroundColor(Color::srgb_u8(227, 51, 75)),
        CharacterCreateScreen,
    )).with_children(|parent| {
        // Adam's narrative feedback
        parent.spawn((
            Text::new(format!("Your {} awaits a name, Commander", selected_class.display_name())),
            TextFont {
                font_size: 36.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            },
        ));
        
        // Character name input field (visual representation)
        parent.spawn((
            Node {
                width: Val::Px(400.0),
                height: Val::Px(50.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::WHITE),
            BorderColor(Color::srgb(0.8, 0.8, 0.8)),
        )).with_children(|input_field| {
            input_field.spawn((
                Text::new("Type your character name..."),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
                InputText, // Marker for input text updates
            ));
        });
        
        // Instructions
        parent.spawn((
            Text::new("Type your name and press ENTER to begin your journey"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        ));
    });
}

/// Handle character naming input during Naming phase
fn handle_naming_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut creation_state: ResMut<CharacterCreationState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut input_text_query: Query<&mut Text, With<InputText>>,
) {
    if let CreationPhase::Naming(_selected_class) = creation_state.phase {
        // Handle character input  
        for event in keyboard_events.read() {
            if let KeyboardInput { logical_key: Key::Character(ch), state: ButtonState::Pressed, .. } = event {
                let ch = ch.chars().next().unwrap_or(' ');
                if (ch.is_alphanumeric() || ch == ' ') && creation_state.character_name.len() < 20 {
                    creation_state.character_name.push(ch);
                    
                    update_input_display(&mut input_text_query, &creation_state);
                }
            }
        }
        
        // Handle backspace
        if keyboard.just_pressed(KeyCode::Backspace) && !creation_state.character_name.is_empty() {
            creation_state.character_name.pop();
            
            update_input_display(&mut input_text_query, &creation_state);
        }
        
        // Handle Enter to complete character creation
        if keyboard.just_pressed(KeyCode::Enter) && !creation_state.character_name.trim().is_empty() {
            // Store the created character data for other states to access
            if let CreationPhase::Naming(selected_class) = creation_state.phase {
                commands.insert_resource(CreatedCharacter {
                    class: selected_class,
                    name: creation_state.character_name.trim().to_string(),
                });
            }
            next_state.set(GameState::Intro);
        }
    }
}

/// Helper function to update the input field display text
fn update_input_display(input_text_query: &mut Query<&mut Text, With<InputText>>, creation_state: &CharacterCreationState) {
    for mut text in input_text_query {
        text.0 = if creation_state.character_name.is_empty() {
            "Type your character name...".to_string()
        } else {
            creation_state.character_name.clone()
        };
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

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;

    #[test]
    fn character_classes_have_complete_data() {
        for class in CharacterClass::all() {
            assert!(!class.display_name().is_empty(), "Class {:?} missing display name", class);
            assert!(!class.tagline().is_empty(), "Class {:?} missing tagline", class);
            assert!(class.texture_path().ends_with(".png"), "Class {:?} texture path should end with .png", class);
            assert!(class.texture_path().starts_with("bosses/"), "Class {:?} texture path should start with bosses/", class);
        }
    }

    #[test]
    fn character_creation_state_default() {
        let state = CharacterCreationState::default();
        assert_eq!(state.phase, CreationPhase::Selection);
        assert!(state.character_name.is_empty());
    }

    #[test]
    fn creation_phase_transitions() {
        let selection_phase = CreationPhase::Selection;
        let naming_phase = CreationPhase::Naming(CharacterClass::Trapper);
        
        assert_ne!(selection_phase, naming_phase);
        
        if let CreationPhase::Naming(class) = naming_phase {
            assert_eq!(class, CharacterClass::Trapper);
        } else {
            panic!("Expected Naming phase with Trapper class");
        }
    }

    #[test]
    fn created_character_resource() {
        let character = CreatedCharacter {
            class: CharacterClass::Alchemist,
            name: "Test Character".to_string(),
        };
        
        assert_eq!(character.class, CharacterClass::Alchemist);
        assert_eq!(character.name, "Test Character");
    }

    #[test]
    fn character_create_plugin_registers_systems() {
        let mut app = App::new();
        app.add_plugins(CharacterCreatePlugin);
        
        // Verify the resource is initialized
        assert!(app.world().contains_resource::<CharacterCreationState>());
    }

    #[test]
    fn all_character_classes_count() {
        assert_eq!(CharacterClass::all().len(), 8, "Should have exactly 8 character classes");
    }

    #[test]
    fn character_class_display_names_are_unique() {
        let classes = CharacterClass::all();
        let mut names = std::collections::HashSet::new();
        
        for class in classes {
            let display_name = class.display_name();
            assert!(names.insert(display_name), "Duplicate display name found: {}", display_name);
        }
    }

    #[test]
    fn character_class_texture_paths_are_unique() {
        let classes = CharacterClass::all();
        let mut paths = std::collections::HashSet::new();
        
        for class in classes {
            let texture_path = class.texture_path();
            assert!(paths.insert(texture_path), "Duplicate texture path found: {}", texture_path);
        }
    }
}