## System Architecture Overview

Our character creation system follows Bevy's plugin architecture with ECS Component-based data management:

1. **Plugin Registration**: `CharacterCreatePlugin` encapsulates all functionality
2. **Component-Based Data**: Character entities with `Character` and `Name` components (not Resources)
3. **System Coordination**: Multiple systems handle different concerns (UI, input, state transitions)

```rust
// High-level system architecture - Nested Enum State + ECS Component Approach
CharacterCreatePlugin
├── States
│   └── GameState::CharacterCreate(CharacterPhase) (unified state management)
├── Resources
│   └── InputBuffer (temporary input storage only)
├── Components
│   ├── Character (attached to character entities)
│   ├── Name (Bevy's built- in component for character names)
│   ├── CharacterCard (data binding for UI)
│   ├── HoverState (interaction tracking)
│   └── InputText (text field management)
├── Systems
│   ├── setup_selection_ui (Selection phase UI spawning)
│   ├── setup_naming_ui (Naming phase UI spawning)
│   ├── handle_character_selection (card interactions)
│   ├── handle_naming_input (keyboard processing)
│   ├── update_card_hover_effects (visual feedback)
│   └── setup_character_in_guild_house (parent character to guild house arena)
└── Entities
└── Character Entity (spawned with Character + Name components, persists across states)
```

**Active Recall Checkpoint**: Before continuing, explain in your own words how Bevy's ECS pattern separates data (
Components/Resources) from behavior (Systems). How does this separation benefit our character creation system?

### Memory Management and Performance Considerations

**Entity Spawning Strategy**: The current implementation spawns/despawns entire UI hierarchies on state transitions.
This is actually optimal for Bevy's ECS—entity creation/destruction is highly optimized compared to component
modification. However, consider these production patterns:

```rust
// Production pattern: Batch entity operations for better performance
fn cleanup_character_create(
    mut commands: Commands,
    query: Query<Entity, With<CharacterCreateScreen>>,
) {
    // Collect entities first, then despawn in batch
    let entities: Vec<Entity> = query.iter().collect();
    for entity in entities {
        commands.entity(entity).despawn_recursive(); // Use despawn_recursive for hierarchies
    }
}
```

**Query Performance**: The hover effects system uses `Changed<Interaction>` which is excellent. In production, always
prefer change detection over polling:

```rust
// ❌ BAD: Polling approach - runs every frame even when nothing changes
fn update_card_hover_effects_polling(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut HoverState),
        With<SelectableCard> // No Changed<> filter - processes ALL cards every frame
    >,
) {
    // This system processes every card every frame (60+ times per second)
    // even when no interactions have changed - wasteful and hurts performance
    for (interaction, mut bg_color, mut hover_state) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                if !hover_state.is_hovered {
                    *bg_color = BackgroundColor(Color::srgb(1.0, 1.0, 1.0));
                    hover_state.is_hovered = true;
                }
            }
            Interaction::None => {
                if hover_state.is_hovered {
                    *bg_color = BackgroundColor(Color::srgb(0.92, 0.92, 0.92));
                    hover_state.is_hovered = false;
                }
            }
            _ => {}
        }
    }
}

// ✅ GOOD: Change detection approach - only runs when interactions actually change
fn update_card_hover_effects(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut HoverState),
        (Changed<Interaction>, With<SelectableCard>, Without<Pressed>) // Only processes changed entities
    >,
) {
    // This system only processes cards when their Interaction component changes
    // Dramatically reduces CPU usage, especially with many UI elements
    for (interaction, mut bg_color, mut hover_state) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                if !hover_state.is_hovered {
                    *bg_color = BackgroundColor(Color::srgb(1.0, 1.0, 1.0));
                    hover_state.is_hovered = true;
                }
            }
            Interaction::None => {
                if hover_state.is_hovered {
                    *bg_color = BackgroundColor(Color::srgb(0.92, 0.92, 0.92));
                    hover_state.is_hovered = false;
                }
            }
            _ => {}
        }
    }
}

// Production insight: Consider adding Without<> filters for complex scenes
fn update_card_hover_effects_optimized(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut HoverState),
        (Changed<Interaction>, With<SelectableCard>, Without<Pressed>) // Avoid processed entities
    >,
) {
    // System implementation remains the same, but query is more efficient
}
```

