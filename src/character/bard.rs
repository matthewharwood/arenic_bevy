use crate::character::Character;
use crate::utils::AudioClips;
use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct CharacterBard;

impl Character for CharacterBard {
    const CLASS_NAME: &'static str = "Bard";
    const AUDIO: AudioClips<4> = [
        ("character/bard-0.mp3", "Let the music guide us."),
        ("character/bard-1.mp3", "Together we are unstoppable."),
        ("character/bard-2.mp3", "I'll amplify your strength."),
        (
            "character/bard-greet.mp3",
            "Ah, a new face! Let me play you the song of our people.",
        ),
    ];
    const ICON: (&'static str, &'static str) = ("characters/bard_icon.png", "characters/bard_icon_selected.png");
    const PORTRAIT: &'static str = "characters/bard.png";
}
