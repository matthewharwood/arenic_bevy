//! Entity bundles for convenient spawning.
//! 
//! Bundles group related components together to make entity spawning
//! more convenient and less error-prone.

use bevy::prelude::*;
use crate::components::{Character, CharacterSelected};
use crate::config::{display::TILE_SIZE, assets};

/// Trait for types that can be spawned at specific positions
pub trait Spawnable {
    /// The bundle type this spawner creates
    type Bundle: Bundle;
    
    /// Create a bundle at the specified world position
    fn spawn_at(asset_server: &AssetServer, x: f32, y: f32) -> Self::Bundle;
}

/// Bundle for spawning a game character
#[derive(Bundle)]
pub struct CharacterBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub character: Character,
}

impl CharacterBundle {
    /// Create a new character bundle at the specified world position
    pub fn new(asset_server: &AssetServer, x: f32, y: f32, selected: bool) -> Self {
        let image_path = if selected {
            assets::PLAYER_SELECTED
        } else {
            assets::PLAYER_UNSELECTED
        };

        Self {
            sprite: Sprite {
                image: asset_server.load(image_path),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 1.0),
            character: Character,
        }
    }
}

/// Bundle for spawning a selected character (includes selection marker)
#[derive(Bundle)]
pub struct SelectedCharacterBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub character: Character,
    pub selected: CharacterSelected,
}

impl SelectedCharacterBundle {
    /// Create a new selected character bundle at the specified world position
    pub fn new(asset_server: &AssetServer, x: f32, y: f32) -> Self {
        Self {
            sprite: Sprite {
                image: asset_server.load(assets::PLAYER_SELECTED),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 1.0),
            character: Character,
            selected: CharacterSelected,
        }
    }
}

/// Spawner for regular character bundles
pub struct CharacterSpawner;

impl Spawnable for CharacterSpawner {
    type Bundle = CharacterBundle;
    
    fn spawn_at(asset_server: &AssetServer, x: f32, y: f32) -> Self::Bundle {
        CharacterBundle::new(asset_server, x, y, false)
    }
}

/// Spawner for selected character bundles
pub struct SelectedCharacterSpawner;

impl Spawnable for SelectedCharacterSpawner {
    type Bundle = SelectedCharacterBundle;
    
    fn spawn_at(asset_server: &AssetServer, x: f32, y: f32) -> Self::Bundle {
        SelectedCharacterBundle::new(asset_server, x, y)
    }
}