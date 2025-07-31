use bevy::prelude::Component;
use crate::character::Character;
use crate::utils::AudioClips;

#[derive(Component)]
pub struct CharacterCardinal;

impl Character for CharacterCardinal {
    const AUDIO: AudioClips<4> = [
        ("character/cardinal-0.mp3", "Light shall heal and harm."),
        ("character/cardinal-1.mp3", "Sacred power flows through me."),
        ("character/cardinal-2.mp3", "I bring divine judgment."),
        ("character/cardinal-greet.mp3", "Blessings upon you, traveler."),
    ];
}