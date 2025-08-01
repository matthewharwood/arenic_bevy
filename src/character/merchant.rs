use crate::character::Character;
use crate::utils::AudioClips;
use bevy::prelude::Component;

#[derive(Component)]
pub struct CharacterMerchant;

impl Character for CharacterMerchant {
    const CLASS_NAME: &'static str = "Merchant";
    const AUDIO: AudioClips<4> = [
        ("character/merchant-0.mp3", "Fortune favors the bold."),
        ("character/merchant-1.mp3", "Everything has a price."),
        ("character/merchant-2.mp3", "Luck is my greatest weapon."),
        (
            "character/merchant-greet.mp3",
            "Every deal's a gamble, but I always win.",
        ),
    ];
    const ICON: (&'static str, &'static str) = ("characters/merchant_icon.png", "characters/merchant_icon_selected.png");
    const PORTRAIT: &'static str = "characters/merchant.png";
}