### Error Handling in Production

The current implementation assumes happy paths. In production games, you need defensive programming:

```rust
/// Production version with error handling
fn spawn_character_card(
    parent: &mut ChildSpawnerCommands,
    class: CharacterClass,
    asset_server: &AssetServer,
) {
    let texture_handle = asset_server.load(class.texture_path());

    // In production, validate asset existence or provide fallbacks
    parent.spawn((
        Button,
        Node {
            width: Val::Px(200.0),
            height: Val::Px(160.0),
            // ... rest of node config
        },
        BackgroundColor(Color::srgb(0.92, 0.92, 0.92)),
        BorderColor(Color::WHITE),
        HoverState { is_hovered: false },
        CharacterCard { class },
        SelectableCard,
        // Production: Add error recovery component
        AssetLoadState::Loading, // Track loading state for better UX
    )).with_children(|card| {
        // Spawn with error handling
        card.spawn((
            ImageNode::new(texture_handle),
            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                margin: UiRect::bottom(Val::Px(8.0)),
                ..default()
            },
            // Production: Handle missing textures gracefully
            ImageFallback(asset_server.load("ui/missing_character.png")),
        ));
        // ... rest of card children
    });
}

#[derive(Component)]
enum AssetLoadState {
    Loading,
    Loaded,
    Failed(String),
}

#[derive(Component)]
struct ImageFallback(Handle<Image>);
```

### Nested State Architecture Benefits

The nested enum approach `GameState::CharacterCreate(CharacterPhase)` eliminates the need for manual resource lifecycle
management:

```rust
// OLD APPROACH (Separate Resource) - Complex lifecycle management needed
#[derive(Resource)]
struct CharacterCreationState {
    phase: CreationPhase,
    character_name: String,
}

// NEW APPROACH (Nested States) - Automatic lifecycle management
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    CharacterCreate(CharacterPhase),
    Intro,
    Battle,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CharacterPhase {
    Selection,
    Naming(CharacterClass),
}

// Minimal resource for temporary input only
#[derive(Resource, Default)]
struct InputBuffer {
    character_name: String,
}
```

**Key Benefits**:

- **Unified State Management**: Single source of truth eliminates synchronization issues
- **Automatic Transitions**: Bevy handles UI cleanup and setup without manual intervention
- **Type Safety**: Compile-time guarantees prevent invalid state combinations
- **Reduced Complexity**: No manual resource lifecycle management needed

### Input Handling Robustness

The current keyboard input handling is solid but needs production hardening:

```rust
/// Production-hardened input handling with validation and edge case handling
fn handle_naming_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut creation_state: ResMut<CharacterCreationState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut input_text_query: Query<&mut Text, With<InputText>>,
) {
    if let CreationPhase::Naming(_selected_class) = creation_state.phase {
        // Production: Rate limiting for input events (prevent input flooding)
        let mut input_events_this_frame = 0;
        const MAX_INPUT_EVENTS_PER_FRAME: usize = 10;

        for event in keyboard_events.read() {
            if input_events_this_frame >= MAX_INPUT_EVENTS_PER_FRAME {
                warn!("Input event rate limit exceeded, dropping events");
                break;
            }

            if let KeyboardInput { logical_key: Key::Character(ch), state: ButtonState::Pressed, .. } = event {
                // Production: Better character validation
                if let Some(ch) = ch.chars().next() {
                    if is_valid_name_character(ch) && creation_state.character_name.len() < MAX_NAME_LENGTH {
                        creation_state.character_name.push(ch);
                        update_input_display(&mut input_text_query, &creation_state);
                        input_events_this_frame += 1;
                    }
                }
            }
        }

        // Production: Add escape key handling for better UX
        if keyboard.just_pressed(KeyCode::Escape) {
            // Return to selection phase
            creation_state.phase = CreationPhase::Selection;
            creation_state.character_name.clear();

            // Respawn selection UI (in production, consider state preservation)
            for entity in input_text_query.iter() {
                if let Ok(entity) = query.get_entity(entity) {
                    commands.entity(entity).despawn_recursive();
                }
            }
            // Setup selection UI again...
        }

        // Rest of input handling...
    }
}

const MAX_NAME_LENGTH: usize = 20;

/// Production: More sophisticated character validation
fn is_valid_name_character(ch: char) -> bool {
    ch.is_alphanumeric()
        || ch == ' '
        || ch == '-'
        || ch == '\''
        || ch == '.'
        // Add Unicode letter support for international names
        || ch.is_alphabetic()
}
```

