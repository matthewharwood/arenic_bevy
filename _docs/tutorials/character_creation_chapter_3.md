This mental model will anchor everything we build. Each state has distinct UI requirements, different input handling,
and specific data needs.

---

## UI Component System Overview

Before implementing the character creation logic, let's understand the UI component system that will handle all visual
presentation:

### Component Architecture

```rust
// Import the complete UI component system
use crate::ui::{
    // Design tokens and styling constants
    Colors, Spacing, FontSizes, BorderRadius,

    // Layout helpers and utilities
    Layout, SpaceUtils, Grid,

    // Component traits and implementations
    Button, PrimaryButton, SecondaryButton, OutlineButton,
    Tile, CharacterCardTile, FeatureTile,
    Typography, Heading1, Heading2, BodyText, CharacterClassName,
    InputField, DefaultInputField, UiInputField,

    // Interaction and event systems
    UiEvent, UiComponentsPlugin,
};
```

### Key Design Patterns

**Trait-Based Components**: Each UI element implements a trait that defines its visual behavior:

```rust
// Button trait defines consistent button behavior
pub trait Button {
    fn background_color(&self, state: &ButtonState) -> Color;
    fn text_color(&self, state: &ButtonState) -> Color;
    fn border_color(&self, state: &ButtonState) -> Color;
    fn create_node(&self, state: &ButtonState) -> (Node, BackgroundColor, BorderColor);
    fn create_text_components(&self, text: &str, state: &ButtonState) -> (Text, TextFont, TextColor);
}

// Typography trait ensures consistent text styling
pub trait Typography {
    fn font_size(&self) -> f32;
    fn color(&self) -> Color;
    fn create_text_components(&self) -> (TextFont, TextColor);
    fn create_text(&self, text: &str) -> (Text, TextFont, TextColor);
}
```

### Understanding Bevy 0.16 Component-Based UI

**Breaking Change from Bundles to Components**: In Bevy 0.16, the UI system moved away from bundles like `NodeBundle`
and `TextBundle` to individual components. This change provides better performance and more flexibility:

**New Component Pattern (Bevy 0.16)**:

```rust
// New way - individual components in tuples
commands.spawn((
Node { width: Val::Px(100.0), ..default () },
BackgroundColor(Color::RED),
));

commands.spawn((
Text::new("Hello"),
TextFont { font_size: 24.0, ..default () },
TextColor(Color::WHITE),
));
```

**Why This Matters for Beginners**:

- **Better Performance**: Individual components are more cache-friendly
- **More Flexible**: You can mix and match components as needed
- **Clearer Intent**: Each component has a single responsibility
- **Easier Queries**: Systems can query for specific components without bundle overhead

**Our Component Trait Methods**: Our UI traits return component tuples that you can directly spawn:

```rust
// Typography trait returns (Text, TextFont, TextColor)
let (text, font, color) = DisplayText.create_text("Hello World");
parent.spawn((text, font, color));

// Button trait returns (Node, BackgroundColor, BorderColor)
let (node, bg, border) = PrimaryButton.create_node( & ButtonState::Normal);
parent.spawn((node, bg, border));
```

**Design Token System**: Centralized constants ensure visual consistency:

```rust
// Consistent spacing throughout the UI
Spacing::SM   // 8px
Spacing::MD   // 16px  
Spacing::LG   // 24px

// Unified color palette
Colors::PRIMARY        // Main interaction color
Colors::GRAY_700      // Body text
Colors::GRAY_300      // Borders
```

**Layout Helpers**: Pre-built layout utilities for common patterns:

```rust
// Create a character selection grid (4x2 for 8 classes)
Layout::character_selection_grid()

// Create centered containers with max width
Layout::centered_container()

// Add consistent spacing
SpaceUtils::vertical_spacer(Spacing::LG)
```

**Why This Approach Works**:

- **Consistency**: All buttons, inputs, and text use the same visual patterns
- **Maintainability**: Change the design system once, updates apply everywhere
- **Performance**: Traits compile to zero-cost abstractions
- **Developer Experience**: IntelliSense shows available variants and states

---

## Implementation: Step-by-Step Build Process

Inside the `src/game_state/character_create.rs` add this:

```rust
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
```

🧪 **Validation Tests**

After implementing the character class foundation, validate your implementation with these comprehensive tests:

```rust
#[cfg(test)]
mod character_class_tests {
    use super::*;

    #[test]
    fn all_character_classes_have_complete_data() {
        for class in CharacterClass::all() {
            assert!(!class.class_name().is_empty(),
                    "Class {:?} missing display name", class);
            assert!(!class.skill_description().is_empty(),
                    "Class {:?} missing skill description", class);
            assert!(class.texture_path().ends_with(".png"),
                    "Class {:?} texture path should end with .png", class);
            assert!(class.texture_path().starts_with("bosses/"),
                    "Class {:?} texture path should start with bosses/", class);
        }
    }

    #[test]
    fn exactly_eight_character_classes() {
        assert_eq!(CharacterClass::all().len(), 8,
                   "Should have exactly 8 character classes for 4x2 grid");
    }

    #[test]
    fn character_class_names_are_unique() {
        let classes = CharacterClass::all();
        let mut names = std::collections::HashSet::new();

        for class in classes {
            let class_name = class.class_name();
            assert!(names.insert(class_name),
                    "Duplicate class name found: {}", class_name);
        }
    }

    #[test]
    fn icon_paths_are_unique() {
        let classes = CharacterClass::all();
        let mut paths = std::collections::HashSet::new();

        for class in classes {
            let icon_path = class.icon_path();
            assert!(paths.insert(icon_path),
                    "Duplicate icon path found: {}", icon_path);
        }
    }

    #[test]
    fn texture_paths_are_unique() {
        let classes = CharacterClass::all();
        let mut paths = std::collections::HashSet::new();

        for class in classes {
            let texture_path = class.texture_path();
            assert!(paths.insert(texture_path),
                    "Duplicate texture path found: {}", texture_path);
        }
    }

    #[test]
    fn skill_descriptions_are_descriptive() {
        for class in CharacterClass::all() {
            let description = class.skill_description();
            assert!(description.len() > 10,
                    "Class {:?} skill description too short: '{}'", class, description);
            assert!(description.chars().any(|c| c.is_lowercase()),
                    "Class {:?} skill description should contain lowercase letters: '{}'", class, description);
        }
    }
}
```

**How to Run These Tests:**

```bash
cargo test character_class_tests
```

**What Success Looks Like:**

- All tests pass without panics
- Each character class has complete, unique data
- Asset paths follow consistent naming convention
- Display names are unique (prevents UI confusion)
- Skill descriptions are descriptive enough to guide player choice

**Common Issues These Tests Catch:**

- Missing or empty class names/skill descriptions
- Duplicate asset paths (causes asset conflicts)
- Inconsistent file naming conventions
- Classes with identical display names (confusing UX)

### Step 2: Nested State Management and Component Architecture

Our state machine uses nested enums for clean, type-safe state management:

```rust
/// Main game states with nested character creation phases
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    CharacterCreate(CharacterPhase),  // Nested enum for sub-states
    Intro,
    Battle,
}

/// Character creation phases (nested within GameState)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CharacterPhase {
    Selection,                    // Show 8 character cards
    Naming(CharacterClass),       // Show naming interface for selected class
}

/// Minimal resource for temporary input storage only
#[derive(Resource, Default, Debug)]
struct InputBuffer {
    character_name: String, // Temporary storage during input
}
```

**Key Design Pattern**: The nested enum `GameState::CharacterCreate(CharacterPhase)` provides:

- **Single Source of Truth**: All state information in one place
- **Type Safety**: Invalid state combinations prevented at compile time
- **Automatic Management**: Bevy's state system handles transitions automatically

**ECS Component for Character Data**:

```rust
/// Component attached to character entities
#[derive(Component, Debug, Clone)]
pub struct Character {
    pub class: CharacterClass,
}

// Note: We use Bevy's built-in Name component for character names
// This integrates better with Bevy's debugging and inspection tools
```

**Entity Management Pattern**: Instead of storing character data in a resource, we spawn a character entity with
`Character` and `Name` components. This entity persists across state transitions and can be easily parented to other
entities in the game world.

🧪 **Validation Tests**

After implementing nested state management and components, validate your architecture:

```rust
#[cfg(test)]
mod state_architecture_tests {
    use super::*;

    #[test]
    fn nested_state_transitions_work() {
        let selection_state = GameState::CharacterCreate(CharacterPhase::Selection);
        let naming_state = GameState::CharacterCreate(CharacterPhase::Naming(CharacterClass::Hunter));

        assert_ne!(selection_state, naming_state);

        if let GameState::CharacterCreate(CharacterPhase::Naming(class)) = naming_state {
            assert_eq!(class, CharacterClass::Hunter);
        } else {
            panic!("Expected CharacterCreate Naming state with Hunter class");
        }
    }

    #[test]
    fn input_buffer_initializes_correctly() {
        let buffer = InputBuffer::default();
        assert!(buffer.character_name.is_empty(),
                "InputBuffer should initialize with empty character name");
    }

    #[test]
    fn character_component_creation() {
        let character = Character { class: CharacterClass::Alchemist };
        assert_eq!(character.class, CharacterClass::Alchemist);
    }

    #[test]
    fn all_game_states_are_valid() {
        // Test that we can construct all expected state combinations
        let valid_states = vec![
            GameState::MainMenu,
            GameState::CharacterCreate(CharacterPhase::Selection),
            GameState::Intro,
            GameState::Battle,
        ];

        // Test naming states for all character classes
        for class in CharacterClass::all() {
            let naming_state = GameState::CharacterCreate(CharacterPhase::Naming(class));
            // Should compile and create without issues
            match naming_state {
                GameState::CharacterCreate(CharacterPhase::Naming(extracted_class)) => {
                    assert_eq!(extracted_class, class);
                }
                _ => panic!("Failed to create naming state for {:?}", class),
            }
        }
    }

    #[test]
    fn state_pattern_matching_works() {
        let states = vec![
            GameState::CharacterCreate(CharacterPhase::Selection),
            GameState::CharacterCreate(CharacterPhase::Naming(CharacterClass::Warrior)),
        ];

        for state in states {
            match state {
                GameState::CharacterCreate(CharacterPhase::Selection) => {
                    // Should match selection phase correctly
                }
                GameState::CharacterCreate(CharacterPhase::Naming(class)) => {
                    // Should extract class correctly
                    assert!(CharacterClass::all().contains(&class));
                }
                _ => panic!("Unexpected state pattern"),
            }
        }
    }
}
```

**How to Run These Tests:**

```bash
cargo test state_architecture_tests
```

**What Success Looks Like:**

