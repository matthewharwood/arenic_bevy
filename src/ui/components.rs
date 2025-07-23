//! UI component definitions.
//! 
//! This module contains all UI-specific component types used for
//! marking and managing user interface entities.

use bevy::prelude::*;

/// Trait for UI components to enable generic operations
pub trait UIComponent {
    /// The name of this UI component type
    const COMPONENT_NAME: &'static str;
    
    /// Whether this component supports user interaction
    fn is_interactive(&self) -> bool {
        false
    }
}

/// Marker component for the top navigation bar
#[derive(Component, Debug)]
pub struct TopNavBar;

impl UIComponent for TopNavBar {
    const COMPONENT_NAME: &'static str = "TopNavBar";
}

/// Marker component for side navigation bars (left and right)
#[derive(Component, Debug)]
pub struct SideNavBar;

impl UIComponent for SideNavBar {
    const COMPONENT_NAME: &'static str = "SideNavBar";
}

/// Marker component for the bottom navigation bar
#[derive(Component, Debug)]
pub struct BottomNavBar;

impl UIComponent for BottomNavBar {
    const COMPONENT_NAME: &'static str = "BottomNavBar";
}