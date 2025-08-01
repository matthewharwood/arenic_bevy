use bevy::prelude::*;

/// Color palette for the UI theme
pub struct Colors;

impl Colors {
    // Primary colors
    pub const PRIMARY: Color = Color::srgb(0.2, 0.5098, 0.8902);
    pub const PRIMARY_HOVER: Color = Color::srgb(0.9647, 0.9804, 0.9961);
    pub const PRIMARY_ACTIVE: Color = Color::srgb(0.0, 0.4, 0.6);

    // Secondary colors
    pub const SECONDARY: Color = Color::srgb(0.6, 0.4, 0.8);
    pub const SECONDARY_HOVER: Color = Color::srgb(0.5, 0.3, 0.7);
    pub const SECONDARY_ACTIVE: Color = Color::srgb(0.4, 0.2, 0.6);

    // Neutral colors
    pub const WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
    pub const BLACK: Color = Color::srgb(0.0, 0.0, 0.0);
    pub const GRAY_50: Color = Color::srgb(0.98, 0.98, 0.98);
    pub const GRAY_100: Color = Color::srgb(0.95, 0.95, 0.95);
    pub const GRAY_200: Color = Color::srgb(0.9, 0.9, 0.9);
    pub const GRAY_300: Color = Color::srgb(0.8, 0.8, 0.8);
    pub const GRAY_400: Color = Color::srgb(0.6, 0.6, 0.6);
    pub const GRAY_500: Color = Color::srgb(0.5, 0.5, 0.5);
    pub const GRAY_600: Color = Color::srgb(0.4, 0.4, 0.4);
    pub const GRAY_700: Color = Color::srgb(0.3, 0.3, 0.3);
    pub const GRAY_800: Color = Color::srgb(0.2, 0.2, 0.2);
    pub const GRAY_900: Color = Color::srgb(0.1, 0.1, 0.1);

    // Status colors
    pub const SUCCESS: Color = Color::srgb(0.2, 0.7, 0.3);
    pub const WARNING: Color = Color::srgb(0.9, 0.7, 0.2);
    pub const ERROR: Color = Color::srgb(0.8, 0.2, 0.2);
    pub const INFO: Color = Color::srgb(0.2, 0.5, 0.8);
}

/// Spacing constants for consistent layout
pub struct Spacing;

impl Spacing {
    pub const NONE: Val = Val::Px(0.0);
    pub const XS: Val = Val::Px(4.0);
    pub const SM: Val = Val::Px(8.0);
    pub const MD: Val = Val::Px(16.0);
    pub const LG: Val = Val::Px(24.0);
    pub const XL: Val = Val::Px(32.0);
    pub const XXL: Val = Val::Px(48.0);
    pub const XXXL: Val = Val::Px(64.0);
}

/// Font sizes for typography
pub struct FontSizes;

impl FontSizes {
    pub const XS: f32 = 12.0;
    pub const SM: f32 = 14.0;
    pub const BASE: f32 = 16.0;
    pub const LG: f32 = 18.0;
    pub const XL: f32 = 20.0;
    pub const XXL: f32 = 24.0;
    pub const XXXL: f32 = 32.0;
    pub const XXXXL: f32 = 48.0;
    pub const XXXXXL: f32 = 58.0;
}

/// Border radius values
pub struct BorderRadius;

impl BorderRadius {
    pub const NONE: Val = Val::Px(0.0);
    pub const SM: Val = Val::Px(4.0);
    pub const MD: Val = Val::Px(8.0);
    pub const LG: Val = Val::Px(12.0);
    pub const XL: Val = Val::Px(16.0);
    pub const FULL: Val = Val::Px(9999.0); // Effectively a circle/pill shape
}

/// Box shadow definitions
pub struct BoxShadow;

impl BoxShadow {
    pub const NONE: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    pub const SM: [f32; 4] = [0.0, 1.0, 3.0, 0.1];
    pub const MD: [f32; 4] = [0.0, 4.0, 6.0, 0.1];
    pub const LG: [f32; 4] = [0.0, 10.0, 15.0, 0.15];
    pub const XL: [f32; 4] = [0.0, 20.0, 25.0, 0.2];
}

/// Grid system constants
pub struct Grid;

impl Grid {
    pub const CONTAINER_MAX_WIDTH: Val = Val::Px(1200.0);
    pub const GUTTER: Val = Val::Px(16.0);
    pub const COLUMNS: u32 = 12;

    // Column width helpers
    pub fn col_width(span: u32) -> Val {
        Val::Percent((span as f32 / Self::COLUMNS as f32) * 100.0)
    }
}

/// Z-index layers for stacking context
pub struct ZIndex;

impl ZIndex {
    pub const BASE: i32 = 0;
    pub const DROPDOWN: i32 = 10;
    pub const STICKY: i32 = 20;
    pub const FIXED: i32 = 30;
    pub const MODAL_BACKDROP: i32 = 40;
    pub const MODAL: i32 = 50;
    pub const POPOVER: i32 = 60;
    pub const TOOLTIP: i32 = 70;
    pub const TOAST: i32 = 80;
}

/// Animation timing and easing
pub struct Animation;

impl Animation {
    pub const FAST: f32 = 0.15;
    pub const BASE: f32 = 0.3;
    pub const SLOW: f32 = 0.5;
    pub const SLOWER: f32 = 0.75;
}

/// Breakpoints for responsive design
pub struct Breakpoints;

impl Breakpoints {
    pub const SM: f32 = 640.0;
    pub const MD: f32 = 768.0;
    pub const LG: f32 = 1024.0;
    pub const XL: f32 = 1280.0;
    pub const XXL: f32 = 1536.0;
}
