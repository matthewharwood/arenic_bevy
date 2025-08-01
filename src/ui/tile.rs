use bevy::prelude::*;
use crate::ui::styles_config::{Colors, Spacing, BorderRadius, BoxShadow};

#[derive(Component, Debug, Clone)]
pub struct UiTile {
    pub variant: TileVariant,
    pub size: TileSize,
    pub state: TileState,
    pub selectable: bool,
    pub selected: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileVariant {
    Default,
    Card,
    Character,
    Feature,
    Interactive,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileState {
    Normal,
    Hover,
    Active,
    Selected,
    Disabled,
}

pub trait Tile {
    const VARIANT: TileVariant;
    const SIZE: TileSize;
    
    fn background_color(&self, state: &TileState) -> Color;
    fn border_color(&self, state: &TileState) -> Color;
    fn shadow(&self, state: &&TileState) -> [f32; 4];
    fn padding(&self) -> UiRect;
    fn border_width(&self) -> Val;
    fn border_radius(&self) -> Val;
    fn width(&self) -> Val;
    fn height(&self) -> Val;
    
    fn create_container(&self, state: &TileState) -> (Node, BackgroundColor, BorderColor) {
        (
            Node {
                width: self.width(),
                height: self.height(),
                padding: self.padding(),
                border: UiRect::all(self.border_width()),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(self.background_color(state)),
            BorderColor(self.border_color(state)),
        )
    }
}

// Character Card Tile Implementation (for character selection screen)
#[derive(Component, Debug)]
pub struct CharacterCardTile;

impl Tile for CharacterCardTile {
    const VARIANT: TileVariant = TileVariant::Character;
    const SIZE: TileSize = TileSize::Large;
    
    fn background_color(&self, state: &TileState) -> Color {
        match state {
            TileState::Normal => Colors::WHITE,
            TileState::Hover => Colors::GRAY_100,
            TileState::Active => Colors::GRAY_200,
            TileState::Selected => Colors::PRIMARY.with_alpha(0.1),
            TileState::Disabled => Colors::GRAY_100,
        }
    }
    
    fn border_color(&self, state: &TileState) -> Color {
        match state {
            TileState::Normal => Colors::GRAY_300,
            TileState::Hover => Colors::GRAY_400,
            TileState::Active => Colors::GRAY_500,
            TileState::Selected => Colors::PRIMARY,
            TileState::Disabled => Colors::GRAY_200,
        }
    }
    
    fn shadow(&self, state: &&TileState) -> [f32; 4] {
        match state {
            TileState::Normal => BoxShadow::SM,
            TileState::Hover => BoxShadow::MD,
            TileState::Active => BoxShadow::SM,
            TileState::Selected => BoxShadow::LG,
            TileState::Disabled => BoxShadow::NONE,
        }
    }
    
    fn padding(&self) -> UiRect {
        UiRect::all(Spacing::LG)
    }
    
    fn border_width(&self) -> Val {
        Val::Px(2.0)
    }
    
    fn border_radius(&self) -> Val {
        BorderRadius::LG
    }
    
    fn width(&self) -> Val {
        Val::Px(200.0)
    }
    
    fn height(&self) -> Val {
        Val::Px(280.0)
    }
}

// Feature Tile Implementation (for smaller content cards)
#[derive(Component, Debug)]
pub struct FeatureTile;

impl Tile for FeatureTile {
    const VARIANT: TileVariant = TileVariant::Feature;
    const SIZE: TileSize = TileSize::Medium;
    
    fn background_color(&self, state: &TileState) -> Color {
        match state {
            TileState::Normal => Colors::WHITE,
            TileState::Hover => Colors::GRAY_50,
            TileState::Active => Colors::GRAY_100,
            TileState::Selected => Colors::SECONDARY.with_alpha(0.1),
            TileState::Disabled => Colors::GRAY_100,
        }
    }
    
    fn border_color(&self, state: &TileState) -> Color {
        match state {
            TileState::Normal => Colors::GRAY_200,
            TileState::Hover => Colors::GRAY_300,
            TileState::Active => Colors::GRAY_400,
            TileState::Selected => Colors::SECONDARY,
            TileState::Disabled => Colors::GRAY_200,
        }
    }
    
    fn shadow(&self, state: &&TileState) -> [f32; 4] {
        match state {
            TileState::Normal => BoxShadow::SM,
            TileState::Hover => BoxShadow::MD,
            TileState::Selected => BoxShadow::MD,
            _ => BoxShadow::NONE,
        }
    }
    
    fn padding(&self) -> UiRect {
        UiRect::all(Spacing::MD)
    }
    
    fn border_width(&self) -> Val {
        Val::Px(1.0)
    }
    
    fn border_radius(&self) -> Val {
        BorderRadius::MD
    }
    
    fn width(&self) -> Val {
        Val::Px(150.0)
    }
    
    fn height(&self) -> Val {
        Val::Px(120.0)
    }
}

// Interactive Tile Implementation (for clickable elements)
#[derive(Component, Debug)]
pub struct InteractiveTile;

impl Tile for InteractiveTile {
    const VARIANT: TileVariant = TileVariant::Interactive;
    const SIZE: TileSize = TileSize::Medium;
    
    fn background_color(&self, state: &TileState) -> Color {
        match state {
            TileState::Normal => Colors::GRAY_100,
            TileState::Hover => Colors::GRAY_200,
            TileState::Active => Colors::GRAY_300,
            TileState::Selected => Colors::PRIMARY.with_alpha(0.2),
            TileState::Disabled => Colors::GRAY_100,
        }
    }
    
    fn border_color(&self, state: &TileState) -> Color {
        match state {
            TileState::Normal => Colors::GRAY_300,
            TileState::Hover => Colors::GRAY_400,
            TileState::Active => Colors::GRAY_500,
            TileState::Selected => Colors::PRIMARY,
            TileState::Disabled => Colors::GRAY_200,
        }
    }
    
    fn shadow(&self, _state: &&TileState) -> [f32; 4] {
        BoxShadow::SM
    }
    
    fn padding(&self) -> UiRect {
        UiRect::all(Spacing::MD)
    }
    
    fn border_width(&self) -> Val {
        Val::Px(1.0)
    }
    
    fn border_radius(&self) -> Val {
        BorderRadius::MD
    }
    
    fn width(&self) -> Val {
        Val::Percent(100.0)
    }
    
    fn height(&self) -> Val {
        Val::Auto
    }
}

// Tile interaction system (stub for tutorial)
pub fn tile_interaction_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut UiTile,
        ),
        (Changed<Interaction>, With<UiTile>),
    >,
) {
    // TODO: Implement tile interaction handling
    // This system will handle:
    // - Hover state changes
    // - Click/selection handling
    // - State transitions
    // - Selection events
}

// Character selection specific components
#[derive(Component, Debug)]
pub struct CharacterSelectionTile {
    pub character_class: String, // TODO: Replace with actual CharacterClass enum
    pub selected: bool,
}

#[derive(Component, Debug)]
pub struct TileImage {
    pub image_path: String,
}

#[derive(Component, Debug)]
pub struct TileTitle {
    pub text: String,
}

#[derive(Component, Debug)]
pub struct TileDescription {
    pub text: String,
}

// Helper functions for creating tile layouts (stubs for tutorial)
impl UiTile {
    pub fn new(variant: TileVariant, size: TileSize) -> Self {
        Self {
            variant,
            size,
            state: TileState::Normal,
            selectable: false,
            selected: false,
        }
    }
    