### Asset Pipeline Integration

In production, character assets need proper management:

```rust
impl CharacterClass {
    /// Production: Structured asset organization
    pub fn texture_path(self) -> &'static str {
        match self {
            Self::Trapper => "characters/portraits/trapper.png",      // More organized path
            Self::Alchemist => "characters/portraits/alchemist.png",
            // ... consistent structure for all classes
        }
    }

    /// Production: Multiple asset types per character
    pub fn model_path(self) -> &'static str {
        match self {
            Self::Trapper => "characters/models/trapper.glb",
            Self::Alchemist => "characters/models/alchemist.glb",
            // ... 3D models for gameplay
        }
    }

    /// Production: Animation asset management
    pub fn animation_path(self) -> &'static str {
        match self {
            Self::Trapper => "characters/animations/trapper_idle.ron",
            Self::Alchemist => "characters/animations/alchemist_idle.ron",
            // ... idle animations for character preview
        }
    }

    /// Production: Sound integration
    pub fn selection_sound(self) -> &'static str {
        match self {
            Self::Trapper => "audio/character_select/trapper.ogg",
            Self::Alchemist => "audio/character_select/alchemist.ogg",
            // ... audio feedback for selection
        }
    }
}
```

### System Ordering and Scheduling

The current system registration is clean but lacks production scheduling considerations:

```rust
impl Plugin for CharacterCreatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CharacterCreationState>()
            .add_systems(OnEnter(GameState::CharacterCreate), setup_character_create)
            .add_systems(
                Update,
                (
                    // Production: Explicit system ordering prevents frame delay issues
                    handle_character_selection.before(update_card_hover_effects),
                    handle_naming_input.after(handle_character_selection),
                    update_card_hover_effects.after(handle_character_selection),

                    // Production: Add asset loading system
                    update_asset_loading_states,

                    // Production: Add analytics system
                    track_character_selection_metrics.after(handle_character_selection),
                ).run_if(in_state(GameState::CharacterCreate))
            )
            .add_systems(OnExit(GameState::CharacterCreate), cleanup_character_create)

            // Production: Add asset preloading system
            .add_systems(OnEnter(GameState::Loading), preload_character_assets);
    }
}

/// Production: Asset preloading prevents hitches during character selection
fn preload_character_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut handles = Vec::new();

    for class in CharacterClass::all() {
        handles.push(asset_server.load(class.texture_path()));
        handles.push(asset_server.load(class.model_path()));
        handles.push(asset_server.load(class.animation_path()));
        handles.push(asset_server.load(class.selection_sound()));
    }

    // Store handles to prevent unloading
    commands.insert_resource(PreloadedCharacterAssets { handles });
}

#[derive(Resource)]
struct PreloadedCharacterAssets {
    handles: Vec<UntypedHandle>,
}
```

### Platform and Accessibility Considerations

Production games need platform-specific adaptations:

```rust
/// Production: Platform-aware UI scaling
fn setup_character_create(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut creation_state: ResMut<CharacterCreationState>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();

    // Production: Responsive design based on screen size
    let (card_width, card_height, grid_columns) = if window.width() < 800.0 {
        // Mobile/small screen layout
        (Val::Px(150.0), Val::Px(120.0), 2)
    } else {
        // Desktop layout
        (Val::Px(200.0), Val::Px(160.0), 4)
    };

    // Production: Accessibility - high contrast mode support
    let background_color = if is_high_contrast_mode() {
        Color::BLACK // High contrast background
    } else {
        Color::srgb_u8(227, 51, 75) // Normal theme
    };

    // ... rest of setup with responsive values
}

/// Production: Accessibility helper
fn is_high_contrast_mode() -> bool {
    // Check system settings or game preferences
    false // Placeholder implementation
}

/// Production: Gamepad support for character selection
fn handle_gamepad_navigation(
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    mut current_selection: Local<usize>,
    mut creation_state: ResMut<CharacterCreationState>,
) {
    if matches!(creation_state.phase, CreationPhase::Selection) {
        for gamepad in gamepads.iter() {
            // Handle D-pad navigation
            if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadRight)) {
                *current_selection = (*current_selection + 1).min(7);
            }
            if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadLeft)) {
                *current_selection = current_selection.saturating_sub(1);
            }
            // ... additional navigation logic
        }
    }
}
```

