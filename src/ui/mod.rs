use bevy::prelude::*;

// Module declarations
pub mod styles_config;
pub mod button;
pub mod input_field;
pub mod tile;
pub mod typography;

// Re-exports for easier importing
pub use styles_config::*;
pub use button::*;
pub use input_field::*;
pub use tile::*;
// pub use typography::*; // Temporarily commented out to avoid unused warnings

/// UI Components Plugin
/// 
/// This plugin registers all UI systems and components needed for the
/// character creation screen and other UI elements throughout the game.
pub struct UiComponentsPlugin;

impl Plugin for UiComponentsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add button interaction systems
            .add_systems(Update, (
                button_interaction_system,
                input_field_interaction_system,
                tile_interaction_system,
            ))
            // TODO: Add other UI systems as needed
            ;
    }
}

/// Marker trait for UI components that can be styled
pub trait Styleable {
    fn apply_theme(&mut self, theme: &UiTheme);
}

/// Theme configuration for consistent UI styling
#[derive(Resource, Debug, Clone)]
pub struct UiTheme {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub background_color: Color,
    pub text_color: Color,
    pub accent_color: Color,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            primary_color: Colors::PRIMARY,
            secondary_color: Colors::SECONDARY,
            background_color: Colors::WHITE,
            text_color: Colors::GRAY_900,
            accent_color: Colors::INFO,
        }
    }
}

/// Common UI layout helpers
pub struct Layout;

impl Layout {
    /// Creates a centered container with max width
    pub fn centered_container() -> Node {
        Node {
            width: Val::Percent(100.0),
            max_width: Grid::CONTAINER_MAX_WIDTH,
            margin: UiRect::horizontal(Val::Auto),
            padding: UiRect::all(Spacing::LG),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        }
    }
    
    /// Creates a flex row container
    pub fn flex_row() -> Node {
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Spacing::MD,
            ..default()
        }
    }
    
    /// Creates a flex column container
    pub fn flex_column() -> Node {
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: Spacing::MD,
            ..default()
        }
    }
    
    /// Creates a card container with shadow and border
    pub fn card() -> (Node, BackgroundColor, BorderColor) {
        (
            Node {
                padding: UiRect::all(Spacing::LG),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Colors::WHITE),
            BorderColor(Colors::GRAY_200),
        )
    }
    
    /// Creates a character selection grid (4x2 for 8 classes)
    pub fn character_selection_grid() -> Node {
        TileGrid::character_selection().create_grid_container()
    }
}

/// Spacing utilities for consistent layouts
pub struct SpaceUtils;

impl SpaceUtils {
    /// Creates a vertical spacer
    pub fn vertical_spacer(height: Val) -> Node {
        Node {
            width: Val::Percent(100.0),
            height,
            ..default()
        }
    }
    
    /// Creates a horizontal spacer
    pub fn horizontal_spacer(width: Val) -> Node {
        Node {
            width,
            height: Val::Percent(100.0),
            ..default()
        }
    }
}

/// Animation helpers (stubs for future implementation)
pub struct Animations;

impl Animations {
    /// Fade in animation component
    pub fn fade_in(duration: f32) -> AnimationConfig {
        AnimationConfig {
            duration,
            animation_type: AnimationType::FadeIn,
        }
    }
    
    /// Slide in animation component
    pub fn slide_in(duration: f32, direction: SlideDirection) -> AnimationConfig {
        AnimationConfig {
            duration,
            animation_type: AnimationType::SlideIn(direction),
        }
    }
}

#[derive(Component, Debug)]
pub struct AnimationConfig {
    pub duration: f32,
    pub animation_type: AnimationType,
}

#[derive(Debug)]
pub enum AnimationType {
    FadeIn,
    FadeOut,
    SlideIn(SlideDirection),
    SlideOut(SlideDirection),
    Scale(f32),
}

#[derive(Debug)]
pub enum SlideDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Responsive design helpers (stubs for future implementation)
pub struct Responsive;

impl Responsive {
    /// Check if screen width is below mobile breakpoint
    pub fn is_mobile(window_width: f32) -> bool {
        window_width < Breakpoints::SM
    }
    
    /// Check if screen width is below tablet breakpoint  
    pub fn is_tablet(window_width: f32) -> bool {
        window_width < Breakpoints::MD
    }
    
    /// Check if screen width is below desktop breakpoint
    pub fn is_desktop(window_width: f32) -> bool {
        window_width >= Breakpoints::LG
    }
}

/// Accessibility helpers (stubs for future implementation)
pub struct A11y;

impl A11y {
    /// Creates an accessible button with proper ARIA attributes
    /// TODO: Implement when Bevy accessibility APIs are available
    pub fn accessible_button(_label: &str) {
        // Placeholder for future accessibility implementation
    }
    
    /// Creates an accessible input field with label
    /// TODO: Implement when Bevy accessibility APIs are available
    pub fn accessible_input(_label: &str, _placeholder: &str) {
        // Placeholder for future accessibility implementation
    }
}

/// Common UI events for inter-component communication
#[derive(Event, Debug)]
pub enum UiEvent {
    ButtonClicked { button_id: String },
    InputChanged { input_id: String, value: String },
    TileSelected { tile_id: String },
    CharacterSelected { character_class: String },
    NavigationRequested { target: String },
}

/// UI event handling system (stub for tutorial)
pub fn ui_event_system(
    mut ui_events: EventReader<UiEvent>,
    // TODO: Add other system parameters as needed
) {
    for event in ui_events.read() {
        match event {
            UiEvent::ButtonClicked { button_id } => {
                // TODO: Handle button click events
                println!("Button clicked: {}", button_id);
            }
            UiEvent::InputChanged { input_id, value } => {
                // TODO: Handle input change events
                println!("Input changed: {} = {}", input_id, value);
            }
            UiEvent::TileSelected { tile_id } => {
                // TODO: Handle tile selection events
                println!("Tile selected: {}", tile_id);
            }
            UiEvent::CharacterSelected { character_class } => {
                // TODO: Handle character selection events
                println!("Character selected: {}", character_class);
            }
            UiEvent::NavigationRequested { target } => {
                // TODO: Handle navigation events
                println!("Navigation requested: {}", target);
            }
        }
    }
}