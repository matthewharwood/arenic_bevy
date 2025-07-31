use bevy::prelude::Component;
use crate::character::Character;
use crate::utils::AudioClips;

#[derive(Component)]
pub struct CharacterAlchemist;

impl Character for CharacterAlchemist {
    const AUDIO: AudioClips<4> = [
        ("character/alchemist-0.mp3", "Science conquers all."),
        ("character/alchemist-1.mp3", "Every element serves me."),
        ("character/alchemist-2.mp3", "Behold the power of transmutation."),
        ("character/alchemist-greet.mp3", "Need a Potion? A Transmutation?"),
    ];
}
