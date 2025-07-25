//! Arena management plugin.
//!
//! This plugin handles all arena functionality, including arena setup,
//! grid/tile spawning, timer management, and arena lifecycle.

use crate::components::{ArenaName, ArenaTimer, CurrentArena};
use crate::config::arena::{ARENA_HEIGHT, ARENA_WIDTH, GRID_HEIGHT, GRID_WIDTH};
use crate::config::display::{HALF_TILE_SIZE, HALF_WINDOW_HEIGHT, HALF_WINDOW_WIDTH, TILE_SIZE};
use bevy::prelude::*;

/// Plugin that handles arena systems
pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_arenas, setup_arena_timers))
            .add_systems(Update, update_arena_timers);
    }
}

/// Setup arena grids, tiles, and current arena marker
fn setup_arenas(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn current arena marker
    commands.spawn(CurrentArena(1));

    // Set up 3x3 grid of arenas (9 arenas total)
    for arena_index in 0..9 {
        let arenas_per_row = 3;
        let arena_col = arena_index % arenas_per_row;
        let arena_row = arena_index / arenas_per_row;

        let x_offset = arena_col as f32 * ARENA_WIDTH;
        let y_offset = arena_row as f32 * ARENA_HEIGHT;

        let mut arena = commands.spawn(Transform::from_xyz(
            -HALF_WINDOW_WIDTH + HALF_TILE_SIZE + x_offset,
            HALF_WINDOW_HEIGHT - HALF_TILE_SIZE - y_offset,
            0.0,
        ));

        // Load arena-specific tile image
        let image_path = format!("{}.png", "default_grid_tile");

        // Create a grid of tiles for this arena
        for row in 0..GRID_HEIGHT {
            for col in 0..GRID_WIDTH {
                arena
                    .insert(InheritedVisibility::default())
                    .with_children(|parent| {
                        parent
                            .spawn(Sprite {
                                image: asset_server.load(image_path.clone()),
                                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                                ..default()
                            })
                            .insert(Transform::from_xyz(
                                col as f32 * TILE_SIZE,
                                -(row as f32 * TILE_SIZE),
                                0.0,
                            ));
                    });
            }
        }
    }
}

/// Setup arena timers for each arena
fn setup_arena_timers(mut commands: Commands) {
    // Spawn a timer entity for each arena
    for arena_index in 0..9 {
        let arena_name = ArenaName::from_index(arena_index);
        commands.spawn(ArenaTimer::new(arena_name));
    }
}

/// Update arena timers each frame
fn update_arena_timers(mut timer_query: Query<&mut ArenaTimer>, time: Res<Time>) {
    for mut arena_timer in &mut timer_query {
        // Only tick the timer if it's not paused
        if !arena_timer.timer.paused() {
            arena_timer.timer.tick(time.delta());

            // Check if the timer finished (2 minutes elapsed)
            if arena_timer.timer.just_finished() {
                println!(
                    "Timer finished for arena: {} - Restarting...",
                    arena_timer.arena.name()
                );
            }
        }
    }
}
