//! UI component definitions.
//! 
//! This module contains all UI-specific component types used for
//! marking and managing user interface entities.

use bevy::prelude::*;

/// Marker component for the top navigation bar
#[derive(Component, Debug)]
pub struct TopNavBar;

/// Marker component for side navigation bars (left and right)
#[derive(Component, Debug)]
pub struct SideNavBar;

/// Marker component for the bottom navigation bar
#[derive(Component, Debug)]
pub struct BottomNavBar;