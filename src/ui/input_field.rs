use bevy::prelude::*;
use crate::ui::styles_config::{Colors, Spacing, FontSizes, BorderRadius};

#[derive(Component, Debug, Clone)]
pub struct UiInputField {
    pub variant: InputVariant,
    pub size: InputSize,
    pub state: InputState,
    pub placeholder: String,
    pub value: String,
    pub max_length: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputVariant {
    Default,
    Search,
    Email,
    Password,
    Number,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputState {
    Normal,
    Focus,
    Error,
    Disabled,
}

pub trait InputField {
    const VARIANT: InputVariant;
    const SIZE: InputSize;
    
    fn background_color(&self, state: &InputState) -> Color;
    fn text_color(&self, state: &InputState) -> Color;
    fn border_color(&self, state: &InputState) -> Color;
    fn placeholder_color(&self, state: &InputState) -> Color;
    fn padding(&self) -> UiRect;
    fn font_size(&self) -> f32;
    fn border_width(&self) -> Val;
    fn border_radius(&self) -> Val;
    fn height(&self) -> Val;
    
    fn create_container(&self, state: &InputState) -> (Node, BackgroundColor, BorderColor) {
        (
            Node {
                width: Val::Percent(100.0),
                height: self.height(),
                padding: self.padding(),
                border: UiRect::all(self.border_width()),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(self.background_color(state)),
            BorderColor(self.border_color(state)),
        )
    }
    
    fn create_text_components(&self, text: &str, state: &InputState) -> (Text, TextFont, TextColor) {
        (
            Text::new(text),
            TextFont {
                font_size: self.font_size(),
                ..default()
            },
            TextColor(self.text_color(state)),
        )
    }
    
    fn create_placeholder_components(&self, placeholder: &str, state: &InputState) -> (Text, TextFont, TextColor) {
        (
            Text::new(placeholder),
            TextFont {
                font_size: self.font_size(),
                ..default()
            },
            TextColor(self.placeholder_color(state)),
        )
    }
}

// Default Input Field Implementation
#[derive(Component, Debug)]
pub struct DefaultInputField;

impl InputField for DefaultInputField {
    const VARIANT: InputVariant = InputVariant::Default;
    const SIZE: InputSize = InputSize::Medium;
    
    fn background_color(&self, state: &InputState) -> Color {
        match state {
            InputState::Normal => Colors::WHITE,
            InputState::Focus => Colors::WHITE,
            InputState::Error => Colors::WHITE,
            InputState::Disabled => Colors::GRAY_100,
        }
    }
    
    fn text_color(&self, state: &InputState) -> Color {
        match state {
            InputState::Disabled => Colors::GRAY_400,
            _ => Colors::GRAY_900,
        }
    }
    
    fn border_color(&self, state: &InputState) -> Color {
        match state {
            InputState::Normal => Colors::GRAY_300,
            InputState::Focus => Colors::PRIMARY,
            InputState::Error => Colors::ERROR,
            InputState::Disabled => Colors::GRAY_200,
        }
    }
    
    fn placeholder_color(&self, _state: &InputState) -> Color {
        Colors::GRAY_400
    }
    
    fn padding(&self) -> UiRect {
        UiRect::all(Spacing::MD)
    }
    
    fn font_size(&self) -> f32 {
        FontSizes::BASE
    }
    
    fn border_width(&self) -> Val {
        Val::Px(2.0)
    }
    
    fn border_radius(&self) -> Val {
        BorderRadius::MD
    }
    
    fn height(&self) -> Val {
        Val::Px(44.0)
    }
}

// Search Input Field Implementation
#[derive(Component, Debug)]
pub struct SearchInputField;

impl InputField for SearchInputField {
    const VARIANT: InputVariant = InputVariant::Search;
    const SIZE: InputSize = InputSize::Medium;
    
    fn background_color(&self, state: &InputState) -> Color {
        match state {
            InputState::Normal => Colors::GRAY_100,
            InputState::Focus => Colors::WHITE,
            InputState::Error => Colors::WHITE,
            InputState::Disabled => Colors::GRAY_100,
        }
    }
    
    fn text_color(&self, state: &InputState) -> Color {
        match state {
            InputState::Disabled => Colors::GRAY_400,
            _ => Colors::GRAY_900,
        }
    }
    
    fn border_color(&self, state: &InputState) -> Color {
        match state {
            InputState::Normal => Colors::GRAY_200,
            InputState::Focus => Colors::PRIMARY,
            InputState::Error => Colors::ERROR,
            InputState::Disabled => Colors::GRAY_200,
        }
    }
    
    fn placeholder_color(&self, _state: &InputState) -> Color {
        Colors::GRAY_500
    }
    
    fn padding(&self) -> UiRect {
        UiRect::all(Spacing::MD)
    }
    
    fn font_size(&self) -> f32 {
        FontSizes::BASE
    }
    
    fn border_width(&self) -> Val {
        Val::Px(1.0)
    }
    
    fn border_radius(&self) -> Val {
        BorderRadius::FULL
    }
    
    fn height(&self) -> Val {
        Val::Px(40.0)
    }
}

// Input validation trait (stub for tutorial)
pub trait InputValidation {
    fn validate(&self, value: &str) -> Result<(), String>;
    fn format_input(&self, value: &str) -> String;
}

// Character name input validator (specific to character creation)
#[derive(Component, Debug)]
pub struct CharacterNameInput;

impl InputValidation for CharacterNameInput {
    fn validate(&self, value: &str) -> Result<(), String> {
        // TODO: Implement character name validation
        // - Check minimum/maximum length
        // - Check for allowed characters only
        // - Check for profanity filter
        Ok(())
    }
    
    fn format_input(&self, value: &str) -> String {
        // TODO: Implement character name formatting
        // - Trim whitespace
        // - Capitalize first letter
        // - Remove invalid characters
        value.to_string()
    }
}

// Input field interaction system (stub for tutorial)
pub fn input_field_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, &UiInputField),
        (Changed<Interaction>, With<UiInputField>),
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // TODO: Implement input field focus and text input handling
    // This system will handle:
    // - Focus management
    // - Text input capture
    // - Cursor position
    // - Text selection
    // - Validation on input change
}

// Helper functions for spawning input fields (stubs for tutorial)
impl UiInputField {
    pub fn new(variant: InputVariant, size: InputSize, placeholder: String) -> Self {
        Self {
            variant,
            size,
            state: InputState::Normal,
            placeholder,
            value: String::new(),
            max_length: None,
        }
    }
    
    pub fn default(placeholder: &str) -> Self {
        Self::new(InputVariant::Default, InputSize::Medium, placeholder.to_string())
    }
    
    pub fn search(placeholder: &str) -> Self {
        Self::new(InputVariant::Search, InputSize::Medium, placeholder.to_string())
    }
    
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }
    
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
    
    pub fn get_value(&self) -> &str {
        &self.value
    }
}