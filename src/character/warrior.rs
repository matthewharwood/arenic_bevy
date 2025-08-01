use crate::character::Character;
use crate::utils::AudioClips;
use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct CharacterWarrior;

impl Character for CharacterWarrior {
    const CLASS_NAME: &'static str = "Warrior";
    const AUDIO: AudioClips<4> = [
        ("character/warrior-0.mp3", "My shield protects all."),
        ("character/warrior-1.mp3", "Stand behind me!"),
        ("character/warrior-2.mp3", "Honor guides my blade."),
        ("character/warrior-greet.mp3", "Hail, friend!"),
    ];
    const ICON: (&'static str, &'static str) = ("characters/warrior_icon.png", "characters/warrior_icon_selected.png");
    const PORTRAIT: &'static str = "characters/warrior.png";
}
