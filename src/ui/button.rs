use bevy::prelude::*;
use crate::ui::styles_config::{Colors, Spacing, FontSizes, BorderRadius};

#[derive(Component, Debug, Clone)]
pub struct UiButton {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub state: ButtonState,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Danger,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonState {
    Normal,
    Hover,
    Active,
    Disabled,
}

pub trait Button {
    const VARIANT: ButtonVariant;
    const SIZE: ButtonSize;
    
    fn background_color(&self, state: &ButtonState) -> Color;
    fn text_color(&self, state: &ButtonState) -> Color;
    fn border_color(&self, state: &ButtonState) -> Color;
    fn padding(&self) -> UiRect;
    fn font_size(&self) -> f32;
    fn border_width(&self) -> Val;
    fn border_radius(&self) -> Val;
    
    fn create_node(&self, state: &ButtonState) -> (Node, BackgroundColor, BorderColor) {
        (
            Node {
                padding: self.padding(),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(self.border_width()),
                ..default()
            },
            BackgroundColor(self.background_color(state)),
            BorderColor(self.border_color(state)),
        )
    }
    
    fn create_text_components(&self, text: &str, state: &ButtonState) -> (Text, TextFont, TextColor) {
        (
            Text::new(text),
            TextFont {
                font_size: self.font_size(),
                ..default()
            },
            TextColor(self.text_color(state)),
        )
    }
}

// Primary Button Implementation
#[derive(Component, Debug)]
pub struct PrimaryButton;

impl Button for PrimaryButton {
    const VARIANT: ButtonVariant = ButtonVariant::Primary;
    const SIZE: ButtonSize = ButtonSize::Medium;
    
    fn background_color(&self, state: &ButtonState) -> Color {
        match state {
            ButtonState::Normal => Colors::PRIMARY,
            ButtonState::Hover => Colors::PRIMARY_HOVER,
            ButtonState::Active => Colors::PRIMARY_ACTIVE,
            ButtonState::Disabled => Colors::GRAY_300,
        }
    }
    
    fn text_color(&self, state: &ButtonState) -> Color {
        match state {
            ButtonState::Disabled => Colors::GRAY_500,
            _ => Colors::WHITE,
        }
    }
    
    fn border_color(&self, state: &ButtonState) -> Color {
        self.background_color(state)
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
}

// Secondary Button Implementation
#[derive(Component, Debug)]
pub struct SecondaryButton;

impl Button for SecondaryButton {
    const VARIANT: ButtonVariant = ButtonVariant::Secondary;
    const SIZE: ButtonSize = ButtonSize::Medium;
    
    fn background_color(&self, state: &ButtonState) -> Color {
        match state {
            ButtonState::Normal => Colors::SECONDARY,
            ButtonState::Hover => Colors::SECONDARY_HOVER,
            ButtonState::Active => Colors::SECONDARY_ACTIVE,
            ButtonState::Disabled => Colors::GRAY_300,
        }
    }
    
    fn text_color(&self, state: &ButtonState) -> Color {
        match state {
            ButtonState::Disabled => Colors::GRAY_500,
            _ => Colors::WHITE,
        }
    }
    
    fn border_color(&self, state: &ButtonState) -> Color {
        self.background_color(state)
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
}

// Outline Button Implementation
#[derive(Component, Debug)]
pub struct OutlineButton;

impl Button for OutlineButton {
    const VARIANT: ButtonVariant = ButtonVariant::Outline;
    const SIZE: ButtonSize = ButtonSize::Medium;
    
    fn background_color(&self, state: &ButtonState) -> Color {
        match state {
            ButtonState::Normal => Color::NONE,
            ButtonState::Hover => Colors::PRIMARY.with_alpha(0.1),
            ButtonState::Active => Colors::PRIMARY.with_alpha(0.2),
            ButtonState::Disabled => Color::NONE,
        }
    }
    
    fn text_color(&self, state: &ButtonState) -> Color {
        match state {
            ButtonState::Normal => Colors::PRIMARY,
            ButtonState::Hover => Colors::PRIMARY_HOVER,
            ButtonState::Active => Colors::PRIMARY_ACTIVE,
            ButtonState::Disabled => Colors::GRAY_400,
        }
    }
    
    fn border_color(&self, state: &ButtonState) -> Color {
        self.text_color(state)
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
}

// Button interaction system (stub for tutorial)
pub fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, &UiButton),
        (Changed<Interaction>, With<UiButton>),
    >,
) {
    // TODO: Implement button state changes based on interaction
    // This system will handle hover, click, and disabled states
}

// Helper functions for spawning buttons (stubs for tutorial)
impl UiButton {
    pub fn new(variant: ButtonVariant, size: ButtonSize) -> Self {
        Self {
            variant,
            size,
            state: ButtonState::Normal,
        }
    }
    
    pub fn primary() -> Self {
        Self::new(ButtonVariant::Primary, ButtonSize::Medium)
    }
    
    pub fn secondary() -> Self {
        Self::new(ButtonVariant::Secondary, ButtonSize::Medium)
    }
    
    pub fn outline() -> Self {
        Self::new(ButtonVariant::Outline, ButtonSize::Medium)
    }
}