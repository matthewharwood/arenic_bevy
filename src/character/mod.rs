use crate::utils::AudioClips;
use bevy::prelude::*;

mod alchemist;
mod bard;
mod cardinal;
mod forager;
mod hunter;
mod merchant;
mod thief;
mod warrior;

use alchemist::ALCHEMIST_DATA;
use bard::BARD_DATA;
use cardinal::CARDINAL_DATA;
use forager::FORAGER_DATA;
use hunter::HUNTER_DATA;
use merchant::MERCHANT_DATA;
use thief::THIEF_DATA;
use warrior::WARRIOR_DATA;

/// Data structure containing all static information for a character type
#[derive(Clone, Copy, Debug)]
pub struct CharacterData {
    pub name: &'static str,
    pub audio: AudioClips<4>,
    pub icon: (&'static str, &'static str),
    pub portrait: &'static str,
    pub ability_1: (&'static str, &'static str),
    pub ability_2: (&'static str, &'static str),
    pub ability_3: (&'static str, &'static str),
    pub ability_4: (&'static str, &'static str),
}

/// Unified character system using enum-based architecture
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum CharacterType {
    Warrior,
    #[default]
    Hunter,
    Thief,
    Alchemist,
    Bard,
    Cardinal,
    Forager,
    Merchant,
}

/// Const lookup table containing all character data
const CHARACTER_DATA: [CharacterData; 8] = [
    WARRIOR_DATA,
    HUNTER_DATA,
    THIEF_DATA,
    ALCHEMIST_DATA,
    BARD_DATA,
    CARDINAL_DATA,
    FORAGER_DATA,
    MERCHANT_DATA,
];

impl CharacterType {
    /// Convert enum to index for array lookup
    const fn index(self) -> usize {
        self as usize
    }

    /// Get all character data for this type
    pub const fn data(self) -> &'static CharacterData {
        &CHARACTER_DATA[self.index()]
    }

    /// Get character class name
    pub const fn class_name(self) -> &'static str {
        self.data().name
    }

    /// Get character audio clips
    pub const fn audio(self) -> AudioClips<4> {
        self.data().audio
    }

    /// Get character icon paths (normal, selected)
    pub const fn icon(self) -> (&'static str, &'static str) {
        self.data().icon
    }

    /// Get character portrait path
    pub const fn portrait(self) -> &'static str {
        self.data().portrait
    }

    /// Get ability 1 (name, description)
    pub const fn ability_1(self) -> (&'static str, &'static str) {
        self.data().ability_1
    }

    /// Get ability 2 (name, description)
    pub const fn ability_2(self) -> (&'static str, &'static str) {
        self.data().ability_2
    }

    /// Get ability 3 (name, description)
    pub const fn ability_3(self) -> (&'static str, &'static str) {
        self.data().ability_3
    }

    /// Get ability 4 (name, description)
    pub const fn ability_4(self) -> (&'static str, &'static str) {
        self.data().ability_4
    }

    /// Returns all character types in the order they should appear in the UI
    pub const fn all() -> [CharacterType; 8] {
        [
            CharacterType::Warrior,
            CharacterType::Hunter,
            CharacterType::Thief,
            CharacterType::Alchemist,
            CharacterType::Bard,
            CharacterType::Cardinal,
            CharacterType::Forager,
            CharacterType::Merchant,
        ]
    }
}