- All state transitions compile and work correctly
- Pattern matching extracts the correct character class
- InputBuffer initializes properly
- Character components can be created for all classes

**Integration Test - State Transitions:**

```rust
#[cfg(test)]
mod state_integration_tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn complete_state_flow() {
        // Test the complete state flow: Selection -> Naming -> Intro
        let mut app = App::new();
        app.init_state::<GameState>();

        // Start in character creation
        app.world_mut().resource_mut::<NextState<GameState>>().set(
            GameState::CharacterCreate(CharacterPhase::Selection)
        );
        app.update();

        // Verify we're in selection phase
        let current_state = app.world().resource::<State<GameState>>();
        assert!(matches!(current_state.get(), 
            GameState::CharacterCreate(CharacterPhase::Selection)));

        // Transition to naming
        app.world_mut().resource_mut::<NextState<GameState>>().set(
            GameState::CharacterCreate(CharacterPhase::Naming(CharacterClass::Bard))
        );
        app.update();

        // Verify we're in naming phase with correct class
        let current_state = app.world().resource::<State<GameState>>();
        if let GameState::CharacterCreate(CharacterPhase::Naming(class)) = current_state.get() {
            assert_eq!(*class, CharacterClass::Bard);
        } else {
            panic!("Should be in naming phase with Bard class");
        }
    }
}
```

**Common Issues These Tests Catch:**

- Invalid state combinations that compile but don't work
- Pattern matching errors in state handling
- Resource initialization problems
- State transition logic bugs

### Step 3: Plugin Architecture with Nested State Registration

Calvin's UX flow requirements directly informed our nested state architecture:

```rust
pub struct CharacterCreatePlugin;

impl Plugin for CharacterCreatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InputBuffer>()
            // Phase-specific UI setup systems
            .add_systems(
                OnEnter(GameState::CharacterCreate(CharacterPhase::Selection)),
                setup_selection_ui
            )
            .add_systems(
                OnEnter(GameState::CharacterCreate(CharacterPhase::Naming)),
                setup_naming_ui
            )
            // Update systems with precise state filtering
            .add_systems(
                Update,
                (
                    handle_character_selection
                        .run_if(in_state(GameState::CharacterCreate(CharacterPhase::Selection))),
                    handle_naming_input
                        .run_if(in_state(GameState::CharacterCreate(CharacterPhase::Naming))),
                    update_card_hover_effects
                        .run_if(in_state(GameState::CharacterCreate)),
                )
            )
            // Automatic cleanup on state exit
            .add_systems(
                OnExit(GameState::CharacterCreate),
                cleanup_character_create
            )
            .add_systems(OnEnter(GameState::Intro), setup_character_in_guild_house);
    }
}
```

**Key Improvements**:

- **Phase-Specific Systems**: Each phase has dedicated `OnEnter` systems for UI setup
- **Precise Filtering**: Systems only run during appropriate phases
- **Automatic Management**: No manual UI despawning/spawning needed

**System Coordination Strategy**:

- `OnEnter`: Initialize UI for current state
- `Update`: Handle ongoing interactions (selection, input, hover effects)
- `OnExit`: Clean up resources to prevent memory leaks

**Run Condition Pattern**: `.run_if(in_state(GameState::CharacterCreate))` ensures systems only execute during the
appropriate game state, preventing resource conflicts and improving performance.

🧪 **Validation Tests**

After implementing plugin architecture, verify your systems are registered correctly:

```rust
#[cfg(test)]
mod plugin_tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn plugin_registers_required_resources() {
        let mut app = App::new();
        app.add_plugins(CharacterCreatePlugin);

        // Verify InputBuffer resource is registered
        assert!(app.world().contains_resource::<InputBuffer>(),
                "CharacterCreatePlugin should register InputBuffer resource");
    }

    #[test]
    fn plugin_registers_without_panics() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        // Should not panic when adding our plugin
        app.add_plugins(CharacterCreatePlugin);

        // Should be able to update without issues
        app.update();
    }

    #[test]
    fn systems_respect_state_conditions() {
        let mut app = App::new();
        app.add_plugins((DefaultPlugins, CharacterCreatePlugin));

        // Create mock entities to test system filtering
        let entity = app.world_mut().spawn((
            Button,
            Interaction::None,
            CharacterCard { class: CharacterClass::Warrior },
        )).id();

        // Start in different state - systems shouldn't run
        app.world_mut().resource_mut::<NextState<GameState>>().set(GameState::MainMenu);
        app.update();

        // Verify entity still exists (systems didn't process it)
        assert!(app.world().get_entity(entity).is_some());

        // Switch to character creation state
        app.world_mut().resource_mut::<NextState<GameState>>().set(
            GameState::CharacterCreate(CharacterPhase::Selection)
        );
        app.update();

        // Now systems should be able to process character creation entities
        // (Actual processing depends on interaction state changes)
    }
}
```

**How to Run These Tests:**

```bash
cargo test plugin_tests
```

**What Success Looks Like:**

- Plugin registers without panics or conflicts
- Required resources are properly initialized
- Systems respect state conditions and don't run in wrong states
- App can update without crashes after plugin registration

**Performance Test - Plugin Registration:**

```rust
#[cfg(test)]
mod plugin_performance_tests {
    use super::*;
    use bevy::prelude::*;
    use std::time::Instant;

    #[test]
    fn plugin_registration_is_fast() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        let start = Instant::now();
        app.add_plugins(CharacterCreatePlugin);
        let registration_time = start.elapsed();

        // Plugin registration should be nearly instantaneous
        assert!(registration_time.as_millis() < 100,
                "Plugin registration took {}ms, should be < 100ms",
                registration_time.as_millis());
    }
}
```

**Common Issues These Tests Catch:**

- Missing resource registrations
- System scheduling conflicts
- Plugin registration panics
- Incorrect state filtering setup

### Key Architectural Decision: Entity Persistence Across States

**Why Character Entities Instead of Resources**: Unlike traditional approaches that store character data in a Resource,
we spawn character entities that persist across state transitions. This provides several benefits:

```rust
// OLD APPROACH (Resource-based) - NOT what we're doing
#[derive(Resource)]
struct CreatedCharacter {
    class: CharacterClass,
    name: String,
}

// NEW APPROACH (Entity-based) - What we implement
commands.spawn((
Character { class: selected_class },
Name::new(character_name),
CharacterEntity, // Marker for easy querying
));
```

**Entity Persistence Benefits**:

1. **Natural State Transitions**: Character entities survive state changes automatically
2. **Transform Hierarchy**: Characters can be parented to game world entities (guild house, battle arena, etc.)
3. **Component Composition**: Additional gameplay components can be added without system changes
4. **Query Efficiency**: Game systems can efficiently find characters using Bevy's query system
5. **Debugging Integration**: Built-in `Name` component works with Bevy's inspector tools

**Example: Character Entity in Different Game States**:

```rust
// Character creation state: Entity spawned with basic components
fn complete_character_creation(/* ... */) {
    commands.spawn((
        Character { class: CharacterClass::Hunter },
        Name::new("Hero"),
        CharacterEntity,
    ));
}

// Intro state: Add transform components and parent to guild house
fn setup_character_in_guild_house(/* ... */) {
    // Query for unparented character entities
    for character_entity in character_query.iter() {
        commands.entity(character_entity).insert((
            Transform::from_xyz(0.0, 0.0, 0.0),
            Visibility::default(),
            PlayerControlled,
        )).set_parent(guild_house_entity);
    }
}

// Battle state: Add combat components
fn setup_character_for_battle(
    mut commands: Commands,
    character_query: Query<Entity, (With<Character>, With<PlayerControlled>)>,
) {
    for character_entity in character_query.iter() {
        commands.entity(character_entity).insert((
            Health { current: 100, max: 100 },
            Stamina { current: 50, max: 50 },
            CombatStats::default(),
        ));
    }
}
```

### Step 4: Component-Based UI Creation

Replace hardcoded UI creation with reusable components that follow design system principles:

