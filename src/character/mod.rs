use crate::utils::AudioClips;
use bevy::prelude::Component;

pub mod alchemist;
pub mod bard;
pub mod cardinal;
pub mod forager;
pub mod hunter;
pub mod merchant;
pub mod thief;
pub mod warrior;

#[derive(Component, Debug)]
pub struct Character;

impl Character {
    const AUDIO: AudioClips<4> = [
        ("boss_sound.ogg", "I am the boss!"),
        ("boss_sound.ogg", "I am the boss!"),
        ("boss_sound.ogg", "I am the boss!"),
        ("boss_sound.ogg", "I am the boss!"),
    ];
}
