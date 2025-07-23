//! UI plugin implementation.
//! 
//! This module contains the main UI plugin that registers all UI-related
//! systems and manages the user interface lifecycle.

use bevy::prelude::*;
use super::systems::*;

/// Plugin responsible for all user interface functionality including
/// navigation bars, menus, and other UI elements.
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_top_nav_bar,
                spawn_side_nav_bars,
                spawn_bottom_nav_bar,
            ),
        );
        
        // Future UI update systems can be added here
        // .add_systems(Update, (
        //     update_ui_state,
        //     handle_ui_interactions,
        // ));
    }
}