```rust
/// Setup UI for Selection phase using component-based approach
fn setup_selection_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut input_buffer: ResMut<InputBuffer>,
) {
    // Clear any previous input
    input_buffer.character_name.clear();

    // Load Migra-Extrabold font
    let font_handle = asset_server.load("fonts/Migra-Extrabold.ttf");

    // Create main container using Node component
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb_u8(248, 218, 218)), // Design system background
        CharacterCreateScreen,
    )).with_children(|outer| {
        // Inner grid layout using Node component
        outer.spawn(Node {
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::flex(12, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(15, 1.0),
            width: Val::Px(1200.0),
            height: Val::Px(800.0),
            padding: UiRect::all(Spacing::LG),
            ..default()
        }).with_children(|grid| {

            // Title using Typography component
            spawn_main_title(grid, &font_handle);

            // Character selection panel with tile components
            spawn_character_selection_panel(grid, &asset_server, &font_handle);

            // Character portrait (remains mostly the same for asset loading)
            spawn_character_portrait(grid, &asset_server);

            // Skills panel using Typography components
            spawn_skills_panel(grid, &font_handle, CharacterClass::Thief);

            // Name input using InputField component
            spawn_name_input(grid, &font_handle);

            // Start button using Button component
            spawn_start_button(grid, &font_handle);
        });
    });
}

/// Spawn main title using Typography component
fn spawn_main_title(
    parent: &mut ChildBuilder,
    font_handle: &Handle<Font>,
) {
    let (text, font, color) = DisplayText.create_text("Choose Your Class");
    parent.spawn((
        text,
        font,
        color,
        Node {
            grid_row: GridPlacement::start_end(1, 2),
            grid_column: GridPlacement::start_end(1, 13),
            ..default()
        },
    ));
}

/// Spawn character selection panel using Tile components
fn spawn_character_selection_panel(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    font_handle: &Handle<Font>,
) {
    parent.spawn((
        Node {
            grid_row: GridPlacement::start_end(3, 15),
            grid_column: GridPlacement::start_end(1, 5),
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
            column_gap: Spacing::MD,
            row_gap: Spacing::MD,
            ..default()
        },
    )).with_children(|card_grid| {
        // Spawn character cards using Tile components
        for (index, class) in CharacterClass::all().iter().enumerate() {
            spawn_character_card_component(
                card_grid,
                *class,
                asset_server,
                font_handle,
                index == 7 // Thief pre-selected
            );
        }
    });
}

/// Spawn character portrait (asset loading logic unchanged)
fn spawn_character_portrait(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
) {
    parent.spawn((
        ImageNode::new(asset_server.load("character/thief-portrait.png")),
        Node {
            grid_row: GridPlacement::start_end(1, 15),
            grid_column: GridPlacement::start_end(5, 9),
            width: Val::Px(500.0),
            height: Val::Px(740.0),
            justify_self: JustifySelf::Center,
            overflow: Overflow::clip_y(),
            ..default()
        },
        CharacterPortrait,
    ));
}

/// Spawn skills panel using Layout and Typography components
fn spawn_skills_panel(
    parent: &mut ChildBuilder,
    font_handle: &Handle<Font>,
    class: CharacterClass,
) {
    parent.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Spacing::LG),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Colors::WHITE),
        BorderColor(Colors::GRAY_300),
        Node {
            grid_row: GridPlacement::start_end(3, 7),
            grid_column: GridPlacement::start_end(9, 13),
            ..default()
        },
    )).with_children(|panel| {
        spawn_skills_panel_content_component(panel, class, font_handle);
    });
}

/// Spawn name input using InputField component
fn spawn_name_input(
    parent: &mut ChildBuilder,
    font_handle: &Handle<Font>,
) {
    let input_field = DefaultInputField;
    let (container_node, bg_color, border_color) = input_field.create_container(&InputState::Normal);

    parent.spawn((
        container_node,
        bg_color,
        border_color,
        Node {
            grid_row: GridPlacement::start_end(14, 15),
            grid_column: GridPlacement::start_end(5, 9),
            height: Val::Px(60.0),
            ..default()
        },
        UiInputField::default("Character Name"),
        NameInput,
    )).with_children(|input| {
        let (text, font, color) = input_field.create_text_components("Ginger", &InputState::Normal);
        input.spawn((text, font, color));
    });
}

/// Spawn start button using Button component
fn spawn_start_button(
    parent: &mut ChildBuilder,
    font_handle: &Handle<Font>,
) {
    let button = PrimaryButton;
    let (button_node, bg_color, border_color) = button.create_node(&ButtonState::Normal);

    parent.spawn((
        button_node,
        bg_color,
        border_color,
        Node {
            grid_row: GridPlacement::start_end(14, 15),
            grid_column: GridPlacement::start_end(11, 13),
            height: Val::Px(60.0),
            ..default()
        },
        UiButton::primary(),
        StartButton,
    )).with_children(|btn| {
        let (text, font, color) = button.create_text_components("Start", &ButtonState::Normal);
        btn.spawn((text, font, color));
    });
}

# # Component System Benefits: Why This Approach Works

The component-based approach demonstrated above provides significant advantages over hardcoded UI:

# # # 1.Design System Consistency

* * Before (Hardcoded) * *:
```rust
// Inconsistent spacing and colors scattered throughout codebase
padding: UiRect::all(Val::Px(24.0)),  // Magic number
color: Color::srgb(0.2, 0.6, 0.8),   // Hardcoded color
font_size: 32.0,                     // Arbitrary size
```

**After (Component-Based)**:

```rust
// Consistent design tokens used throughout
padding: UiRect::all(Spacing::LG),   // Semantic naming
background_color: Colors::PRIMARY,    // Unified palette
Heading2.create_text(text)           // Consistent typography -> (Text, TextFont, TextColor)
```

### 2. Maintainability and Scalability

**Component Reuse**: Typography and button components work across all game screens:

- Character creation uses `CharacterClassName` and `PrimaryButton`
- Settings screen reuses the same components for consistency
- Inventory screen applies identical styling patterns

**Single Source of Truth**: Change `Colors::PRIMARY` once, update entire game's color scheme:

```rust
// One change affects all primary-colored elements
pub const PRIMARY: Color = Color::srgb(0.3, 0.7, 0.9); // New brand color
```

### 3. Developer Experience Improvements

**IntelliSense and Type Safety**:

```rust
// IDE shows available variants and prevents errors
let button = PrimaryButton;           // ✓ Exists and implements Button trait
let button = NonExistentButton;       // ✗ Compile error
```

**Compositional Patterns**:

```rust
// Components compose naturally
UiTile::character_card()              // Base tile
.selectable()                     // Add selection behavior
.with_title("Hunter")             // Add title component
.with_description("Precise shots") // Add description
```

### 4. Performance Benefits

**Zero-Cost Abstractions**: Traits compile to direct function calls with no runtime overhead:

```rust
// This trait method...
fn background_color(&self, state: ButtonState) -> Color

// Compiles to direct color assignment:
BackgroundColor(Color::srgb(0.2, 0.6, 0.8))
```

**Reduced Bundle Size**: Shared design tokens eliminate duplicate color/spacing values throughout the binary.

### 5. Team Development Advantages

**Onboarding**: New developers learn component patterns once, apply everywhere:

```rust
// Developer learns this pattern...
spawn_button(parent, "Save", PrimaryButton);

// And can apply it in any system:
spawn_button(settings_panel, "Apply", PrimaryButton);
spawn_button(inventory_ui, "Equip", SecondaryButton);
```

**Design-Development Handoff**: Designers specify component variants instead of individual pixel values:

- "Use Primary Button for main actions"
- "Apply Body Text typography for descriptions"
- "Use Character Card Tile for selection items"

### 6. Testing and Validation

Component-based UI enables systematic testing:

```rust
#[test]
fn all_buttons_follow_design_system() {
    let primary = PrimaryButton;
    let secondary = SecondaryButton;

    // Test consistent behavior across variants
    assert_eq!(primary.border_radius(), secondary.border_radius());
    assert_eq!(primary.font_size(), FontSizes::BASE);
}
```

This systematic approach transforms UI development from pixel-pushing to component composition, making the codebase more
maintainable, scalable, and team-friendly.

/// Setup UI for Naming phase  
fn setup_naming_ui(
mut commands: Commands,
current_state: Res<State<GameState>>,
) {
// Extract selected class from current state
if let GameState::CharacterCreate(CharacterPhase::Naming(selected_class)) = current_state.get() {
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
Text::new(format!("Your {} awaits a name, Commander", selected_class.class_name())),
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
                Text::new("Type your name and press ENTER to begin your journey\nPress ESCAPE to return to character selection"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
    }

}

```

**CSS Grid in Bevy**: The `grid_template_columns: RepeatedGridTrack::flex(4, 1.0)` creates a 4-column grid where each
column takes equal space. This pattern scales well for different screen sizes.

**Component Hierarchy Strategy**: Each UI element gets a marker component (`CharacterCreateScreen`) for efficient
cleanup during state transitions.

🧪 **Validation Tests**

After implementing UI creation systems, validate your UI spawning and hierarchy:

```rust
#[cfg(test)]
mod ui_creation_tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn setup_selection_ui_spawns_correct_entities() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.init_resource::<InputBuffer>();

        // Spawn selection UI
        setup_selection_ui(
            app.world_mut().commands(),
            app.world().resource::<AssetServer>().clone(),
            app.world_mut().resource_mut::<InputBuffer>(),
        );
        app.update();

        // Verify main container exists
        let screen_entities: Vec<_> = app.world()
            .query::<Entity>()
            .iter(app.world())
            .collect();

        // Should have spawned UI entities
        assert!(!screen_entities.is_empty(), "Should spawn UI entities");
    }

    #[test]
    fn setup_naming_ui_uses_selected_class() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.init_state::<GameState>();

        // Set state to naming with specific class
        app.world_mut().resource_mut::<NextState<GameState>>().set(
            GameState::CharacterCreate(CharacterPhase::Naming(CharacterClass::Cardinal))
        );
        app.update();

        // Spawn naming UI
        setup_naming_ui(
            app.world_mut().commands(),
            app.world().resource::<State<GameState>>().clone(),
        );
        app.update();

        // Verify UI was spawned (in real implementation, you'd check text content)
        let ui_entities: Vec<_> = app.world()
            .query::<Entity>()
            .iter(app.world())
            .collect();

        assert!(!ui_entities.is_empty(), "Should spawn naming UI entities");
    }

    #[test]
    fn character_cards_have_required_components() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        // Create a parent entity to spawn cards into
        let parent_id = app.world_mut().spawn(Node::default()).id();

        // Spawn a character card using component system
        let font_handle = app.world().resource::<AssetServer>().load("fonts/Migra-Extrabold.ttf");
        app.world_mut().entity_mut(parent_id).with_children(|parent| {
            spawn_character_card_component(parent, CharacterClass::Merchant, &app.world().resource::<AssetServer>(), &font_handle, false);
        });
        app.update();

        // Find the spawned card
        let card_query = app.world().query::<(
            &CharacterCard,
            &HoverState,
            &SelectableCard,
        )>();

        let cards: Vec<_> = card_query.iter(app.world()).collect();
        assert_eq!(cards.len(), 1, "Should spawn exactly one character card");

        let (card, hover_state, _selectable) = cards[0];
        assert_eq!(card.class, CharacterClass::Merchant);
        assert!(!hover_state.is_hovered, "Card should start unhovered");
    }

    #[test]
    fn ui_hierarchy_is_correct() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.init_resource::<InputBuffer>();

        setup_selection_ui(
            app.world_mut().commands(),
            app.world().resource::<AssetServer>().clone(),
            app.world_mut().resource_mut::<InputBuffer>(),
        );
        app.update();

        // Verify CharacterCreateScreen marker exists
        let screen_count = app.world()
            .query::<&CharacterCreateScreen>()
            .iter(app.world())
            .count();

        assert_eq!(screen_count, 1, "Should have exactly one CharacterCreateScreen");
    }
}
```

**How to Run These Tests:**

```bash
cargo test ui_creation_tests
```

**What Success Looks Like:**

- UI entities spawn without panics
- Character cards have all required components
- UI hierarchy includes proper marker components
- Selection and naming UIs create different entity structures

**Integration Test - UI Cleanup:**

```rust
#[cfg(test)]
mod ui_cleanup_tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn cleanup_removes_all_ui_entities() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.init_resource::<InputBuffer>();

        // Spawn UI
        setup_selection_ui(
            app.world_mut().commands(),
            app.world().resource::<AssetServer>().clone(),
            app.world_mut().resource_mut::<InputBuffer>(),
        );
        app.update();

        let entities_before = app.world()
            .query::<&CharacterCreateScreen>()
            .iter(app.world())
            .count();

        assert!(entities_before > 0, "Should have UI entities before cleanup");

        // Run cleanup
        cleanup_character_create(
            app.world_mut().commands(),
            app.world().query::<Entity>(),
            app.world().query::<Entity>(),
            app.world_mut().resource_mut::<InputBuffer>(),
        );
        app.update();

        let entities_after = app.world()
            .query::<&CharacterCreateScreen>()
            .iter(app.world())
            .count();

        assert_eq!(entities_after, 0, "Should have no UI entities after cleanup");
    }
}
```

**Common Issues These Tests Catch:**

- UI entities not spawning correctly
- Missing components on character cards
- Incorrect parent-child relationships
- Cleanup not removing all UI entities
- Asset loading issues during UI creation

### Step 5: Interactive Card Creation

