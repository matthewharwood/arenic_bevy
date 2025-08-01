use crate::character::Character;
use crate::utils::AudioClips;
use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct CharacterCardinal;

impl Character for CharacterCardinal {
    const CLASS_NAME: &'static str = "Cardinal";
    const AUDIO: AudioClips<4> = [
        ("character/cardinal-0.mp3", "Light shall heal and harm."),
        ("character/cardinal-1.mp3", "Sacred power flows through me."),
        ("character/cardinal-2.mp3", "I bring divine judgment."),
        (
            "character/cardinal-greet.mp3",
            "Blessings upon you, traveler.",
        ),
    ];
    const ICON: (&'static str, &'static str) = ("characters/cardinal_icon.png", "characters/cardinal_icon_selected.png");
    const PORTRAIT: &'static str = "characters/cardinal.png";
}
