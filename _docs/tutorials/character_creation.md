# Building Character Creation Systems in Bevy

**What You'll Build**: A complete character creation system featuring 8 character classes, interactive card selection,
character naming, and seamless state transitions—all architected for maintainability and extensibility using a
modern component-based UI system.

> **Architecture Focus**: This tutorial demonstrates how to build production-ready UI using reusable components
> with trait-based patterns. Instead of hardcoded UI creation, you'll learn to use a composable component system
> that promotes consistency, maintainability, and team scalability. The UI components (`Button`, `Tile`, `Typography`,
> `InputField`) use traits to define behavior while maintaining type safety and design system compliance.


---

## Typography System Integration

The character creation screen uses Bevy's typography system to maintain consistent text styling across all UI elements.
The typography system provides trait-based components that ensure design system compliance while maintaining type safety
and performance.

### Basic Typography Usage

The typography system provides three main methods for creating text entities:

#### 1. Simple Text Creation

```rust
fn setup_character_create(mut commands: Commands) {
    // Create a display title
    let display_typography = DisplayText;
    let (text, font, color) = display_typography.create_text("Choose Your Class");

    commands.spawn((
        text,
        font,
        color,
        CharacterCreateScreen,
    ));
}
```

#### 2. Text with Layout Control

```rust
fn spawn_centered_heading(commands: &mut Commands) {
    let heading = Heading1;
    let (text, font, color, layout) = heading.create_text_with_layout(
        "Character Creation",
        JustifyText::Center
    );

    commands.spawn((
        text,
        font,
        color,
        layout,
        CharacterCreateScreen,
    ));
}
```

#### 3. Component-Based Text

```rust
fn spawn_character_card_text(commands: &mut Commands, class_name: &str, description: &str) {
    // Character class name
    let class_typography = CharacterClassName;
    let (class_text, class_font, class_color) = class_typography.create_text(class_name);

    // Skill description
    let desc_typography = CharacterSkillDescription;
    let (desc_text, desc_font, desc_color) = desc_typography.create_text(description);

    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Spacing::XS,
                ..default()
            },
            CharacterCreateScreen,
        ))
        .with_children(|parent| {
            // Class name
            parent.spawn((
                class_text,
                class_font,
                class_color,
            ));

            // Description
            parent.spawn((
                desc_text,
                desc_font,
                desc_color,
            ));
        });
}
```

### Complete Character Creation Example

Here's a complete example showing how to integrate typography into the character creation screen:

