use bevy::prelude::*;

/// Color palette for consistent UI theming
pub struct Colors;

impl Colors {
    // Primary colors
    pub const PRIMARY: Color = Color::srgb(0.2, 0.5098, 0.8902);
    pub const PRIMARY_HOVER: Color = Color::srgb(0.9647, 0.9804, 0.9961);
    pub const PRIMARY_ACTIVE: Color = Color::srgb(0.0, 0.4, 0.6);

    // Neutral colors
    pub const WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
    pub const BLACK: Color = Color::srgb(0.0, 0.0, 0.0);
}

/// Spacing constants for consistent layout
pub struct Spacing;

impl Spacing {
    pub const MD: Val = Val::Px(16.0);
    pub const XL: Val = Val::Px(32.0);
}

/// Font sizes for typography
pub struct FontSizes;

impl FontSizes {
    pub const XXL: f32 = 24.0;
}