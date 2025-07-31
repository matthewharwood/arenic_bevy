use bevy::prelude::Component;
use crate::character::Character;
use crate::utils::AudioClips;

#[derive(Component, Debug)]
pub struct CharacterHunter;

impl Character for CharacterHunter {
    const AUDIO: AudioClips<4> = [
        ("character/hunter-0.mp3", "Patience. Precision. Perfection."),
        ("character/hunter-1.mp3", "The hunt begins now."),
        ("character/hunter-2.mp3", "My traps never miss."),
        ("character/hunter-greet.mp3", "Tracking prey? Or tracking treasure? Either way, I never miss."),
    ];
}