---






---



**Automatic Cleanup Benefits**: Bevy's state system automatically calls cleanup systems when exiting states, ensuring no
memory leaks.

**Simplified Resource Management**: Only the minimal `InputBuffer` needs cleanup - no complex state resource lifecycle
management.

**Marker Component Strategy**: Using `CharacterCreateScreen` as a marker component enables efficient bulk cleanup with a
single query.

🧪 **Validation Tests**

After implementing cleanup and resource management, validate proper entity lifecycle:

```rust
#[cfg(test)]
mod cleanup_tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn cleanup_removes_all_ui_entities() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.init_resource::<InputBuffer>();

        // Spawn some UI entities with CharacterCreateScreen marker
        let ui_entities = vec![
            app.world_mut().spawn(CharacterCreateScreen).id(),
            app.world_mut().spawn(CharacterCreateScreen).id(),
            app.world_mut().spawn(CharacterCreateScreen).id(),
        ];
        app.update();

        // Verify entities exist
        let count_before = app.world()
            .query::<&CharacterCreateScreen>()
            .iter(app.world())
            .count();
        assert_eq!(count_before, 3, "Should have 3 UI entities before cleanup");

        // Run cleanup
        cleanup_character_create(
            app.world_mut().commands(),
            app.world().query::<Entity>(),
            app.world().query::<Entity>(),
            app.world_mut().resource_mut::<InputBuffer>(),
        );
        app.update();

        // Verify entities were removed
        let count_after = app.world()
            .query::<&CharacterCreateScreen>()
            .iter(app.world())
            .count();
        assert_eq!(count_after, 0, "Should have no UI entities after cleanup");
    }

    #[test]
    fn cleanup_preserves_character_entities() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.init_resource::<InputBuffer>();

        // Spawn UI entities and character entities
        app.world_mut().spawn(CharacterCreateScreen);
        let character_entity = app.world_mut().spawn((
            Character { class: CharacterClass::Tank },
            Name::new("TestHero".to_string()),
            CharacterEntity,
        )).id();
        app.update();

        // Verify both types exist
        let ui_count_before = app.world()
            .query::<&CharacterCreateScreen>()
            .iter(app.world())
            .count();
        let character_count_before = app.world()
            .query::<(&Character, &Name)>()
            .iter(app.world())
            .count();

        assert_eq!(ui_count_before, 1);
        assert_eq!(character_count_before, 1);

        // Run cleanup
        cleanup_character_create(
            app.world_mut().commands(),
            app.world().query::<Entity>(),
            app.world().query::<Entity>(),
            app.world_mut().resource_mut::<InputBuffer>(),
        );
        app.update();

        // Verify UI removed but character preserved
        let ui_count_after = app.world()
            .query::<&CharacterCreateScreen>()
            .iter(app.world())
            .count();
        let character_count_after = app.world()
            .query::<(&Character, &Name)>()
            .iter(app.world())
            .count();

        assert_eq!(ui_count_after, 0, "UI entities should be removed");
        assert_eq!(character_count_after, 1, "Character entities should be preserved");
    }

    #[test]
    fn input_buffer_is_cleared() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        // Set up input buffer with data
        app.world_mut().insert_resource(InputBuffer {
            character_name: "SomeInput".to_string(),
        });

        // Verify buffer has data
        let buffer_before = app.world().resource::<InputBuffer>();
        assert!(!buffer_before.character_name.is_empty());

        // Run cleanup
        cleanup_character_create(
            app.world_mut().commands(),
            app.world().query::<Entity>(),
            app.world().query::<Entity>(),
            app.world_mut().resource_mut::<InputBuffer>(),
        );
        app.update();

        // Verify buffer is cleared
        let buffer_after = app.world().resource::<InputBuffer>();
        assert!(buffer_after.character_name.is_empty(), "Input buffer should be cleared");
    }

    #[test]
    fn character_setup_in_guild_house() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        // Spawn guild house and character entities
        let guild_house = app.world_mut().spawn((
            GuildHouseArena,
            Transform::default(),
            GlobalTransform::default(),
        )).id();

        let character_entity = app.world_mut().spawn((
            Character { class: CharacterClass::Cardinal },
            Name::new("GuildHero".to_string()),
            CharacterEntity,
        )).id();
        app.update();

        // Verify character has no parent initially
        assert!(app.world().get::<Parent>(character_entity).is_none());

        // Run guild house setup
        setup_character_in_guild_house(
            app.world_mut().commands(),
            app.world().query::<Entity>(),
            app.world().query::<Entity>(),
        );
        app.update();

        // Verify character is now parented to guild house
        let parent = app.world().get::<Parent>(character_entity);
        assert!(parent.is_some(), "Character should have parent after guild house setup");
        assert_eq!(parent.unwrap().get(), guild_house, "Character should be parented to guild house");

        // Verify character has transform components
        assert!(app.world().get::<Transform>(character_entity).is_some());
        assert!(app.world().get::<PlayerControlled>(character_entity).is_some());
    }

    #[test]
    fn cleanup_handles_hierarchical_ui() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.init_resource::<InputBuffer>();

        // Create parent-child UI hierarchy
        let parent_ui = app.world_mut().spawn(CharacterCreateScreen).id();
        let child_ui = app.world_mut().spawn((
            CharacterCreateScreen,
            Parent(parent_ui),
        )).id();

        // Add child to parent's children list
        app.world_mut().entity_mut(parent_ui).insert(Children::from(&[child_ui]));
        app.update();

        // Verify hierarchy exists
        let ui_count_before = app.world()
            .query::<&CharacterCreateScreen>()
            .iter(app.world())
            .count();
        assert_eq!(ui_count_before, 2);

        // Run cleanup (should use despawn_recursive)
        cleanup_character_create(
            app.world_mut().commands(),
            app.world().query::<Entity>(),
            app.world().query::<Entity>(),
            app.world_mut().resource_mut::<InputBuffer>(),
        );
        app.update();

        // Verify all UI entities removed
        let ui_count_after = app.world()
            .query::<&CharacterCreateScreen>()
            .iter(app.world())
            .count();
        assert_eq!(ui_count_after, 0, "All UI entities should be removed including children");
    }
}
```

