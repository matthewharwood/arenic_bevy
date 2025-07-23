//! Character movement plugin.
//! 
//! This plugin handles all character movement functionality including
//! input handling, position updates, and movement constraints.

use bevy::prelude::*;
use crate::components::CharacterSelected;
use crate::config::display::TILE_SIZE;
use crate::utils::clamp_to_grid_boundaries;

/// Plugin that handles character movement
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_selected_player);
    }
}

/// System that handles player movement input and updates character positions
fn move_selected_player(
    mut player_query: Query<&mut Transform, With<CharacterSelected>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in &mut player_query {
        let mut new_x = transform.translation.x;
        let mut new_y = transform.translation.y;

        if input.just_pressed(KeyCode::KeyA) {
            // Move left
            new_x -= TILE_SIZE;
        }
        if input.just_pressed(KeyCode::KeyS) {
            // Move down
            new_y -= TILE_SIZE;
        }
        if input.just_pressed(KeyCode::KeyD) {
            // Move right
            new_x += TILE_SIZE;
        }
        if input.just_pressed(KeyCode::KeyW) {
            // Move up
            new_y += TILE_SIZE;
        }

        // Clamp position to stay within the 3x3 grid boundaries
        let (clamped_x, clamped_y) = clamp_to_grid_boundaries(new_x, new_y);

        // Apply the clamped position
        transform.translation.x = clamped_x;
        transform.translation.y = clamped_y;
    }
}