use crate::character::Character;
use crate::utils::AudioClips;
use bevy::prelude::Component;

#[derive(Component, Default)]
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
    const ABILITY_1: (&'static str, &'static str) = ("Dice", "Stackable 1% Chance to increase Critical strike on yourself for next attack. instant cast stacks");
    const ABILITY_2: (&'static str, &'static str) = ("Coin Toss", "Spend money to give a skill shot. Big damage if hits, get money back. Up to 5s cast time. 10second cd");
    const ABILITY_3: (&'static str, &'static str) = ("Fortune", "Aura: The merchant and any adjacent hero's will have a luck % increase to gain gold on offensive attack (luck stacks)");
    const ABILITY_4: (&'static str, &'static str) = ("TBD", "4x4 grid that 2x Crit damage. 10 secs and 30s cd. (Stacks with other merchant)");
}