**How to Run These Tests:**

```bash
cargo test cleanup_tests
```

**What Success Looks Like:**

- All UI entities with CharacterCreateScreen marker are removed
- Character entities with Character + Name components are preserved
- InputBuffer resource is properly cleared
- Character entities get parented to guild house with additional components
- Hierarchical UI cleanup works correctly (no orphaned child entities)

**Memory Leak Detection Test:**

```rust
#[cfg(test)]
mod memory_leak_tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn no_entities_leak_across_multiple_cycles() {
        let mut app = App::new();
        app.add_plugins((DefaultPlugins, CharacterCreatePlugin));

        let initial_entity_count = app.world().entities().len();

        // Run multiple character creation cycles
        for cycle in 0..5 {
            // Enter character creation
            app.world_mut().resource_mut::<NextState<GameState>>().set(
                GameState::CharacterCreate(CharacterPhase::Selection)
            );
            app.update();

            // Create some UI (simulated)
            app.world_mut().spawn(CharacterCreateScreen);
            app.world_mut().spawn(CharacterCreateScreen);
            app.update();

            // Exit character creation
            app.world_mut().resource_mut::<NextState<GameState>>().set(GameState::Intro);
            app.update();
        }

        let final_entity_count = app.world().entities().len();

        // Should not have significantly more entities (allowing for some system entities)
        let entity_growth = final_entity_count - initial_entity_count;
        assert!(entity_growth < 10,
                "Entity count grew by {} after {} cycles, possible memory leak",
                entity_growth, 5);
    }

    #[test]
    fn character_entities_persist_across_state_changes() {
        let mut app = App::new();
        app.add_plugins((DefaultPlugins, CharacterCreatePlugin));

        // Create character entity
        let character_id = app.world_mut().spawn((
            Character { class: CharacterClass::Sprinter },
            Name::new("PersistentHero".to_string()),
            CharacterEntity,
        )).id();
        app.update();

        // Go through multiple state changes
        let states = vec![
            GameState::CharacterCreate(CharacterPhase::Selection),
            GameState::CharacterCreate(CharacterPhase::Naming(CharacterClass::Sprinter)),
            GameState::Intro,
            GameState::Battle,
            GameState::MainMenu,
        ];

        for state in states {
            app.world_mut().resource_mut::<NextState<GameState>>().set(state);
            app.update();

            // Verify character entity still exists
            assert!(app.world().get_entity(character_id).is_some(),
                    "Character entity should persist across state changes");

            let character = app.world().get::<Character>(character_id).unwrap();
            assert_eq!(character.class, CharacterClass::Sprinter);

            let name = app.world().get::<Name>(character_id).unwrap();
            assert_eq!(name.as_str(), "PersistentHero");
        }
    }
}
```

