use crate::character::Character;
use crate::utils::AudioClips;
use bevy::prelude::Component;

#[derive(Component)]
pub struct CharacterForager;

impl Character for CharacterForager {
    const CLASS_NAME: &'static str = "Forager";
    const AUDIO: AudioClips<4> = [
        ("character/forager-0.mp3", "From earth comes power."),
        ("character/forager-1.mp3", "Nature provides everything."),
        ("character/forager-2.mp3", "I'll dig deep for victory."),
        (
            "character/forager-greet.mp3",
            "The Earth whispers Secrets to those who listen.",
        ),
    ];
    const ICON: &'static str = "characters/forager_icon.png";
    const PORTRAIT: &'static str = "characters/forager.png";
}
