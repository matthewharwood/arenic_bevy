use crate::character::Character;
use crate::utils::AudioClips;
use bevy::prelude::Component;

#[derive(Component, Debug, Default)]
pub struct CharacterHunter;

impl Character for CharacterHunter {
    const CLASS_NAME: &'static str = "Hunter";
    const AUDIO: AudioClips<4> = [
        ("character/hunter-0.mp3", "Patience. Precision. Perfection."),
        ("character/hunter-1.mp3", "The hunt begins now."),
        ("character/hunter-2.mp3", "My traps never miss."),
        (
            "character/hunter-greet.mp3",
            "Tracking prey? Or tracking treasure? Either way, I never miss.",
        ),
    ];
    const ICON: (&'static str, &'static str) = ("characters/hunter_icon.png", "characters/hunter_icon_selected.png");
    const PORTRAIT: &'static str = "characters/hunter.png";
}
