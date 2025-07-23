//! Entity bundles for convenient spawning.
//! 
//! Bundles group related components together to make entity spawning
//! more convenient and less error-prone.

use bevy::prelude::*;
use crate::components::{Character, CharacterSelected};
use crate::config::{display::TILE_SIZE, assets};

/// Bundle for spawning a game character
#[derive(Bundle)]
pub struct CharacterBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub character: Character,
}

impl CharacterBundle {
    /// Create a new character bundle at the specified world position
    pub fn new(asset_server: &AssetServer, x: f32, y: f32, selected: bool, name: impl Into<String>) -> Self {
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
            character: Character::new(name),
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
    pub fn new(asset_server: &AssetServer, x: f32, y: f32, name: impl Into<String>) -> Self {
        Self {
            sprite: Sprite {
                image: asset_server.load(assets::PLAYER_SELECTED),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 1.0),
            character: Character::new(name),
            selected: CharacterSelected,
        }
    }
}