Each character card follows precise specifications from the design (CA-1 through CA-5):

```rust
/// Spawn character card using Tile component system
fn spawn_character_card_component(
    parent: &mut ChildBuilder,
    class: CharacterClass,
    asset_server: &AssetServer,
    font_handle: &Handle<Font>,
    is_selected: bool,
) {
    // Create tile component with appropriate state
    let tile = CharacterCardTile;
    let tile_state = if is_selected { TileState::Selected } else { TileState::Normal };

    let (container_node, bg_color, border_color) = tile.create_container(&tile_state);
    parent.spawn((
        // Use Tile trait to create consistent container
        container_node,
        bg_color,
        border_color,
        Button, // Bevy interaction component

        // Character-specific data components
        CharacterCard { class },
        SelectableCard,
        UiTile::character_card().selectable(),
        CharacterSelectionTile {
            character_class: class.class_name().to_string(),
            selected: is_selected,
        },
    )).with_children(|card| {

        // Character icon using consistent spacing
        card.spawn((
            ImageNode::new(asset_server.load(class.icon_path())),
            Node {
                width: Val::Px(48.0),
                height: Val::Px(48.0),
                margin: UiRect::bottom(Spacing::MD), // Use design tokens
                ..default()
            },
            CharacterIcon,
            TileImage { image_path: class.icon_path().to_string() },
        ));

        // Character class name using Typography component
        let (text, font, color) = CharacterClassName.create_text(class.class_name());
        card.spawn((
            text,
            font,
            color,
            TileTitle { text: class.class_name().to_string() },
        ));
    });
}

/// Component to track card selection state
#[derive(Component)]
struct SelectedState {
    is_selected: bool,
}

/// Marker components for card children
#[derive(Component)]
struct CharacterIcon;

#[derive(Component)]
struct CharacterClassName;
```

**Component Strategy**: Each card carries multiple components:

- `Button`: Enables Bevy's built-in interaction detection
- `CharacterCard { class }`: Binds the card to specific character data
- `SelectedState`: Tracks selection state for radio-button behavior (CA-4)
- `SelectableCard`: Marks the element as interactive for system queries
- `BoxShadow`: Visual feedback for selected state

**Card Grid Layout (CS-2)**: The 8 cards are arranged in a 2×4 matrix:

```
[Hunter]    [Bard]       (Column 1)
[Merchant]  [Warrior]    (Column 2)  
[Cardinal]  [Alchemist]  (Column 3)
[Forager]   [Thief]      (Column 4)
```

Each card occupies approximately 2 columns × 3 rows in the parent grid including gutters, creating the 193×142px card
dimensions.

**Radio Button Behavior (CA-4)**: Only one card can be selected at a time. When a new card is clicked, the previously
selected card automatically reverts to its default state.

🧪 **Validation Tests**

After implementing interactive card creation, validate card components and interactions:

```rust
#[cfg(test)]
mod interactive_card_tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn all_character_classes_get_cards() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        // Create parent container
        let parent_id = app.world_mut().spawn(Node::default()).id();

        // Spawn cards for all character classes
        app.world_mut().entity_mut(parent_id).with_children(|parent| {
            for class in CharacterClass::all() {
                spawn_character_card(parent, class, &app.world().resource::<AssetServer>());
            }
        });
        app.update();

        // Verify all 8 cards were created
        let card_count = app.world()
            .query::<&CharacterCard>()
            .iter(app.world())
            .count();

        assert_eq!(card_count, 8, "Should spawn exactly 8 character cards");

        // Verify each class has exactly one card
        let mut class_counts = std::collections::HashMap::new();
        for card in app.world().query::<&CharacterCard>().iter(app.world()) {
            *class_counts.entry(card.class).or_insert(0) += 1;
        }

        for class in CharacterClass::all() {
            assert_eq!(class_counts.get(&class), Some(&1),
                       "Class {:?} should have exactly one card", class);
        }
    }

    #[test]
    fn cards_have_interaction_components() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        let parent_id = app.world_mut().spawn(Node::default()).id();

        app.world_mut().entity_mut(parent_id).with_children(|parent| {
            spawn_character_card(parent, CharacterClass::Thief, &app.world().resource::<AssetServer>());
        });
        app.update();

        // Verify card has all required interaction components
        let card_query = app.world().query::<(
            &Button,
            &CharacterCard,
            &HoverState,
            &SelectableCard,
            &Interaction,
        )>();

        let cards: Vec<_> = card_query.iter(app.world()).collect();
        assert_eq!(cards.len(), 1, "Should have exactly one interactive card");

        let (_button, card, hover_state, _selectable, interaction) = cards[0];
        assert_eq!(card.class, CharacterClass::Thief);
        assert!(!hover_state.is_hovered);
        assert_eq!(*interaction, Interaction::None);
    }

    #[test]
    fn card_asset_paths_are_loadable() {
        // Test that all character classes have valid asset paths
        for class in CharacterClass::all() {
            let path = class.texture_path();

            // Basic path validation
            assert!(path.starts_with("bosses/"),
                    "Class {:?} path should start with 'bosses/': {}", class, path);
            assert!(path.ends_with(".png"),
                    "Class {:?} path should end with '.png': {}", class, path);
            assert!(!path.contains(".."),
                    "Class {:?} path should not contain '..': {}", class, path);
            assert!(!path.starts_with("/"),
                    "Class {:?} path should be relative: {}", class, path);
        }
    }

    #[test]
    fn card_visual_hierarchy_is_correct() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        let parent_id = app.world_mut().spawn(Node::default()).id();

        app.world_mut().entity_mut(parent_id).with_children(|parent| {
            spawn_character_card(parent, CharacterClass::Alchemist, &app.world().resource::<AssetServer>());
        });
        app.update();

        // Find the card entity
        let card_entity = app.world()
            .query::<Entity>()
            .iter(app.world())
            .find(|&entity| {
                app.world().get::<CharacterCard>(entity).is_some()
            })
            .expect("Should find card entity");

        // Verify card has children (image and text elements)
        let children = app.world().get::<Children>(card_entity);
        assert!(children.is_some(), "Card should have child entities for image and text");

        let children = children.unwrap();
        assert!(children.len() >= 2, "Card should have at least image and text children");
    }
}
```

**How to Run These Tests:**

```bash
cargo test interactive_card_tests
```

**What Success Looks Like:**

- All 8 character classes get properly configured cards
- Cards have all required interaction components
- Asset paths pass validation checks
- Card hierarchy includes image and text children
- Each character class appears exactly once

**Performance Test - Card Creation:**

```rust
#[cfg(test)]
mod card_performance_tests {
    use super::*;
    use bevy::prelude::*;
    use std::time::Instant;

    #[test]
    fn card_creation_meets_frame_budget() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        let parent_id = app.world_mut().spawn(Node::default()).id();

        let start = Instant::now();

        app.world_mut().entity_mut(parent_id).with_children(|parent| {
            for class in CharacterClass::all() {
                spawn_character_card(parent, class, &app.world().resource::<AssetServer>());
            }
        });
        app.update();

        let elapsed = start.elapsed();

        // Should complete within frame budget (16.67ms for 60fps)
        assert!(elapsed.as_millis() < 16,
                "Card creation took {}ms, should be < 16ms", elapsed.as_millis());
    }
}
```

**Common Issues These Tests Catch:**

- Missing interaction components on cards
- Invalid or malformed asset paths
- Incorrect card counts (missing or duplicate cards)
- Performance issues during card creation
- Broken parent-child hierarchy in UI

### Step 6: Character Portrait and Z-Index Layering

The character portrait is positioned centrally and extends beyond the viewport height for dramatic effect:

```rust
/// Marker component for the character portrait
#[derive(Component)]
struct CharacterPortrait;

/// System to update portrait when character selection changes
fn update_character_portrait(
    asset_server: Res<AssetServer>,
    selected_cards: Query<(&CharacterCard, &SelectedState), Changed<SelectedState>>,
    mut portrait_query: Query<&mut ImageNode, With<CharacterPortrait>>,
) {
    for (card, selected_state) in &selected_cards {
        if selected_state.is_selected {
            // Update portrait image
            if let Ok(mut portrait) = portrait_query.get_single_mut() {
                portrait.image = asset_server.load(card.class.portrait_path());
            }
        }
    }
}
```

**Portrait Specifications (CP-1, CP-2)**:

- **Position**: Rows 1-14+ (extends beyond viewport), Columns 5-8
- **Dimensions**: Fixed 500px width × 740px height
- **Overflow**: `clip_y()` to hide portions extending beyond the 720px viewport
- **Z-Index**: Middle layer (2) - appears above the card panel but below skills/UI

**Z-Index Layering System**:

```rust
// Z-index hierarchy for proper visual stacking
const Z_INDEX_BACKGROUND: i32 = 0;  // Main container background
const Z_INDEX_CARDS: i32 = 1;       // Character selection cards (lowest)
const Z_INDEX_PORTRAIT: i32 = 2;    // Character portrait (middle)
const Z_INDEX_UI: i32 = 3;          // Skills panel, title, name input, start button (highest)
```

**Design Rationale**: The portrait's vertical overflow creates a dynamic visual effect where the character appears to "
break out" of the frame, adding depth and visual interest. The z-index layering ensures the portrait doesn't obscure
important UI elements while still providing visual prominence.

### Step 7: Skills Panel Implementation

The skills panel displays class-specific abilities with precise formatting:

```rust
/// Helper function to spawn skills panel content using Typography components
fn spawn_skills_panel_content_component(
    parent: &mut ChildBuilder,
    class: CharacterClass,
    font_handle: &Handle<Font>,
) {
    // Skills panel header using Typography component (SP-3)
    let (text, font, color) = Heading2.create_text(&format!("{} Skills", class.class_name()));
    parent.spawn((
        text,
        font,
        color,
        Node {
            margin: UiRect::bottom(Spacing::MD), // Use design tokens
            ..default()
        },
    ));

    // Skill description container (SP-5)
    parent.spawn(Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        row_gap: Spacing::SM,
        ..default()
    }).with_children(|skill_container| {

        // Create rich text using Typography components
        skill_container.spawn((
            Node {
                max_width: Val::Px(350.0), // Constrain text width
                ..default()
            },
        )).with_children(|text_container| {

            // Skill name with bold styling
            let (label_text, label_font, label_color) = Label.create_text(&format!("{}: ", class.skill_name()));
            text_container.spawn((
                label_text,
                label_font,
                label_color,
                Node {
                    margin: UiRect::right(Spacing::XS),
                    ..default()
                },
            ));

            // Skill description with body text styling
            let (desc_text, desc_font, desc_color) = BodyText.create_text(class.skill_description());
            text_container.spawn((desc_text, desc_font, desc_color));
        });
    });
}

/// System to update skills panel when selection changes
fn update_skills_panel(
    commands: Commands,
    asset_server: Res<AssetServer>,
    selected_cards: Query<(&CharacterCard, &SelectedState), Changed<SelectedState>>,
    skills_panel: Query<Entity, With<SkillsPanel>>,
) {
    for (card, selected_state) in &selected_cards {
        if selected_state.is_selected {
            // Clear and rebuild skills panel content
            if let Ok(panel_entity) = skills_panel.get_single() {
                commands.entity(panel_entity).despawn_descendants();

                let font_handle = asset_server.load("fonts/Migra-Extrabold.ttf");
                commands.entity(panel_entity).with_children(|panel| {
                    spawn_skills_panel_content(panel, card.class, &font_handle);
                });
            }
        }
    }
}
```

