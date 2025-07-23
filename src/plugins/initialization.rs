//! Initialization plugin for game setup.
//! 
//! This plugin handles the initial setup of the game world including
//! camera, arenas, and initial game state.

use bevy::prelude::*;
use crate::{
    components::CurrentArena,
    config::display::*,
    const_camera::TypedCameraBundle,
    generators::{arena_positions, arena_tile_positions},
};

/// Plugin responsible for initializing the game world
pub struct InitializationPlugin;

impl Plugin for InitializationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_arenas, setup_initial_state));
    }
}

/// Sets up the game camera using const generic type-safe camera bundle
fn setup_camera(mut commands: Commands) {
    // Use const generic typed camera bundle for arena 1 (starting position)
    commands.spawn(TypedCameraBundle::normal_for_arena(1));
}

/// Sets up all arenas in the 3x3 grid using generators
fn setup_arenas(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Use Rust 2024 generator for cleaner arena setup
    for arena_pos in arena_positions() {
        let mut arena = commands.spawn((
            Transform::from_xyz(
                arena_pos.world_x,
                arena_pos.world_y,
                0.0,
            ),
            Visibility::default(),
        ));
        
        let image_path = format!("Grid_{}.png", arena_pos.index);
        
        // Use generator for tile positions with proper hierarchy
        arena.with_children(|parent| {
            for tile_pos in arena_tile_positions() {
                parent.spawn((
                    Sprite {
                        image: asset_server.load(image_path.clone()),
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    Transform::from_xyz(
                        tile_pos.world_x,
                        tile_pos.world_y,
                        0.0,
                    ),
                ));
            }
        });
    }
}

/// Sets up initial game state using const generic type-safe arena
fn setup_initial_state(mut commands: Commands) {
    // Use const generic CurrentArena with explicit type parameter
    commands.spawn(CurrentArena::<9>::new_unchecked(1));
}