```rust
use super::GameState;
use crate::character::Character;
use crate::ui::typography::{DisplayText, Heading2, CharacterClassName, CharacterSkillDescription, Typography};
use crate::ui::{Colors, Spacing, Layout};
use bevy::prelude::*;

fn setup_character_create(mut commands: Commands) {
    let container = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            CharacterCreateScreen,
        ))
        .id();

    // Main title
    let title_typography = DisplayText;
    let (title_text, title_font, title_color) = title_typography.create_text_with_layout(
        "Choose Your Class",
        JustifyText::Center
    );

    let title_entity = commands.spawn((
        title_text,
        title_font,
        title_color,
        Node {
            margin: UiRect::bottom(Spacing::XL),
            ..default()
        },
        CharacterCreateScreen,
    )).id();

    // Subtitle
    let subtitle_typography = Heading2;
    let (subtitle_text, subtitle_font, subtitle_color) = subtitle_typography.create_text_with_layout(
        "Select a character class to begin your adventure",
        JustifyText::Center
    );

    let subtitle_entity = commands.spawn((
        subtitle_text,
        subtitle_font,
        subtitle_color,
        Node {
            margin: UiRect::bottom(Spacing::XXL),
            ..default()
        },
        CharacterCreateScreen,
    )).id();

    // Character class grid
    let grid_entity = spawn_character_class_grid(&mut commands);

    // Parent all elements to the container
    commands.entity(container).push_children(&[
        title_entity,
        subtitle_entity,
        grid_entity,
    ]);
}

fn spawn_character_class_grid(commands: &mut Commands) -> Entity {
    let grid_container = commands
        .spawn((
            Node {
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
                column_gap: Spacing::MD,
                row_gap: Spacing::MD,
                ..default()
            },
            CharacterCreateScreen,
        ))
        .id();

    // Spawn character class cards
    for character_class in UICharacterClass::all() {
        let card_entity = spawn_character_card(commands, &character_class);
        commands.entity(grid_container).add_child(card_entity);
    }

    grid_container
}

fn spawn_character_card(commands: &mut Commands, character_class: &UICharacterClass) -> Entity {
    let class_name = character_class.class_name();
    let skill_description = get_character_skill_description(character_class);

    commands
        .spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Px(120.0),
                padding: UiRect::all(Spacing::MD),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Colors::WHITE),
            BorderColor(Colors::GRAY_300),
            Button,
            CharacterCard { character_class: character_class.clone() },
            CharacterCreateScreen,
        ))
        .with_children(|parent| {
            // Character class name
            let class_typography = CharacterClassName;
            let (class_text, class_font, class_color) = class_typography.create_text(class_name);

            parent.spawn((
                class_text,
                class_font,
                class_color,
            ));

            // Skill description
            let desc_typography = CharacterSkillDescription;
            let (desc_text, desc_font, desc_color) = desc_typography.create_text(&skill_description);

            parent.spawn((
                desc_text,
                desc_font,
                desc_color,
            ));
        })
        .id()
}

fn get_character_skill_description(character_class: &UICharacterClass) -> String {
    match character_class {
        UICharacterClass::Hunter(_) => "Eagle Eye precision targeting".to_string(),
        UICharacterClass::Bard(_) => "Inspiring melodies boost party".to_string(),
        UICharacterClass::Merchant(_) => "Trade mastery yields resources".to_string(),
        UICharacterClass::Warrior(_) => "Battle fury area attacks".to_string(),
        UICharacterClass::Cardinal(_) => "Divine grace heals allies".to_string(),
        UICharacterClass::Alchemist(_) => "Transmutation creates potions".to_string(),
        UICharacterClass::Forager(_) => "Nature's bounty finds resources".to_string(),
        UICharacterClass::Thief(_) => "Backstab positional attacks".to_string(),
    }
}

#[derive(Component)]
struct CharacterCard {
    character_class: UICharacterClass,
}
```

### Typography Best Practices

1. **Consistency**: Always use the predefined typography components rather than manual text styling
2. **Hierarchy**: Use appropriate typography variants to establish clear visual hierarchy
3. **Performance**: Typography components are optimized for Bevy's ECS system and reuse
4. **Maintainability**: Changes to the design system automatically update all typography usage
5. **Accessibility**: Typography components include proper contrast ratios and sizing

### Advanced Usage

For specialized typography needs, you can create custom typography components:

```rust
#[derive(Component, Debug)]
pub struct CharacterNameInput;

impl Typography for CharacterNameInput {
    const VARIANT: TypographyVariant = TypographyVariant::Body;

    fn font_size(&self) -> f32 { FontSizes::LG }
    fn color(&self) -> Color { Colors::GRAY_800 }
    fn weight(&self) -> FontWeight { FontWeight::Medium }

    // ... implement other required methods
}
```

This typography system ensures that all text in your character creation screen maintains visual consistency while
providing the flexibility needed for complex UI layouts.

---

## Mental Model: Character Creation as a State Machine

Before diving into code, establish this central concept: **Character creation is a finite state machine with two primary
states and multiple transition triggers**.

```
GameState::CharacterCreate(Selection) → [User Clicks Card] → GameState::CharacterCreate(Naming) → [User Presses Enter] → GameState::Intro
                    ↑                                                           ↓
                    └────────────── [User Presses Escape] ────────────────────┘
```

```bash
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