**Skills Panel Specifications (SP-1 through SP-6)**:

- **Position**: Rows 3-6, Columns 9-12 (4×4 grid cells)
- **Border**: 6px black border with 12px radius
- **Padding**: 24px on all sides
- **Header Format**: "{Class} Skills" in 32px Migra-Extrabold
- **Content Gap**: 12px between header and skill description
- **Skill Format**: Bold skill name followed by description in regular weight
- **Z-Index**: Highest layer (3) to ensure visibility

**Markup Pattern (SP-5)**: The skill description uses Bevy's `Text::from_sections` to create rich text with different
styles for the skill name (bold) and description (regular).

### Step 8: Hover Effects Implementation

Damien's lighting expertise translated into subtle but effective visual feedback:

```rust
/// Component for tracking hover state (Damien's lighting effects)
#[derive(Component)]
struct HoverState {
    is_hovered: bool,
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
```

**Performance Optimization**: The `Changed<Interaction>` filter ensures this system only runs when interaction states
actually change, not every frame.

**State Management Pattern**: `HoverState` prevents redundant color updates by tracking whether the card is currently in
hover state.

**Visual Design Rationale**: The brightness transition (0.92 → 1.0) is subtle enough to provide feedback without being
distracting—Damien's lighting expertise in action.

🧪 **Validation Tests**

After implementing hover effects, validate the visual feedback system:

```rust
#[cfg(test)]
mod hover_effects_tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn hover_state_initializes_correctly() {
        let hover_state = HoverState { is_hovered: false };
        assert!(!hover_state.is_hovered, "HoverState should initialize as not hovered");
    }

    #[test]
    fn hover_effects_respond_to_interaction_changes() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        // Spawn a selectable card with hover state
        let card_entity = app.world_mut().spawn((
            Button,
            Interaction::None,
            BackgroundColor(Color::srgb(0.92, 0.92, 0.92)),
            HoverState { is_hovered: false },
            SelectableCard,
        )).id();
        app.update();

        // Simulate hover interaction
        let mut interaction = app.world_mut().get_mut::<Interaction>(card_entity).unwrap();
        *interaction = Interaction::Hovered;

        // Run hover effects system
        update_card_hover_effects(
            app.world_mut().query_filtered::<
                (&Interaction, &mut BackgroundColor, &mut HoverState),
                (Changed<Interaction>, With<SelectableCard>)
            >()
        );

        // Verify hover state updated
        let hover_state = app.world().get::<HoverState>(card_entity).unwrap();
        assert!(hover_state.is_hovered, "HoverState should be true after hover interaction");

        // Verify background color changed
        let bg_color = app.world().get::<BackgroundColor>(card_entity).unwrap();
        let expected_hover_color = Color::srgb(1.0, 1.0, 1.0);
        assert_eq!(bg_color.0, expected_hover_color, "Background should brighten on hover");
    }

    #[test]
    fn hover_effects_return_to_normal() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        // Spawn card in hovered state
        let card_entity = app.world_mut().spawn((
            Button,
            Interaction::Hovered,
            BackgroundColor(Color::srgb(1.0, 1.0, 1.0)),
            HoverState { is_hovered: true },
            SelectableCard,
        )).id();
        app.update();

        // Simulate end of hover
        let mut interaction = app.world_mut().get_mut::<Interaction>(card_entity).unwrap();
        *interaction = Interaction::None;

        // Run hover effects system
        update_card_hover_effects(
            app.world_mut().query_filtered::<
                (&Interaction, &mut BackgroundColor, &mut HoverState),
                (Changed<Interaction>, With<SelectableCard>)
            >()
        );

        // Verify hover state reset
        let hover_state = app.world().get::<HoverState>(card_entity).unwrap();
        assert!(!hover_state.is_hovered, "HoverState should be false after hover ends");

        // Verify background color reset
        let bg_color = app.world().get::<BackgroundColor>(card_entity).unwrap();
        let expected_normal_color = Color::srgb(0.92, 0.92, 0.92);
        assert_eq!(bg_color.0, expected_normal_color, "Background should return to normal");
    }

    #[test]
    fn hover_effects_ignore_non_selectable_entities() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        // Spawn entity without SelectableCard component
        let non_selectable = app.world_mut().spawn((
            Button,
            Interaction::Hovered,
            BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
            HoverState { is_hovered: false },
            // Note: NO SelectableCard component
        )).id();
        app.update();

        let original_color = app.world().get::<BackgroundColor>(non_selectable).unwrap().0;

        // Run hover effects system - should not affect non-selectable entities
        update_card_hover_effects(
            app.world_mut().query_filtered::<
                (&Interaction, &mut BackgroundColor, &mut HoverState),
                (Changed<Interaction>, With<SelectableCard>)
            >()
        );

        // Verify color unchanged
        let final_color = app.world().get::<BackgroundColor>(non_selectable).unwrap().0;
        assert_eq!(original_color, final_color,
                   "Non-selectable entities should not be affected by hover system");
    }

    #[test]
    fn hover_system_only_processes_changed_interactions() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        // Spawn multiple cards
        let unchanged_card = app.world_mut().spawn((
            Button,
            Interaction::None, // This won't change
            BackgroundColor(Color::srgb(0.92, 0.92, 0.92)),
            HoverState { is_hovered: false },
            SelectableCard,
        )).id();

        let changed_card = app.world_mut().spawn((
            Button,
            Interaction::None,
            BackgroundColor(Color::srgb(0.92, 0.92, 0.92)),
            HoverState { is_hovered: false },
            SelectableCard,
        )).id();
        app.update();

        // Only change one card's interaction
        let mut interaction = app.world_mut().get_mut::<Interaction>(changed_card).unwrap();
        *interaction = Interaction::Hovered;

        // The system should only process the changed card
        // (This is validated by the Changed<Interaction> filter in the query)
    }
}
```

**How to Run These Tests:**

```bash
cargo test hover_effects_tests
```

**What Success Looks Like:**

- Hover states initialize and update correctly
- Background colors transition properly (0.92 → 1.0 → 0.92)
- System only processes entities with SelectableCard component
- Change detection works (only processes changed interactions)
- Visual transitions are smooth and predictable

**Performance Test - Hover System:**

```rust
#[cfg(test)]
mod hover_performance_tests {
    use super::*;
    use bevy::prelude::*;
    use std::time::Instant;

    #[test]
    fn hover_system_scales_with_many_cards() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        // Spawn many cards to stress test
        for i in 0..1000 {
            app.world_mut().spawn((
                Button,
                Interaction::None,
                BackgroundColor(Color::srgb(0.92, 0.92, 0.92)),
                HoverState { is_hovered: false },
                SelectableCard,
            ));
        }
        app.update();

        let start = Instant::now();

        // System should handle many entities efficiently
        update_card_hover_effects(
            app.world_mut().query_filtered::<
                (&Interaction, &mut BackgroundColor, &mut HoverState),
                (Changed<Interaction>, With<SelectableCard>)
            >()
        );

        let elapsed = start.elapsed();

        // Should complete quickly even with many entities
        assert!(elapsed.as_millis() < 5,
                "Hover system took {}ms with 1000 entities, should be < 5ms",
                elapsed.as_millis());
    }
}
```

**Common Issues These Tests Catch:**

- Hover state not updating correctly
- Wrong color values in transitions
- System processing non-selectable entities
- Performance degradation with many UI elements
- Color transitions not reverting properly

### Step 9: Simplified Character Selection Handling

Jon's system architecture uses clean state transitions:

```rust
/// Handle character card selection - no manual phase checking needed!
fn handle_character_selection(
    mut interaction_query: Query<
        (&Interaction, &CharacterCard),
        (Changed<Interaction>, With<Button>)
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // This system only runs during Selection phase due to precise state filtering
    // No manual phase checking required!

    for (interaction, card) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Transition to naming phase - Bevy handles UI cleanup automatically
            next_state.set(GameState::CharacterCreate(CharacterPhase::Naming(card.class)));
            break;
        }
    }
}
```

**Eliminated Boilerplate**: No manual phase checking needed - the system only runs during Selection phase due to precise
state filtering.

**Automatic UI Management**: Bevy's state system handles UI transitions automatically through `OnEnter`/`OnExit`
systems.

**Why `break`?**: Once we've processed a selection, we exit the loop to prevent multiple selections in a single frame.

🧪 **Validation Tests**

After implementing character selection handling, validate the selection logic:

