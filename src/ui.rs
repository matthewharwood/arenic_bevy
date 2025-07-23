//! UI plugin for the game interface.
//!
//! This module contains all UI-related functionality including navigation bars
//! and other interface elements.

use bevy::prelude::*;
use crate::components::{TopNavBar, SideNavBar, BottomNavBar};
use crate::config::{display::TILE_SIZE, ui::{CAMERA_PADDING_Y, SIDEBAR_WIDTH}};

/// Plugin that handles all UI-related functionality
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Startup,
                (
                    spawn_top_nav_bar,
                    spawn_side_nav_bars,
                    spawn_bottom_nav_bar,
                ),
            );
    }
}

fn spawn_top_nav_bar(mut commands: Commands) {
    // Calculate the navigation bar height based on CAMERA_PADDING_Y + 1 pixel
    let nav_bar_height = CAMERA_PADDING_Y.abs() + 1.0;

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(nav_bar_height),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        })
        .insert(BackgroundColor(Color::WHITE))
        .insert(TopNavBar);
}

fn spawn_side_nav_bars(mut commands: Commands) {
    // Calculate the sidebar width based on CAMERA_PADDING_Y + 1 pixel
    let sidebar_width = SIDEBAR_WIDTH;

    // Spawn left sidebar
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Px(sidebar_width),
            height: Val::Percent(100.0),
            ..default()
        })
        .insert(BackgroundColor(Color::WHITE.with_alpha(0.5)))
        .insert(SideNavBar);

    // Spawn right sidebar
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            right: Val::Px(0.0),
            width: Val::Px(sidebar_width),
            height: Val::Percent(100.0),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        })
        .insert(BackgroundColor(Color::WHITE.with_alpha(0.5)))
        .insert(SideNavBar);
}

fn spawn_bottom_nav_bar(mut commands: Commands) {
    // Calculate the bottom navigation bar height based on TILE_SIZE * 5
    let nav_bar_height = TILE_SIZE * 5.0;

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(nav_bar_height),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        })
        .insert(BackgroundColor(Color::WHITE.with_alpha(0.5)))
        .insert(BottomNavBar);
}