**Common Issues These Tests Catch:**

- UI entities not being properly cleaned up (memory leaks)
- Character entities being accidentally removed during cleanup
- InputBuffer not being cleared between sessions
- Parent-child relationships not being established correctly
- despawn_recursive not being used for hierarchical UI
- Entity references becoming invalid after cleanup

---

## Testing Strategy and Quality Assurance

Marcus's testing approach ensures system reliability and maintainability:

### Unit Tests for Data Integrity

```rust
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
```

### Nested State Management Tests

```rust
#[test]
fn input_buffer_default() {
    let buffer = InputBuffer::default();
    assert!(buffer.character_name.is_empty());
}

#[test]
fn nested_state_transitions() {
    let selection_state = GameState::CharacterCreate(CharacterPhase::Selection);
    let naming_state = GameState::CharacterCreate(CharacterPhase::Naming(CharacterClass::Trapper));

    assert_ne!(selection_state, naming_state);

    if let GameState::CharacterCreate(CharacterPhase::Naming(class)) = naming_state {
        assert_eq!(class, CharacterClass::Trapper);
    } else {
        panic!("Expected CharacterCreate Naming state with Trapper class");
    }
}

#[test]
fn state_type_safety() {
    // Compiler ensures only valid state combinations
    let valid_states = vec![
        GameState::MainMenu,
        GameState::CharacterCreate(CharacterPhase::Selection),
        GameState::CharacterCreate(CharacterPhase::Naming(CharacterClass::Alchemist)),
        GameState::Intro,
    ];

    // These would not compile - invalid nested states:
    // GameState::MainMenu(CharacterPhase::Selection) // ❌ Compile error
    // GameState::Intro(CharacterPhase::Naming) // ❌ Compile error
}
```

### System Registration Tests

```rust
#[test]
fn character_create_plugin_registers_systems() {
    let mut app = App::new();
    app.add_plugins(CharacterCreatePlugin);

    // Verify the InputBuffer resource is initialized
    assert!(app.world().contains_resource::<InputBuffer>());

    // Verify nested state is properly configured
    // (Bevy's state system automatically handles state registration)
}







```

**Testing Philosophy**: Each test validates a specific assumption about system behavior. As the system evolves, these
tests catch regressions and ensure continued reliability.

### Jon's Production Testing Additions

**Performance and Memory Tests**: Production games need performance validation, not just correctness tests:

```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    use bevy::time::Time;
    use std::time::Duration;

    #[test]
    fn character_creation_ui_spawns_within_frame_budget() {
        let mut app = App::new();
        app.add_plugins((DefaultPlugins, CharacterCreatePlugin));

        // Simulate entering character creation state
        app.world_mut().resource_mut::<NextState<GameState>>().set(GameState::CharacterCreate);

        let start = std::time::Instant::now();
        app.update(); // This should include OnEnter systems
        let elapsed = start.elapsed();

        // Production constraint: UI setup must complete within one frame (16.67ms at 60fps)
        assert!(elapsed < Duration::from_millis(16),
                "Character creation setup took {}ms, exceeding frame budget", elapsed.as_millis());
    }

    #[test]
    fn memory_cleanup_after_state_exit() {
        let mut app = App::new();
        app.add_plugins((DefaultPlugins, CharacterCreatePlugin));

        // Enter and exit character creation multiple times
        for _ in 0..5 {
            app.world_mut().resource_mut::<NextState<GameState>>().set(GameState::CharacterCreate);
            app.update();

            app.world_mut().resource_mut::<NextState<GameState>>().set(GameState::Intro);
            app.update();
        }

        // Verify no character creation entities remain
        let remaining_entities: Vec<_> = app.world()
            .query::<Entity>()
            .iter(&app.world())
            .collect();

        // Should only have system entities, not UI entities
        assert!(remaining_entities.len() < 10,
                "Memory leak detected: {} entities remaining after cleanup", remaining_entities.len());
    }
}
```