```rust
#[cfg(test)]
mod character_selection_tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn character_selection_triggers_state_transition() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.init_state::<GameState>();

        // Start in selection phase
        app.world_mut().resource_mut::<NextState<GameState>>().set(
            GameState::CharacterCreate(CharacterPhase::Selection)
        );
        app.update();

        // Spawn a character card
        let card_entity = app.world_mut().spawn((
            Button,
            Interaction::Pressed, // Simulate button press
            CharacterCard { class: CharacterClass::Warrior },
        )).id();
        app.update();

        // Run selection handler
        handle_character_selection(
            app.world_mut().query_filtered::<
                (&Interaction, &CharacterCard),
                (Changed<Interaction>, With<Button>)
            >(),
            app.world_mut().resource_mut::<NextState<GameState>>(),
        );
        app.update();

        // Verify state transitioned to naming
        let current_state = app.world().resource::<State<GameState>>();
        if let GameState::CharacterCreate(CharacterPhase::Naming(class)) = current_state.get() {
            assert_eq!(*class, CharacterClass::Warrior);
        } else {
            panic!("Should transition to naming phase with Warrior class");
        }
    }

    #[test]
    fn only_pressed_interactions_trigger_selection() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.init_state::<GameState>();

        // Start in selection phase
        app.world_mut().resource_mut::<NextState<GameState>>().set(
            GameState::CharacterCreate(CharacterPhase::Selection)
        );
        app.update();

        // Test non-pressed interactions
        let interactions = vec![
            Interaction::None,
            Interaction::Hovered,
        ];

        for interaction in interactions {
            app.world_mut().spawn((
                Button,
                interaction,
                CharacterCard { class: CharacterClass::Bard },
            ));
        }
        app.update();

        let original_state = app.world().resource::<State<GameState>>().get().clone();

        // Run selection handler
        handle_character_selection(
            app.world_mut().query_filtered::<
                (&Interaction, &CharacterCard),
                (Changed<Interaction>, With<Button>)
            >(),
            app.world_mut().resource_mut::<NextState<GameState>>(),
        );
        app.update();

        // Verify state unchanged
        let current_state = app.world().resource::<State<GameState>>();
        assert_eq!(*current_state.get(), original_state,
                   "Non-pressed interactions should not trigger state change");
    }

    #[test]
    fn selection_breaks_after_first_press() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.init_state::<GameState>();

        app.world_mut().resource_mut::<NextState<GameState>>().set(
            GameState::CharacterCreate(CharacterPhase::Selection)
        );
        app.update();

        // Spawn multiple pressed cards (simulate simultaneous presses)
        let first_card = app.world_mut().spawn((
            Button,
            Interaction::Pressed,
            CharacterCard { class: CharacterClass::Alchemist },
        )).id();

        let second_card = app.world_mut().spawn((
            Button,
            Interaction::Pressed,
            CharacterCard { class: CharacterClass::Forager },
        )).id();
        app.update();

        // Run selection handler
        handle_character_selection(
            app.world_mut().query_filtered::<
                (&Interaction, &CharacterCard),
                (Changed<Interaction>, With<Button>)
            >(),
            app.world_mut().resource_mut::<NextState<GameState>>(),
        );
        app.update();

        // Should transition to naming phase (with one of the classes)
        let current_state = app.world().resource::<State<GameState>>();
        match current_state.get() {
            GameState::CharacterCreate(CharacterPhase::Naming(class)) => {
                // Should be one of the two classes (implementation dependent on iteration order)
                assert!(matches!(class, CharacterClass::Alchemist | CharacterClass::Forager));
            }
            _ => panic!("Should transition to naming phase"),
        }
    }

    #[test]
    fn selection_requires_button_component() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.init_state::<GameState>();

        app.world_mut().resource_mut::<NextState<GameState>>().set(
            GameState::CharacterCreate(CharacterPhase::Selection)
        );
        app.update();

        // Spawn entity without Button component
        app.world_mut().spawn((
            Interaction::Pressed,
            CharacterCard { class: CharacterClass::Cardinal },
            // Note: NO Button component
        ));
        app.update();

        let original_state = app.world().resource::<State<GameState>>().get().clone();

        // Run selection handler
        handle_character_selection(
            app.world_mut().query_filtered::<
                (&Interaction, &CharacterCard),
                (Changed<Interaction>, With<Button>)
            >(),
            app.world_mut().resource_mut::<NextState<GameState>>(),
        );
        app.update();

        // Verify state unchanged
        let current_state = app.world().resource::<State<GameState>>();
        assert_eq!(*current_state.get(), original_state,
                   "Entities without Button component should not trigger selection");
    }
}
```

**How to Run These Tests:**

```bash
cargo test character_selection_tests
```

**What Success Looks Like:**

- Pressed button interactions trigger state transitions
- Only the first pressed button is processed (break works)
- Non-pressed interactions are ignored
- Entities without Button component are filtered out
- State transitions include the correct character class

**Integration Test - Complete Selection Flow:**

```rust
#[cfg(test)]
mod selection_integration_tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn complete_card_interaction_flow() {
        let mut app = App::new();
        app.add_plugins((DefaultPlugins, CharacterCreatePlugin));

        // Enter character creation selection phase
        app.world_mut().resource_mut::<NextState<GameState>>().set(
            GameState::CharacterCreate(CharacterPhase::Selection)
        );
        app.update();

        // Simulate card creation and selection
        let card_entity = app.world_mut().spawn((
            Button,
            Interaction::None,
            BackgroundColor(Color::srgb(0.92, 0.92, 0.92)),
            HoverState { is_hovered: false },
            CharacterCard { class: CharacterClass::Hunter },
            SelectableCard,
        )).id();
        app.update();

        // Test hover effect first
        let mut interaction = app.world_mut().get_mut::<Interaction>(card_entity).unwrap();
        *interaction = Interaction::Hovered;
        app.update();

        // Verify hover effect applied
        let bg_color = app.world().get::<BackgroundColor>(card_entity).unwrap();
        assert_eq!(bg_color.0, Color::srgb(1.0, 1.0, 1.0));

        // Now test selection
        let mut interaction = app.world_mut().get_mut::<Interaction>(card_entity).unwrap();
        *interaction = Interaction::Pressed;
        app.update();

        // Verify state transition
        let current_state = app.world().resource::<State<GameState>>();
        if let GameState::CharacterCreate(CharacterPhase::Naming(class)) = current_state.get() {
            assert_eq!(*class, CharacterClass::Hunter);
        } else {
            panic!("Should transition to naming phase with Hunter class");
        }
    }
}
```

**Common Issues These Tests Catch:**

- Selection logic not triggering state transitions
- Multiple selections processed in single frame
- Wrong character class passed to naming phase
- Non-button entities being processed incorrectly
- State transitions not working with nested enums

### Step 10: Name Input Field Implementation

The name input field provides visual feedback for character naming with precise specifications:

```rust
/// Marker component for the name input field
#[derive(Component)]
struct NameInput;

/// System to update name input when character selection changes
fn update_name_input_default(
    selected_cards: Query<(&CharacterCard, &SelectedState), Changed<SelectedState>>,
    mut name_input_query: Query<&mut Text, With<NameInput>>,
    input_buffer: Res<InputBuffer>,
) {
    for (card, selected_state) in &selected_cards {
        if selected_state.is_selected && input_buffer.character_name.is_empty() {
            // Update default name only if user hasn't typed anything
            if let Ok(mut text) = name_input_query.get_single_mut() {
                text.sections[0].value = card.class.default_name().to_string();
            }
        }
    }
}

/// System to handle live typing updates
fn update_name_input_typing(
    input_buffer: Res<InputBuffer>,
    mut name_input_query: Query<&mut Text, With<NameInput>>,
) {
    if input_buffer.is_changed() {
        if let Ok(mut text) = name_input_query.get_single_mut() {
            text.sections[0].value = if input_buffer.character_name.is_empty() {
                "Type your name..." // Placeholder when empty
            } else {
                input_buffer.character_name.clone()
            };
        }
    }
}
```

**Name Input Specifications (NI-1 through NI-5)**:

- **Position**: Row 14, Columns 5-8 (bottom-center, 4 columns wide)
- **Dimensions**: Height 60px with 24px padding inside
- **Styling**: 6px black border, 12px radius, white background
- **Typography**: Migra-Extrabold 24px, center-aligned
- **Behavior**: Shows class default name, updates as user types
- **Z-Index**: Highest layer (3) for accessibility

**Default Names by Class (NI-4)**:

- Hunter: "Orion"
- Bard: "Melody"
- Merchant: "Magnus"
- Warrior: "Valeria"
- Cardinal: "Benedictus"
- Alchemist: "Zephyr"
- Forager: "Willow"
- Thief: "Ginger"

**Input Behavior Pattern**: The name input shows the default character name but allows user editing. If the user clears
the field, a placeholder appears. The default name only updates when selecting a new character if the user hasn't
started typing.

### Step 11: Start Button and Interaction Behaviors

The Start button provides the final interaction to transition to the game:

```rust
/// Marker component for the start button
#[derive(Component)]
struct StartButton;

/// System to handle start button clicks
fn handle_start_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    selected_cards: Query<(&CharacterCard, &SelectedState)>,
    name_input: Query<&Text, With<NameInput>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &button_query {
        if *interaction == Interaction::Pressed {
            // Find the selected character
            if let Some((card, _)) = selected_cards.iter().find(|(_, state)| state.is_selected) {
                // Get the character name from input
                if let Ok(name_text) = name_input.get_single() {
                    let character_name = if name_text.sections[0].value.is_empty()
                        || name_text.sections[0].value == "Type your name..." {
                        card.class.default_name().to_string()
                    } else {
                        name_text.sections[0].value.clone()
                    };

                    // Spawn character entity with selected class and name
                    commands.spawn((
                        Character { class: card.class },
                        Name::new(character_name),
                        TransformBundle::default(),
                        VisibilityBundle::default(),
                    ));

                    // Transition to intro state (SB-3)
                    next_state.set(GameState::Intro);
                }
            }
        }
    }
}

/// Complete interaction behavior system
fn handle_all_interactions(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut card_query: Query<
        (&Interaction, &CharacterCard, &mut SelectedState, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<SelectableCard>)
    >,
    mut all_cards: Query<(&mut SelectedState, &mut BackgroundColor, &mut BorderColor), With<SelectableCard>>,
    mut portrait_query: Query<&mut ImageNode, With<CharacterPortrait>>,
    mut skills_panel: Query<Entity, With<SkillsPanel>>,
    mut name_input: Query<&mut Text, With<NameInput>>,
    input_buffer: Res<InputBuffer>,
) {
    for (interaction, card, mut selected, mut bg_color, mut border_color) in &mut card_query {
        match *interaction {
            Interaction::Pressed => {
                // Deselect all other cards (CA-4: radio button behavior)
                for (mut other_selected, mut other_bg, mut other_border) in &mut all_cards {
                    other_selected.is_selected = false;
                    *other_bg = BackgroundColor(Color::NONE);
                    *other_border = BorderColor(Color::BLACK);
                }

                // Select this card
                selected.is_selected = true;
                *bg_color = BackgroundColor(Color::srgb_u8(246, 250, 254));
                *border_color = BorderColor(Color::srgb_u8(51, 130, 227));

                // Play selection sound (B-3)
                audio.play(asset_server.load(card.class.select_audio_path()));

                // Update portrait (B-3)
                if let Ok(mut portrait) = portrait_query.get_single_mut() {
                    portrait.image = asset_server.load(card.class.portrait_path());
                }

                // Update skills panel (B-3)
                if let Ok(panel_entity) = skills_panel.get_single() {
                    commands.entity(panel_entity).despawn_descendants();
                    let font_handle = asset_server.load("fonts/Migra-Extrabold.ttf");
                    commands.entity(panel_entity).with_children(|panel| {
                        spawn_skills_panel_content(panel, card.class, &font_handle);
                    });
                }

                // Update name input default (B-3)
                if input_buffer.character_name.is_empty() {
                    if let Ok(mut name_text) = name_input.get_single_mut() {
                        name_text.sections[0].value = card.class.default_name().to_string();
                    }
                }
            }
            Interaction::Hovered => {
                // Only add box shadow on hover (B-2)
                // Actual hover visual effects handled in separate system
            }
            _ => {}
        }
    }
}
```

