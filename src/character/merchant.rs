use bevy::prelude::Component;
use crate::character::Character;
use crate::utils::AudioClips;

#[derive(Component)]
pub struct CharacterMerchant;

impl Character for CharacterMerchant {
    const AUDIO: AudioClips<4> = [
        ("character/merchant-0.mp3", "Fortune favors the bold."),
        ("character/merchant-1.mp3", "Everything has a price."),
        ("character/merchant-2.mp3", "Luck is my greatest weapon."),
        ("character/merchant-greet.mp3", "Every deal's a gamble, but I always win."),
    ];
}