**Integration Tests for Production Scenarios**:

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn complete_character_creation_flow() {
        let mut app = App::new();
        app.add_plugins((DefaultPlugins, CharacterCreatePlugin));

        // Enter character creation
        app.world_mut().resource_mut::<NextState<GameState>>().set(GameState::CharacterCreate);
        app.update();

        // Simulate card selection
        let mut creation_state = app.world_mut().resource_mut::<CharacterCreationState>();
        creation_state.phase = CreationPhase::Naming(CharacterClass::Trapper);
        creation_state.character_name = "TestHero".to_string();

        // Simulate enter key press to complete creation
        let mut keyboard = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        keyboard.press(KeyCode::Enter);
        app.update();
        keyboard.release(KeyCode::Enter);

        // Verify character entity was created
        let character_entities: Vec<_> = app.world()
            .query::<(&Character, &Name)>()
            .iter(&app.world())
            .collect();

        assert_eq!(character_entities.len(), 1, "Should have exactly one character entity");
        let (character, name) = character_entities[0];
        assert_eq!(character.class, CharacterClass::Trapper);
        assert_eq!(name.as_str(), "TestHero");
    }

    #[test]
    fn asset_loading_resilience() {
        let mut app = App::new();
        app.add_plugins((DefaultPlugins, CharacterCreatePlugin));

        // Test that system doesn't crash with missing assets
        // (In production, you'd mock the AssetServer)
        app.world_mut().resource_mut::<NextState<GameState>>().set(GameState::CharacterCreate);

        // This shouldn't panic even with missing texture files
        app.update();

        // Verify UI still spawned despite missing assets
        let ui_entities = app.world().query::<&CharacterCreateScreen>().iter(&app.world()).count();
        assert!(ui_entities > 0, "UI should spawn even with missing assets");
    }
}
```

**Property-Based Testing for Input Validation**:

```rust
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn name_validation_handles_arbitrary_input(name in ".*") {
            // Test that name validation never panics with any Unicode input
            let filtered: String = name.chars()
                .filter(|&c| is_valid_name_character(c))
                .take(MAX_NAME_LENGTH)
                .collect();
            
            // Should never panic or produce invalid strings
            assert!(filtered.len() <= MAX_NAME_LENGTH);
            assert!(filtered.chars().all(is_valid_name_character));
        }
        
        #[test]
        fn character_class_methods_never_panic(class_index in 0usize..8) {
            let class = CharacterClass::all()[class_index];
            
            // All methods should return valid data without panicking
            let _ = class.display_name();
            let _ = class.tagline();
            let _ = class.texture_path();
            
            // Verify paths are well-formed
            assert!(class.texture_path().ends_with(".png"));
            assert!(!class.display_name().is_empty());
            assert!(!class.tagline().is_empty());
        }
    }
}
```

**Benchmark Tests for Performance-Critical Systems**:

```rust
#[cfg(test)]
mod benchmarks {
    use super::*;
    use criterion::{black_box, Criterion};

    pub fn bench_hover_effects_system(c: &mut Criterion) {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        // Spawn many cards to stress test the hover system
        for i in 0..1000 {
            app.world_mut().spawn((
                Button,
                Interaction::None,
                BackgroundColor(Color::WHITE),
                HoverState { is_hovered: false },
                SelectableCard,
            ));
        }

        c.bench_function("hover_effects_1000_cards", |b| {
            b.iter(|| {
                // Simulate hover events on random cards
                update_card_hover_effects(black_box(
                    app.world_mut().query_filtered::<
                        (&Interaction, &mut BackgroundColor, &mut HoverState),
                        (Changed<Interaction>, With<SelectableCard>)
                    >()
                ));
            })
        });
    }
}
```