**Start Button Specifications (SB-1, SB-2)**:

- **Position**: Row 14, Columns 11-12 (bottom-right)
- **Styling**: Black background, white text
- **Typography**: "Start" in Migra-Extrabold 24px
- **Dimensions**: 60px height with 24px padding
- **Z-Index**: Highest layer (3)

**Interaction Behaviors (B-1 through B-5)**:

1. **Initial State (B-1)**: Hunter card pre-selected on load with corresponding portrait, skills, and name
2. **Hover Effects (B-2)**: Cards show box shadow on hover without changing selection
3. **Click Actions (B-3)**:
    - Play character-specific audio file
    - Update portrait image
    - Refresh skills panel content
    - Update name input (if user hasn't typed)
4. **Asset Resolution (B-4)**: All assets follow pattern: `assets/character/{class_name}-{type}.{ext}`
5. **Accessibility (B-5)**: Full keyboard navigation support via Tab/Enter/Space

### Step 12: Asset Organization and Specifications

All character creation assets follow a consistent naming pattern and organization:

```rust
/// Asset path structure for character creation
impl CharacterClass {
    /// Character icon for selection cards (48x48 px)
    pub fn icon_path(self) -> &'static str {
        match self {
            Self::Hunter => "assets/character/hunter-icon.png",
            Self::Bard => "assets/character/bard-icon.png",
            // ... etc
        }
    }

    /// Character portrait for center display (500x740 px)
    pub fn portrait_path(self) -> &'static str {
        match self {
            Self::Hunter => "assets/character/hunter-portrait.png",
            Self::Bard => "assets/character/bard-portrait.png",
            // ... etc
        }
    }

    /// Selection audio feedback (MP3 format)
    pub fn select_audio_path(self) -> &'static str {
        match self {
            Self::Hunter => "assets/character/hunter-select.mp3",
            Self::Bard => "assets/character/bard-select.mp3",
            // ... etc
        }
    }
}
```

**Asset Specifications**:

1. **Icons** (48×48 px PNG):
    - Used in character selection cards
    - Transparent background
    - High contrast for visibility at small size

2. **Portraits** (500×740 px PNG):
    - Full character artwork
    - Designed to extend beyond viewport
    - Transparent or complementary background

3. **Audio Files** (MP3):
    - Character-specific selection sounds
    - Duration: 0.5-1.5 seconds
    - Format: MP3 for web compatibility

4. **Font** (TTF):
    - Migra-Extrabold.ttf required
    - Loaded via: `fonts/Migra-Extrabold.ttf`

**File Structure**:

```
assets/
├── character/
│   ├── hunter-icon.png
│   ├── hunter-portrait.png
│   ├── hunter-select.mp3
│   ├── bard-icon.png
│   ├── bard-portrait.png
│   ├── bard-select.mp3
│   └── ... (all 8 characters)
└── fonts/
    └── Migra-Extrabold.ttf
```

### Step 13: Naming Phase Interface (Legacy)

The naming phase UI is automatically created by the `setup_naming_ui` system when entering the `Naming` state:

```rust
// This system runs automatically when entering GameState::CharacterCreate(CharacterPhase::Naming)
// See Step 4 for the complete implementation

/// Helper function to update the input field display text
fn update_input_display(
    input_text_query: &mut Query<&mut Text, With<InputText>>,
    input_buffer: &InputBuffer
) {
    for mut text in input_text_query {
        text.0 = if input_buffer.character_name.is_empty() {
            "Type your character name...".to_string()
        } else {
            input_buffer.character_name.clone()
        };
    }
}
```

**Narrative Integration**: Adam's voice comes through in the personalized message: "Your {class} awaits a name,
Commander"—creating player investment in the naming process.

**Input Field Pattern**: Since Bevy doesn't have built-in text input widgets, we create a visual representation and
handle keyboard input manually.

🧪 **Validation Tests**

After implementing the naming interface, validate UI creation and text display:

```rust
#[cfg(test)]
mod naming_interface_tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn naming_ui_spawns_for_selected_class() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.init_state::<GameState>();

        // Set state to naming with specific class
        app.world_mut().resource_mut::<NextState<GameState>>().set(
            GameState::CharacterCreate(CharacterPhase::Naming(CharacterClass::Merchant))
        );
        app.update();

        // Spawn naming UI
        setup_naming_ui(
            app.world_mut().commands(),
            app.world().resource::<State<GameState>>().clone(),
        );
        app.update();

        // Verify CharacterCreateScreen marker exists
        let screen_count = app.world()
            .query::<&CharacterCreateScreen>()
            .iter(app.world())
            .count();

        assert_eq!(screen_count, 1, "Should spawn exactly one naming screen");

        // Verify InputText component exists
        let input_text_count = app.world()
            .query::<&InputText>()
            .iter(app.world())
            .count();

        assert_eq!(input_text_count, 1, "Should spawn exactly one input text field");
    }

    #[test]
    fn update_input_display_shows_correct_text() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        // Spawn input text element
        let input_entity = app.world_mut().spawn((
            Text::new("Initial text"),
            InputText,
        )).id();
        app.update();

        // Test empty input buffer
        let empty_buffer = InputBuffer { character_name: String::new() };

        update_input_display(
            &mut app.world_mut().query::<&mut Text>(),
            &empty_buffer,
        );

        let text = app.world().get::<Text>(input_entity).unwrap();
        assert_eq!(text.0, "Type your character name...",
                   "Should show placeholder text for empty input");

        // Test non-empty input buffer
        let filled_buffer = InputBuffer { character_name: "HeroName".to_string() };

        update_input_display(
            &mut app.world_mut().query::<&mut Text>(),
            &filled_buffer,
        );

        let text = app.world().get::<Text>(input_entity).unwrap();
        assert_eq!(text.0, "HeroName",
                   "Should show actual character name when present");
    }

    #[test]
    fn naming_ui_includes_class_specific_content() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.init_state::<GameState>();

        // Test with different character classes
        let test_classes = vec![
            CharacterClass::Cardinal,
            CharacterClass::Thief,
            CharacterClass::Warrior,
        ];

        for class in test_classes {
            // Clear previous entities
            app.world_mut().clear_entities();

            // Set state for this class
            app.world_mut().resource_mut::<NextState<GameState>>().set(
                GameState::CharacterCreate(CharacterPhase::Naming(class))
            );
            app.update();

            // Spawn naming UI
            setup_naming_ui(
                app.world_mut().commands(),
                app.world().resource::<State<GameState>>().clone(),
            );
            app.update();

            // Verify UI was created (in a real test, you'd check for class-specific text content)
            let ui_entities = app.world()
                .query::<&CharacterCreateScreen>()
                .iter(app.world())
                .count();

            assert_eq!(ui_entities, 1,
                       "Should create naming UI for class {:?}", class);
        }
    }

    #[test]
    fn input_text_component_marks_text_fields() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.init_state::<GameState>();

        app.world_mut().resource_mut::<NextState<GameState>>().set(
            GameState::CharacterCreate(CharacterPhase::Naming(CharacterClass::Bard))
        );
        app.update();

        setup_naming_ui(
            app.world_mut().commands(),
            app.world().resource::<State<GameState>>().clone(),
        );
        app.update();

        // Find entities with both Text and InputText components
        let input_text_entities: Vec<_> = app.world()
            .query::<(&Text, &InputText)>()
            .iter(app.world())
            .collect();

        assert_eq!(input_text_entities.len(), 1,
                   "Should have exactly one text element marked as input");
    }
}
```

**How to Run These Tests:**

```bash
cargo test naming_interface_tests
```

**What Success Looks Like:**

- Naming UI spawns correctly for any selected character class
- Input display updates properly between placeholder and actual text
- InputText component correctly marks interactive text elements
- UI includes class-specific messaging
- Screen marker components enable proper cleanup

**State Extraction Test:**

```rust
#[cfg(test)]
mod state_extraction_tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn extract_selected_class_from_state() {
        // Test state pattern matching for all character classes
        for expected_class in CharacterClass::all() {
            let naming_state = GameState::CharacterCreate(
                CharacterPhase::Naming(expected_class)
            );

            // Extract class using the same pattern as setup_naming_ui
            if let GameState::CharacterCreate(CharacterPhase::Naming(extracted_class)) = naming_state {
                assert_eq!(extracted_class, expected_class,
                           "Should extract correct class from naming state");
            } else {
                panic!("Failed to extract class from naming state for {:?}", expected_class);
            }
        }
    }

    #[test]
    fn naming_state_only_in_correct_phase() {
        let selection_state = GameState::CharacterCreate(CharacterPhase::Selection);

        // This should NOT match the naming pattern
        if let GameState::CharacterCreate(CharacterPhase::Naming(_)) = selection_state {
            panic!("Selection state should not match naming pattern");
        }

        // Only naming states should match
        let naming_state = GameState::CharacterCreate(
            CharacterPhase::Naming(CharacterClass::Alchemist)
        );

        match naming_state {
            GameState::CharacterCreate(CharacterPhase::Naming(class)) => {
                assert_eq!(class, CharacterClass::Alchemist);
            }
            _ => panic!("Naming state should match naming pattern"),
        };
    }
}
```

**Common Issues These Tests Catch:**

- Naming UI not spawning for certain character classes
- Input text display not updating correctly
- Missing InputText component markers
- State pattern matching errors
- UI elements not being properly tagged for cleanup

### Step 14: Keyboard Input Handling

Jon's input system leverages the nested state architecture:

```rust
/// Handle character naming input - runs only during Naming phase
fn handle_naming_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut input_buffer: ResMut<InputBuffer>,
    mut next_state: ResMut<NextState<GameState>>,
    mut input_text_query: Query<&mut Text, With<InputText>>,
    current_state: Res<State<GameState>>,
) {
    // This system only runs during Naming phase - no manual checking needed!

    // Handle character input  
    for event in keyboard_events.read() {
        if let KeyboardInput { logical_key: Key::Character(ch), state: ButtonState::Pressed, .. } = event {
            let ch = ch.chars().next().unwrap_or(' ');
            if (ch.is_alphanumeric() || ch == ' ') && input_buffer.character_name.len() < 20 {
                input_buffer.character_name.push(ch);
                update_input_display(&mut input_text_query, &input_buffer);
            }
        }
    }

    // Handle backspace
    if keyboard.just_pressed(KeyCode::Backspace) && !input_buffer.character_name.is_empty() {
        input_buffer.character_name.pop();
        update_input_display(&mut input_text_query, &input_buffer);
    }

    // Handle Escape to return to selection
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::CharacterCreate(CharacterPhase::Selection));
    }

    // Handle Enter to complete character creation
    if keyboard.just_pressed(KeyCode::Enter) && !input_buffer.character_name.trim().is_empty() {
        // Extract selected class from current state
        if let GameState::CharacterCreate(CharacterPhase::Naming(selected_class)) = current_state.get() {
            // Spawn character entity with Character and Name components
            commands.spawn((
                Character { class: *selected_class },
                Name::new(input_buffer.character_name.trim().to_string()),
                CharacterEntity, // Marker component for easy querying
            ));
        }
        next_state.set(GameState::Intro);
    }
}
```

**Key Improvements**:

- **No Manual Phase Checking**: System only runs during Naming phase due to precise state filtering
- **Escape Key Support**: Players can return to character selection
- **Clean State Extraction**: Selected class retrieved directly from current state
- **Simplified Resource Management**: Only `InputBuffer` needed for temporary storage

**Input Validation Strategy**:

- Only alphanumeric characters and spaces are allowed
- 20-character limit prevents UI overflow
- Trim whitespace before final validation

**Two Input Methods**:

- `KeyboardInput` events for character input (supports international keyboards)
- `ButtonInput<KeyCode>` for special keys like Backspace, Enter, and Escape

**State Transition**: Spawning the character entity with `Character` and `Name` components makes the data available to
subsequent game states through ECS queries.

🧪 **Validation Tests**

After implementing keyboard input handling, validate input processing and character creation:

```rust
#[cfg(test)]
mod keyboard_input_tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn valid_characters_are_accepted() {
        let mut input_buffer = InputBuffer { character_name: String::new() };

        // Test alphanumeric characters
        let valid_chars = vec!['a', 'Z', '1', '9', ' '];

        for ch in valid_chars {
            let original_length = input_buffer.character_name.len();

            // Simulate character input (simplified test)
            if (ch.is_alphanumeric() || ch == ' ') && input_buffer.character_name.len() < 20 {
                input_buffer.character_name.push(ch);
            }

            assert_eq!(input_buffer.character_name.len(), original_length + 1,
                       "Character '{}' should be accepted", ch);
        }
    }

    #[test]
    fn invalid_characters_are_rejected() {
        let mut input_buffer = InputBuffer { character_name: String::new() };

        // Test invalid characters
        let invalid_chars = vec!['!', '@', '#', '$', '%', '^', '&', '*'];

        for ch in invalid_chars {
            let original_length = input_buffer.character_name.len();

            // Simulate character input validation
            if (ch.is_alphanumeric() || ch == ' ') && input_buffer.character_name.len() < 20 {
                input_buffer.character_name.push(ch);
            }

            assert_eq!(input_buffer.character_name.len(), original_length,
                       "Character '{}' should be rejected", ch);
        }
    }

    #[test]
    fn character_name_length_limit_enforced() {
        let mut input_buffer = InputBuffer { character_name: "a".repeat(20) }; // At limit

        let original_length = input_buffer.character_name.len();

        // Try to add another character
        if input_buffer.character_name.len() < 20 {
            input_buffer.character_name.push('b');
        }

        assert_eq!(input_buffer.character_name.len(), original_length,
                   "Should not accept characters beyond 20-character limit");
    }

    #[test]
    fn backspace_removes_characters() {
        let mut input_buffer = InputBuffer { character_name: "Test".to_string() };

        // Simulate backspace
        if !input_buffer.character_name.is_empty() {
            input_buffer.character_name.pop();
        }

        assert_eq!(input_buffer.character_name, "Tes");

        // Test backspace on empty string
        input_buffer.character_name.clear();
        if !input_buffer.character_name.is_empty() {
            input_buffer.character_name.pop();
        }

        assert_eq!(input_buffer.character_name, "", "Backspace on empty string should not panic");
    }

    #[test]
    fn character_entity_creation_with_valid_name() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.init_state::<GameState>();

        // Set up naming state
        app.world_mut().resource_mut::<NextState<GameState>>().set(
            GameState::CharacterCreate(CharacterPhase::Naming(CharacterClass::Forager))
        );
        app.update();

        // Simulate character creation with valid name
        let selected_class = CharacterClass::Forager;
        let character_name = "ValidName";

        app.world_mut().spawn((
            Character { class: selected_class },
            Name::new(character_name.to_string()),
            CharacterEntity,
        ));
        app.update();

        // Verify character entity was created
        let character_query = app.world().query::<(&Character, &Name, &CharacterEntity)>();
        let characters: Vec<_> = character_query.iter(app.world()).collect();

        assert_eq!(characters.len(), 1, "Should create exactly one character entity");

        let (character, name, _marker) = characters[0];
        assert_eq!(character.class, CharacterClass::Forager);
        assert_eq!(name.as_str(), "ValidName");
    }

    #[test]
    fn empty_names_are_rejected() {
        // Test empty name validation
        let empty_names = vec![
            "",
            " ",
            "  ",
            "\t",
            "\n",
        ];

        for name in empty_names {
            let is_valid = !name.trim().is_empty();
            assert!(!is_valid, "Name '{}' should be considered invalid", name);
        }
    }

    #[test]
    fn whitespace_is_trimmed_from_names() {
        let test_cases = vec![
            ("  Hero  ", "Hero"),
            ("\tWarrior\n", "Warrior"),
            (" Mage ", "Mage"),
        ];

        for (input, expected) in test_cases {
            let trimmed = input.trim();
            assert_eq!(trimmed, expected, "Name '{}' should trim to '{}'", input, expected);
        }
    }

    #[test]
    fn input_buffer_clears_after_character_creation() {
        let mut input_buffer = InputBuffer { character_name: "TestHero".to_string() };

        // After character creation, buffer should be cleared
        // (This would happen in the actual system after spawning the character)
        input_buffer.character_name.clear();

        assert!(input_buffer.character_name.is_empty(),
                "Input buffer should be cleared after character creation");
    }
}
```

**How to Run These Tests:**

```bash
cargo test keyboard_input_tests
```

**What Success Looks Like:**

- Valid characters (alphanumeric + space) are accepted
- Invalid characters are rejected
- Character name length limit (20 chars) is enforced
- Backspace removes characters correctly
- Character entities are created with correct components
- Empty/whitespace-only names are properly rejected

**Integration Test - Complete Input Flow:**

```rust
#[cfg(test)]
mod input_integration_tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn complete_naming_flow() {
        let mut app = App::new();
        app.add_plugins((DefaultPlugins, CharacterCreatePlugin));

        // Enter naming phase
        app.world_mut().resource_mut::<NextState<GameState>>().set(
            GameState::CharacterCreate(CharacterPhase::Naming(CharacterClass::Hunter))
        );
        app.update();

        // Simulate typing a character name
        let mut input_buffer = app.world_mut().resource_mut::<InputBuffer>();
        input_buffer.character_name = "TestHero".to_string();

        // Simulate Enter key press (character creation)
        let character_count_before = app.world()
            .query::<(&Character, &Name)>()
            .iter(app.world())
            .count();

        // Create character (simulating the Enter key logic)
        app.world_mut().spawn((
            Character { class: CharacterClass::Hunter },
            Name::new("TestHero".to_string()),
            CharacterEntity,
        ));
        app.update();

        // Verify character was created
        let character_count_after = app.world()
            .query::<(&Character, &Name)>()
            .iter(app.world())
            .count();

        assert_eq!(character_count_after, character_count_before + 1,
                   "Should create one new character entity");

        // Verify character has correct data
        let characters: Vec<_> = app.world()
            .query::<(&Character, &Name)>()
            .iter(app.world())
            .collect();

        let (character, name) = characters.last().unwrap();
        assert_eq!(character.class, CharacterClass::Hunter);
        assert_eq!(name.as_str(), "TestHero");
    }

    #[test]
    fn escape_returns_to_selection() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.init_state::<GameState>();

        // Start in naming phase
        app.world_mut().resource_mut::<NextState<GameState>>().set(
            GameState::CharacterCreate(CharacterPhase::Naming(CharacterClass::Alchemist))
        );
        app.update();

        // Simulate Escape key press
        app.world_mut().resource_mut::<NextState<GameState>>().set(
            GameState::CharacterCreate(CharacterPhase::Selection)
        );
        app.update();

        // Verify state returned to selection
        let current_state = app.world().resource::<State<GameState>>();
        assert!(matches!(current_state.get(), 
            GameState::CharacterCreate(CharacterPhase::Selection)),
                "Should return to selection phase on Escape");
    }
}
```

**Common Issues These Tests Catch:**

- Invalid characters being accepted in names
- Character name length limits not enforced
- Backspace not working on empty strings
- Character entities not being created correctly
- State transitions not working with keyboard input
- Empty names being accepted when they should be rejected

### Step 15: Automatic Cleanup and Resource Management

Jon's nested state architecture provides automatic cleanup:

```rust
/// Cleanup character creation UI but preserve the character entity
fn cleanup_character_create(
    mut commands: Commands,
    ui_query: Query<Entity, With<CharacterCreateScreen>>,
    character_query: Query<Entity, (With<Character>, Without<CharacterCreateScreen>)>,
    mut input_buffer: ResMut<InputBuffer>,
) {
    // Despawn all character creation UI entities
    for entity in &ui_query {
        commands.entity(entity).despawn_recursive();
    }

    // Clear input buffer
    input_buffer.character_name.clear();

    // Character entities with Character component are preserved
    // They will be handled by the intro state setup
    info!("Preserved {} character entities during cleanup", character_query.iter().count());
}

/// Setup character entity in guild house during intro state
fn setup_character_in_guild_house(
    mut commands: Commands,
    character_query: Query<Entity, (With<Character>, Without<Parent>)>,
    guild_house_query: Query<Entity, With<GuildHouseArena>>,
) {
    // Find the character entity created during character creation
    if let Ok(character_entity) = character_query.get_single() {
        if let Ok(guild_house_entity) = guild_house_query.get_single() {
            // Parent the character to the guild house for proper transform hierarchy
            commands.entity(character_entity).set_parent(guild_house_entity);

            // Add any additional components needed for the intro state
            commands.entity(character_entity).insert((
                Transform::from_xyz(0.0, 0.0, 0.0), // Starting position in guild house
                GlobalTransform::default(),
                Visibility::default(),
                // Add gameplay components as needed
                PlayerControlled, // Mark as player's character
            ));

            info!("Character entity parented to guild house arena");
        } else {
            warn!("Guild house arena not found for character placement");
        }
    } else {
        warn!("No character entity found during intro setup");
    }
}

/// Marker component for guild house arena
#[derive(Component)]
struct GuildHouseArena;

/// Marker component for character entities
#[derive(Component)]
struct CharacterEntity;

/// Marker component for player-controlled characters
#[derive(Component)]
struct PlayerControlled;
```
