use crate::character::Character;
use crate::utils::AudioClips;
use bevy::prelude::Component;

#[derive(Component)]
pub struct CharacterAlchemist;

impl Character for CharacterAlchemist {
    const CLASS_NAME: &'static str = "Alchemist";
    const AUDIO: AudioClips<4> = [
        ("character/alchemist-0.mp3", "Science conquers all."),
        ("character/alchemist-1.mp3", "Every element serves me."),
        (
            "character/alchemist-2.mp3",
            "Behold the power of transmutation.",
        ),
        (
            "character/alchemist-greet.mp3",
            "Need a Potion? A Transmutation?",
        ),
    ];
    const ICON: &'static str = "characters/alchemist_icon.png";
    const PORTRAIT: &'static str = "characters/alchemist.png";
}
