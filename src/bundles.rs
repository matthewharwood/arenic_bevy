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
    /// Configuration type for spawning (defaults to unit type)
    type Config: Default = ();
    /// Position representation (defaults to (f32, f32) tuple)
    type Position = (f32, f32);
    
    /// Create a bundle at the specified world position with default config
    fn spawn_at(asset_server: &AssetServer, position: Self::Position) -> Self::Bundle {
        Self::spawn_with_config(asset_server, position, Self::Config::default())
    }
    
    /// Create a bundle with custom configuration
    fn spawn_with_config(
        asset_server: &AssetServer, 
        position: Self::Position, 
        config: Self::Config
    ) -> Self::Bundle;
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

/// Configuration for character spawning
#[derive(Default)]
pub struct CharacterConfig {
    pub selected: bool,
}

/// Spawner for regular character bundles
pub struct CharacterSpawner;

impl Spawnable for CharacterSpawner {
    type Bundle = CharacterBundle;
    type Config = CharacterConfig;
    
    fn spawn_with_config(
        asset_server: &AssetServer, 
        (x, y): Self::Position, 
        config: Self::Config
    ) -> Self::Bundle {
        CharacterBundle::new(asset_server, x, y, config.selected)
    }
}

/// Spawner for selected character bundles
pub struct SelectedCharacterSpawner;

impl Spawnable for SelectedCharacterSpawner {
    type Bundle = SelectedCharacterBundle;
    
    fn spawn_with_config(
        asset_server: &AssetServer, 
        (x, y): Self::Position, 
        _config: Self::Config
    ) -> Self::Bundle {
        SelectedCharacterBundle::new(asset_server, x, y)
    }
}