    pub fn character_card() -> Self {
        Self::new(TileVariant::Character, TileSize::Large)
    }
    
    pub fn feature() -> Self {
        Self::new(TileVariant::Feature, TileSize::Medium)
    }
    
    pub fn interactive() -> Self {
        Self::new(TileVariant::Interactive, TileSize::Medium)
    }
    
    pub fn selectable(mut self) -> Self {
        self.selectable = true;
        self
    }
    
    pub fn select(&mut self) {
        if self.selectable {
            self.selected = true;
            self.state = TileState::Selected;
        }
    }
    
    pub fn deselect(&mut self) {
        self.selected = false;
        if self.state == TileState::Selected {
            self.state = TileState::Normal;
        }
    }
}

// Grid layout helper for character selection screen (stub for tutorial)
pub struct TileGrid {
    pub columns: usize,
    pub rows: usize,
    pub gap: Val,
}

impl TileGrid {
    pub fn character_selection() -> Self {
        Self {
            columns: 4,
            rows: 2,
            gap: Spacing::MD,
        }
    }
    
    pub fn create_grid_container(&self) -> Node {
        Node {
            display: Display::Grid,
            grid_template_columns: vec![GridTrack::fr(1.0); self.columns],
            grid_template_rows: vec![GridTrack::fr(1.0); self.rows],
            column_gap: self.gap,
            row_gap: self.gap,
            padding: UiRect::all(Spacing::LG),
            ..default()
        }
    }
}