use bevy::prelude::Component;
use crate::character::Character;
use crate::utils::AudioClips;

#[derive(Component)]
pub struct CharacterWarrior;

impl Character for CharacterWarrior {
    const AUDIO: AudioClips<4> = [
        ("character/warrior-0.mp3", "My shield protects all."),
        ("character/warrior-1.mp3", "Stand behind me!"),
        ("character/warrior-2.mp3", "Honor guides my blade."),
        ("character/warrior-greet.mp3", "Hail, friend!"),
    ];
    const ICON: &'static str = "characters/warrior-icon.png";
    const PORTRAIT: &'static str = "characters/warrior.png";
}