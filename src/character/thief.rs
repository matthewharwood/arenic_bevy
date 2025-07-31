use bevy::prelude::Component;
use crate::character::Character;
use crate::utils::AudioClips;

#[derive(Component)]
pub struct CharacterThief;

impl Character for CharacterThief {
    const AUDIO: AudioClips<4> = [
        ("character/thief-0.mp3", "Shadows hide my strikes."),
        ("character/thief-1.mp3", "You'll never see me coming."),
        ("character/thief-2.mp3", "Quick and silent death."),
        ("character/thief-greet.mp3", "Oh, you actually saw me?"),
    ];
}